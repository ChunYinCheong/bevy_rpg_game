use bevy::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

use super::actions::skill_id::SkillId;
use super::animation::AnimationData;
use super::game_world::GameObjectType;
use super::team::Team;
use super::unit::{self, SpawnUnit};
use super::unit_action::UnitAnimation;
use super::unit_state::UnitState;
use crate::plugins::animation::{AnimationSheet, AnimationState};
use crate::plugins::player::Hero;
use crate::plugins::unit::{KillReward, Unit};
use crate::plugins::units::unit_command::UnitCommand;
use crate::res::GameWorldConfig;
use crate::RAPIER_SCALE;

pub struct FoxPlugin;

impl Plugin for FoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fox_ai)
            .register_type::<FoxAi>()
            .register_type::<Fox>();
    }
}

#[derive(Debug, Component, Reflect)]
pub struct Fox {}

#[derive(Debug, Component, Reflect)]
pub struct FoxAi {}

pub(crate) fn fox_ai(
    mut enemy_q: Query<
        (
            Entity,
            &GlobalTransform,
            &Unit,
            &mut UnitCommand,
            &UnitState,
        ),
        (With<FoxAi>, Without<Hero>),
    >,
    player_q: Query<(Entity, &GlobalTransform), (With<Hero>, Without<FoxAi>)>,
    config: Res<GameWorldConfig>,
) {
    if !config.active {
        return;
    }
    if let Ok((player_entity, player)) = player_q.get_single() {
        for (_, pos, unit, mut command, unit_state) in enemy_q.iter_mut() {
            if unit.dead {
                continue;
            }
            match unit_state.action_id {
                SkillId::Idle | SkillId::MoveTo => {
                    let player_pos = player.translation().truncate();
                    let fox_pos = pos.translation().truncate();
                    let distance = player_pos.distance(fox_pos);
                    let dir = player_pos - fox_pos;
                    // info!("{player_pos} | {fox_pos} | {distance} | {dir}");
                    let dir = dir.normalize_or_zero();
                    if distance <= 1.5 * RAPIER_SCALE {
                        command.action_id = SkillId::Burning;
                        command.movement_direction = Vec2::ZERO;
                        command.target_direction = None;
                    } else if distance <= 5.0 * RAPIER_SCALE {
                        command.action_id = SkillId::MoveTo;
                        command.movement_direction = dir;
                        command.target_direction = Some(dir);
                    } else if distance <= 8.0 * RAPIER_SCALE {
                        command.action_id = SkillId::GhostLight;
                        command.movement_direction = Vec2::ZERO;
                        command.target_direction = None;
                        command.target_unit = Some(player_entity);
                    } else {
                        command.action_id = SkillId::Idle;
                        command.movement_direction = Vec2::ZERO;
                        command.target_direction = None;
                        command.target_position = None;
                    }
                }
                _ => (),
            }
        }
    } else {
        for (_, _, _, mut command, unit_state) in enemy_q.iter_mut() {
            match unit_state.action_id {
                SkillId::Dead => (),
                _ => {
                    command.action_id = SkillId::Idle;
                    command.movement_direction = Vec2::ZERO;
                    command.target_direction = None;
                    command.target_position = None;
                }
            }
        }
    }
}

pub fn spawn_fox(
    commands: &mut Commands,
    position: Vec2,

    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let id = unit::spawn_unit(
        SpawnUnit {
            name: "Fox",
            unit: Unit {
                dead: false,
                hp: 30,
                hp_max: 30,
                atk: 1,
                movement_speed: 5.0,
                stun: 0.0,
            },
            team: Team::Enemy,
            translation: position,
            action_ids: vec![SkillId::Idle, SkillId::MoveTo],
            texture_path: "images/player/spritesheet.png",
            texture_columns: 5,
            texture_rows: 1,
            animation_sheet: AnimationSheet {
                animations: HashMap::from([
                    (
                        UnitAnimation::Idle.to_string(),
                        AnimationData {
                            start: 0,
                            len: 1,
                            frame_time: Duration::from_millis(500),
                            repeat: true,
                        },
                    ),
                    (
                        UnitAnimation::Walk.to_string(),
                        AnimationData {
                            start: 1,
                            len: 2,
                            frame_time: Duration::from_millis(500),
                            repeat: true,
                        },
                    ),
                    (
                        UnitAnimation::Dead.to_string(),
                        AnimationData {
                            start: 4,
                            len: 1,
                            frame_time: Duration::from_millis(500),
                            repeat: true,
                        },
                    ),
                ]),
            },
            animation_state: AnimationState {
                name: UnitAnimation::Idle.to_string(),
                index: 0,
                duration: Duration::ZERO,
            },
        },
        commands,
        asset_server,
        texture_atlases,
    );

    commands
        .entity(id)
        .insert(Fox {})
        .insert(FoxAi {})
        .insert(GameObjectType::Fox)
        .insert(KillReward { exp: 10, money: 10 });
    id
}
