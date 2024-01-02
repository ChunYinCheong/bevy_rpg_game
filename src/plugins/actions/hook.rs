use bevy::prelude::*;

use crate::{
    plugins::{
        animation::ChangeAnimation,
        hook::OnHitHook,
        movement::Movement,
        team::Team,
        unit_action::UnitAnimation,
        unit_state::{ActionSystemLabel, UnitActionEnterActiveEvent, UnitActionEnterEvent},
    },
    utils::{self, Knockback, Shape},
    RAPIER_SCALE,
};

use super::{action::Skill, base::BaseSkill};

pub struct HookPlugin;
impl Plugin for HookPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<HookAction>()
            .add_system(attach)
            .add_system(enter.label(ActionSystemLabel::Enter))
            .add_system(enter_active.label(ActionSystemLabel::EnterActive));
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct HookAction {}

fn attach(mut commands: Commands, q: Query<(Entity, &Skill), Added<Skill>>) {
    for (e, a) in q.iter() {
        let setting = a.action_id.setting();
        if let BaseSkill::Hook = setting.base {
            commands.entity(e).insert(HookAction {});
        }
    }
}

fn enter(
    mut events: EventReader<UnitActionEnterEvent>,
    query: Query<(&HookAction,)>,
    mut unit_q: Query<(&mut Movement,)>,
    mut anim_events: EventWriter<ChangeAnimation>,
) {
    for ev in events.iter() {
        if let Ok((_,)) = query.get(ev.action) {
            if let Ok((mut movement,)) = unit_q.get_mut(ev.unit) {
                movement.speed = 0.0;
                anim_events.send(ChangeAnimation {
                    entity: ev.unit,
                    name: UnitAnimation::Attack.to_string(),
                });
            }
        }
    }
}

fn enter_active(
    mut events: EventReader<UnitActionEnterActiveEvent>,
    query: Query<(&HookAction,)>,
    unit_q: Query<(&GlobalTransform, &Team)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for ev in events.iter() {
        if let Ok((_,)) = query.get(ev.action) {
            debug!("Hook enter active: {:?}", ev);
            if let Ok((position, team)) = unit_q.get(ev.unit) {
                hook(
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

pub fn hook(
    commands: &mut Commands,
    mut position: Transform,
    player: Entity,
    team: &Team,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let animation_entity = {
        let texture_handle = asset_server.load("images/player/spritesheet.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 10, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    index: 9,
                    ..Default::default()
                },
                ..Default::default()
            })
            .id()
    };
    let forward = utils::get_forward(&position);
    let offset = forward * 1.0 * RAPIER_SCALE;
    let linvel = forward * 10.0 * RAPIER_SCALE;
    position.translation.x += offset.x;
    position.translation.y += offset.y;

    let id = utils::spawn_projectile(
        utils::Projectile {
            position,
            lifespan: 1.0,
            linvel,
            source: player,
            shape: Shape::Ball(0.25),
            target_team: team.enemy_target(),
            damage: 1,
            hit_stun: 0.3,
            knockback: Knockback::Center(0.0),
        },
        commands,
    );
    commands
        .entity(id)
        .add_child(animation_entity)
        .insert(OnHitHook {
            target: position.translation.truncate(),
        });
}
