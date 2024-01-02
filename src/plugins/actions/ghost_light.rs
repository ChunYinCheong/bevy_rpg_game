use bevy::prelude::*;
use bevy_rapier2d::{na::UnitComplex, prelude::*};

use crate::{
    plugins::{
        animation::ChangeAnimation,
        movement::Movement,
        team::Team,
        unit_action::UnitAnimation,
        unit_state::{
            ActionSystemLabel, ChangeActionRequest, UnitActionActiveUpdateEvent,
            UnitActionEnterActiveEvent, UnitActionEnterEvent, UnitState,
        },
        units::unit_command::UnitCommand,
    },
    utils::{self, Knockback, Shape},
    RAPIER_SCALE,
};

use super::{action::Skill, base::BaseSkill, skill_id::SkillId};

pub struct GhostLightPlugin;
impl Plugin for GhostLightPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<GhostLightAction>()
            .add_system(attach)
            .add_system(enter.label(ActionSystemLabel::Enter))
            .add_system(enter_active.label(ActionSystemLabel::EnterActive))
            .add_system(active_update.label(ActionSystemLabel::ActiveUpdate))
            .add_system(fire_ghost_light);
    }
}

#[derive(Debug, Component)]
pub struct GhostLight {
    timer: Timer,
    target: Option<Entity>,
}

pub fn spawn_ghost(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    spider: Entity,
    mut position: Transform,
    angle: f32,
    target: Option<Entity>,
    team: &Team,
) -> Entity {
    let animation_entity = {
        let texture_handle = asset_server.load("images/spider/spritesheet.png");
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
                    index: 4,
                    ..Default::default()
                },
                ..Default::default()
            })
            .id()
    };
    let forward = utils::get_forward(&position);

    let rot = UnitComplex::new(angle);
    let offset = rot.transform_vector(&forward.into());
    let offset = offset * 1.0 * RAPIER_SCALE;
    position.translation.x += offset.x;
    position.translation.y += offset.y;

    let id = utils::spawn_projectile(
        utils::Projectile {
            position,
            lifespan: 5.0,
            linvel: Vec2::ZERO,
            source: spider,
            shape: Shape::Ball(0.25),
            target_team: team.enemy_target(),
            damage: 1,
            hit_stun: 0.0,
            knockback: Knockback::Center(0.1),
        },
        commands,
    );
    commands
        .entity(id)
        .insert(GhostLight {
            timer: Timer::from_seconds(2.0, TimerMode::Once),
            target,
        })
        .add_child(animation_entity);
    id
}

