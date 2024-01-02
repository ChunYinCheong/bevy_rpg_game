use bevy::prelude::*;

use crate::{
    plugins::{
        animation::ChangeAnimation,
        movement::Movement,
        team::Team,
        unit_action::UnitAnimation,
        unit_state::{ActionSystemLabel, UnitActionEnterActiveEvent, UnitActionEnterEvent},
    },
    utils::{Knockback, Shape},
};

use super::{action::Skill, base::BaseSkill};

pub struct BurningPlugin;
impl Plugin for BurningPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<BurningAction>()
            .add_system(attach)
            .add_system(enter.label(ActionSystemLabel::Enter))
            .add_system(enter_active.label(ActionSystemLabel::EnterActive));
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct BurningAction {}

fn attach(mut commands: Commands, q: Query<(Entity, &Skill), Added<Skill>>) {
    for (e, a) in q.iter() {
        let setting = a.action_id.setting();
        if let BaseSkill::Burning = setting.base {
            commands.entity(e).insert(BurningAction {});
        }
    }
}

fn enter(
    mut events: EventReader<UnitActionEnterEvent>,
    query: Query<(&BurningAction,)>,
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
    query: Query<(&BurningAction,)>,
    unit_q: Query<(&GlobalTransform, &Team)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for ev in events.iter() {
        if let Ok((_,)) = query.get(ev.action) {
            debug!("Burning enter active: {:?}", ev);
            if let Ok((position, team)) = unit_q.get(ev.unit) {
                let animation_entity = {
                    let texture_handle = asset_server.load("images/fox/unknown.png");
                    let texture_atlas = TextureAtlas::from_grid(
                        texture_handle,
                        Vec2::new(128.0, 128.0),
                        1,
                        1,
                        None,
                        None,
                    );
                    let texture_atlas_handle = texture_atlases.add(texture_atlas);
                    commands
                        .spawn(SpriteSheetBundle {
                            texture_atlas: texture_atlas_handle,
                            transform: Transform {
                                translation: Vec3::new(0.0, 0.0, 1.0),
                                ..Default::default()
                            },
                            sprite: TextureAtlasSprite {
                                index: 0,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .id()
                };
                let id = crate::utils::spawn_melee(
                    crate::utils::Melee {
                        offset: Vec2::new(0.0, 0.0),
                        lifespan: 0.5,
                        source: ev.unit,
                        shape: Shape::Ball(1.5),
                        parent_position: Transform::from(*position),
                        target_team: team.enemy_target(),
                        damage: 1,
                        hit_stun: 0.3,
                        knockback: Knockback::Center(0.1),
                    },
                    &mut commands,
                );
                commands.entity(id).add_child(animation_entity);
            }
        }
    }
}
