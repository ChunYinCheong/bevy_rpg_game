use bevy::prelude::*;

use crate::{
    plugins::{
        animation::ChangeAnimation,
        movement::Movement,
        team::Team,
        unit_action::UnitAnimation,
        unit_state::{
            ActionSystemLabel, UnitActionActiveUpdateEvent, UnitActionEnterActiveEvent,
            UnitActionEnterEvent,
        },
    },
    utils::{self, Knockback, Shape},
    RAPIER_SCALE,
};

use super::{action::Skill, base::BaseSkill};

pub struct BurstFirePlugin;
impl Plugin for BurstFirePlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<BurstFireAction>()
            .add_system(attach)
            .add_system(enter.label(ActionSystemLabel::Enter))
            .add_system(enter_active.label(ActionSystemLabel::EnterActive))
            .add_system(active_update.label(ActionSystemLabel::ActiveUpdate));
    }
}

pub fn burst_fire(
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
            lifespan: 3.0,
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
    commands.entity(id).add_child(animation_entity);
}

#[derive(Debug, Default, Component, Reflect)]
pub struct BurstFireAction {
    pub f: f32,
    pub i: i32,
}

fn attach(mut commands: Commands, q: Query<(Entity, &Skill), Added<Skill>>) {
    for (e, a) in q.iter() {
        let setting = a.action_id.setting();
        if let BaseSkill::BurstFire(data) = setting.base {
            commands.entity(e).insert(BurstFireAction {
                f: data.f,
                i: data.i,
            });
        }
    }
}

fn enter(
    mut events: EventReader<UnitActionEnterEvent>,
    query: Query<(&BurstFireAction,)>,
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
    mut query: Query<(&mut BurstFireAction, &Skill)>,
    unit_q: Query<(&GlobalTransform, &Team)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for ev in events.iter() {
        if let Ok((mut a, act)) = query.get_mut(ev.action) {
            debug!("BurstFire enter active: {:?}", ev);
            let s = act.action_id.setting();
            if let BaseSkill::BurstFire(bf) = s.base {
                a.f = bf.f;
                a.i = bf.i;
            }
            if let Ok((position, team)) = unit_q.get(ev.unit) {
                burst_fire(
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

pub fn active_update(
    mut events: EventReader<UnitActionActiveUpdateEvent>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<(&mut BurstFireAction, &Skill)>,
    unit_q: Query<(&GlobalTransform, &Team)>,
) {
    let delta = time.delta_seconds();
    for ev in events.iter() {
        if let Ok((mut a, _)) = query.get_mut(ev.action) {
            debug!("BurstFire active update: {:?}", ev);
            if let Ok((position, team)) = unit_q.get(ev.unit) {
                debug!("BurstFire active update: {a:?}, {delta:?}");
                a.f -= delta;
                if a.f <= 0.1 && a.i == 0 {
                    debug!("BurstFire active update: {:?}, first", ev);
                    a.i = 1;
                    burst_fire(
                        &mut commands,
                        Transform::from(*position),
                        ev.unit,
                        team,
                        &asset_server,
                        &mut texture_atlases,
                    );
                }
                if a.f <= 0.05 && a.i == 1 {
                    debug!("BurstFire active update: {:?}, second", ev);
                    a.i = 2;
                    burst_fire(
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
}
