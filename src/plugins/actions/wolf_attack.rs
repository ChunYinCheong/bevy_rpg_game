use bevy::prelude::*;

use crate::{
    plugins::{
        animation::ChangeAnimation,
        movement::Movement,
        team::Team,
        unit::Unit,
        unit_action::UnitAnimation,
        unit_state::{
            ActionSystemLabel, UnitActionEnterActiveEvent, UnitActionEnterEvent, UnitState,
        },
    },
    utils::{self, Knockback, Shape},
};

use super::{action::Skill, base::BaseSkill};

pub struct WolfAttackPlugin;
impl Plugin for WolfAttackPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<WolfAttackAction>()
            .add_system(attach)
            .add_system(enter.label(ActionSystemLabel::Enter))
            .add_system(enter_active.label(ActionSystemLabel::EnterActive));
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct WolfAttackAction {}

fn attach(mut commands: Commands, q: Query<(Entity, &Skill), Added<Skill>>) {
    for (e, a) in q.iter() {
        let setting = a.action_id.setting();
        if let BaseSkill::WolfAttack = setting.base {
            commands.entity(e).insert(WolfAttackAction {});
        }
    }
}

fn enter(
    mut events: EventReader<UnitActionEnterEvent>,
    query: Query<(&WolfAttackAction,)>,
    mut unit_q: Query<(&mut Movement, &UnitState)>,
    mut anim_events: EventWriter<ChangeAnimation>,
) {
    for ev in events.iter() {
        if let Ok((_,)) = query.get(ev.action) {
            if let Ok((mut movement, us)) = unit_q.get_mut(ev.unit) {
                movement.speed = 0.0;
                anim_events.send(ChangeAnimation {
                    entity: ev.unit,
                    name: UnitAnimation::Attack.to_string(),
                });

                if let Some(command) = &us.command {
                    movement.face = command.target_direction;
                }
            }
        }
    }
}

fn enter_active(
    mut events: EventReader<UnitActionEnterActiveEvent>,
    query: Query<(&WolfAttackAction,)>,
    mut unit_q: Query<(&GlobalTransform, &Team, &mut Movement, &Unit)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for ev in events.iter() {
        if let Ok((_,)) = query.get(ev.action) {
            debug!("WolfAttack enter active: {:?}", ev);
            if let Ok((position, team, mut movement, unit)) = unit_q.get_mut(ev.unit) {
                movement.speed = unit.movement_speed * 3.0;
                movement.direction = utils::get_forward_global(position);
                wolf_attack(
                    &mut commands,
                    Transform::from(*position),
                    ev.unit,
                    team,
                    &asset_server,
                    &mut texture_atlases,
                );
            }
        }
    }
}

pub fn wolf_attack(
    commands: &mut Commands,
    position: Transform,
    wolf: Entity,
    team: &Team,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let animation_entity = {
        let texture_handle = asset_server.load("images/player/spritesheet.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 5, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    index: 1,
                    ..Default::default()
                },
                ..Default::default()
            })
            .id()
    };
    let id = crate::utils::spawn_melee(
        crate::utils::Melee {
            offset: Vec2::new(0.1, 0.0),
            lifespan: 0.5,
            source: wolf,
            shape: Shape::Ball(0.5),
            target_team: team.enemy_target(),
            parent_position: position,
            damage: 1,
            hit_stun: 0.3,
            knockback: Knockback::Center(0.1),
        },
        commands,
    );
    commands.entity(id).add_child(animation_entity);

    id
}
