use bevy::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

use crate::plugins::animation::{AnimationSheet, AnimationState};
use crate::plugins::player::Hero;
use crate::plugins::unit::{KillReward, Unit};
use crate::res::GameWorldConfig;
use crate::RAPIER_SCALE;

use super::actions::skill_id::SkillId;
use super::animation::AnimationData;
use super::game_world::GameObjectType;
use super::save::ClearOnReset;
use super::team::Team;
use super::unit::{self, SpawnUnit};
use super::unit_action::UnitAnimation;
use super::unit_state::UnitState;
use crate::plugins::units::unit_command::UnitCommand;

pub struct WolfPlugin;

impl Plugin for WolfPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(wolf_ai)
            .register_type::<WolfAi>()
            .register_type::<Wolf>();
    }
}

#[derive(Debug, Component, Reflect)]
pub struct Wolf {}

#[derive(Debug, Component, Reflect)]
pub struct WolfAi {}

pub(crate) fn wolf_ai(
    mut enemy_q: Query<
        (
            Entity,
            &GlobalTransform,
            &Unit,
            &mut UnitCommand,
            &UnitState,
        ),
        (With<WolfAi>, Without<Hero>),
    >,
    player_q: Query<&GlobalTransform, (With<Hero>, Without<WolfAi>)>,
    config: Res<GameWorldConfig>,
) {
    if !config.active {
        return;
    }
    if let Ok(player) = player_q.get_single() {
        for (_, pos, unit, mut command, unit_state) in enemy_q.iter_mut() {
            if unit.dead {
                continue;
            }
            match unit_state.action_id {
                SkillId::Idle | SkillId::MoveTo => {
                    let player_pos = player.translation().truncate();
                    let wolf_pos = pos.translation().truncate();
                    let distance = player_pos.distance(wolf_pos);
                    let dir = player_pos - wolf_pos;
                    // info!("{player_pos} | {wolf_pos} | {distance} | {dir}");
                    let dir = dir.normalize_or_zero();
                    if distance > 10.0 * RAPIER_SCALE {
                        command.action_id = SkillId::Idle;
                    } else if distance > 5.0 * RAPIER_SCALE {
                        command.action_id = SkillId::MoveTo;
                        command.movement_direction = dir;
                        command.target_direction = Some(dir);
                    } else {
                        command.action_id = SkillId::WolfAttack;
                        command.movement_direction = Vec2::ZERO;
                        command.target_direction = Some(dir);
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

pub fn spawn_wolf(
    commands: &mut Commands,
    position: Vec2,

    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let id = unit::spawn_unit(
        SpawnUnit {
            name: "Wolf",
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
            texture_path: "images/wolf/spritesheet.png",
            texture_columns: 5,
            texture_rows: 5,
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
                        UnitAnimation::Attack.to_string(),
                        AnimationData {
                            start: 3,
                            len: 1,
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
        .insert(Wolf {})
        .insert(WolfAi {})
        .insert(GameObjectType::Wolf)
        .insert(ClearOnReset)
        .insert(KillReward { exp: 10, money: 10 });
    id
}