pub fn fire_ghost_light(
    mut q: Query<(&mut GhostLight, &mut Velocity, &Transform)>,
    target_q: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    for (mut ghost, mut vel, pos) in q.iter_mut() {
        ghost.timer.tick(time.delta());
        if ghost.timer.just_finished() {
            if let Some(target) = ghost.target {
                if let Ok(target_pos) = target_q.get(target) {
                    let dir = target_pos.translation().truncate() - pos.translation.truncate();
                    let v = dir.normalize() * 5.0 * RAPIER_SCALE;
                    vel.linvel = v;
                }
            }
        }
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct GhostLightAction {
    pub f: f32,
    pub i: i32,
}

fn attach(mut commands: Commands, q: Query<(Entity, &Skill), Added<Skill>>) {
    for (e, a) in q.iter() {
        let setting = a.action_id.setting();
        if let BaseSkill::GhostLight(data) = setting.base {
            commands.entity(e).insert(GhostLightAction {
                f: data.f,
                i: data.i,
            });
        }
    }
}

fn enter(
    mut events: EventReader<UnitActionEnterEvent>,
    query: Query<(&GhostLightAction,)>,
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
    mut query: Query<(&mut GhostLightAction, &Skill, &Parent)>,
) {
    for ev in events.iter() {
        if let Ok((mut a, act, parent)) = query.get_mut(ev.action) {
            if ev.unit != parent.get() {
                continue;
            }
            debug!("GhostLight enter active: {:?}", ev);
            let s = act.action_id.setting();
            if let BaseSkill::GhostLight(gl) = s.base {
                a.f = gl.f;
                a.i = gl.i;
            }
        }
    }
}

pub fn active_update(
    mut events: EventReader<UnitActionActiveUpdateEvent>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<(&mut GhostLightAction, &Skill, &Parent)>,
    unit_q: Query<(&UnitState, &GlobalTransform, &Team)>,
    mut change_events: EventWriter<ChangeActionRequest>,
) {
    let delta = time.delta_seconds();
    for ev in events.iter() {
        for (mut a, act, parent) in query.iter_mut() {
            // get Action entity id?
            if act.action_id != ev.action_id {
                continue;
            }
            if ev.unit != parent.get() {
                continue;
            }
            if let Ok((unit_state, position, team)) = unit_q.get(ev.unit) {
                a.f -= delta;
                if a.f <= 0.8 && a.i == 0 {
                    a.i = 1;
                    spawn_ghost(
                        &mut commands,
                        &asset_server,
                        &mut texture_atlases,
                        ev.unit,
                        Transform::from(*position),
                        0.0,
                        unit_state.command.as_ref().and_then(|c| c.target_unit),
                        team,
                    );
                }
                if a.f <= 0.6 && a.i == 1 {
                    a.i = 2;
                    spawn_ghost(
                        &mut commands,
                        &asset_server,
                        &mut texture_atlases,
                        ev.unit,
                        Transform::from(*position),
                        std::f32::consts::PI / 4.0,
                        unit_state.command.as_ref().and_then(|c| c.target_unit),
                        team,
                    );
                    spawn_ghost(
                        &mut commands,
                        &asset_server,
                        &mut texture_atlases,
                        ev.unit,
                        Transform::from(*position),
                        std::f32::consts::PI * 7.0 / 4.0,
                        unit_state.command.as_ref().and_then(|c| c.target_unit),
                        team,
                    );
                }
                if a.f <= 0.4 && a.i == 2 {
                    a.i = 3;
                    spawn_ghost(
                        &mut commands,
                        &asset_server,
                        &mut texture_atlases,
                        ev.unit,
                        Transform::from(*position),
                        std::f32::consts::PI / 2.0,
                        unit_state.command.as_ref().and_then(|c| c.target_unit),
                        team,
                    );
                    spawn_ghost(
                        &mut commands,
                        &asset_server,
                        &mut texture_atlases,
                        ev.unit,
                        Transform::from(*position),
                        std::f32::consts::PI * 3.0 / 2.0,
                        unit_state.command.as_ref().and_then(|c| c.target_unit),
                        team,
                    );
                }
                if a.f <= 0.2 && a.i == 3 {
                    a.i = 4;
                    spawn_ghost(
                        &mut commands,
                        &asset_server,
                        &mut texture_atlases,
                        ev.unit,
                        Transform::from(*position),
                        std::f32::consts::PI * 3.0 / 4.0,
                        unit_state.command.as_ref().and_then(|c| c.target_unit),
                        team,
                    );
                    spawn_ghost(
                        &mut commands,
                        &asset_server,
                        &mut texture_atlases,
                        ev.unit,
                        Transform::from(*position),
                        std::f32::consts::PI * 5.0 / 4.0,
                        unit_state.command.as_ref().and_then(|c| c.target_unit),
                        team,
                    );
                }
                if a.f <= 0.0 {
                    spawn_ghost(
                        &mut commands,
                        &asset_server,
                        &mut texture_atlases,
                        ev.unit,
                        Transform::from(*position),
                        std::f32::consts::PI,
                        unit_state.command.as_ref().and_then(|c| c.target_unit),
                        team,
                    );

                    change_events.send(ChangeActionRequest {
                        action_id: SkillId::Idle,
                        command: UnitCommand::default(),
                        entity: ev.unit,
                    });
                }
            }
        }
    }
}
