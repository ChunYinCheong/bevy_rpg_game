use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use std::collections::HashMap;
use std::time::Duration;

use super::animation::AnimationData;
use super::game_world::GameObjectType;
use super::unit::{self, SpawnUnit};
use super::unit_action::{ActionData, ActionId, UnitAnimation};
use super::unit_state::{ActionState, UnitCommand};
use crate::plugins::animation::{AnimationSheet, AnimationState};
use crate::plugins::player::Player;
use crate::plugins::unit::{KillReward, Unit};
use crate::res::GameWorldConfig;
use crate::RAPIER_SCALE;

pub struct FoxPlugin;

impl Plugin for FoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fox_ai)
            .register_inspectable::<FoxAi>()
            .register_inspectable::<Fox>();
    }
}

#[derive(Debug, Component, Inspectable)]
pub struct Fox {}

#[derive(Debug, Component, Inspectable)]
pub struct FoxAi {}

pub(crate) fn fox_ai(
    mut enemy_q: Query<
        (Entity, &Transform, &Unit, &mut UnitCommand),
        (With<FoxAi>, Without<Player>),
    >,
    player_q: Query<&Transform, (With<Player>, Without<FoxAi>)>,
    config: Res<GameWorldConfig>,
) {
    if !config.active {
        return;
    }
    if let Ok(player) = player_q.get_single() {
        for (_, pos, unit, mut command) in enemy_q.iter_mut() {
            if unit.dead {
                continue;
            }
            match unit.action_id {
                ActionId::Idle | ActionId::Walk => {
                    let player_pos = player.translation.truncate();
                    let fox_pos = pos.translation.truncate();
                    let distance = player_pos.distance(fox_pos);
                    let dir = player_pos - fox_pos;
                    // info!("{player_pos} | {fox_pos} | {distance} | {dir}");
                    let dir = dir.normalize_or_zero();
                    if distance > 6.0 * RAPIER_SCALE {
                        command.action_id = ActionId::Walk;
                        command.movement_direction = dir;
                        command.target_direction = Some(dir);
                    } else if distance > 4.0 * RAPIER_SCALE {
                        command.action_id = ActionId::GhostLight;
                        command.movement_direction = Vec2::ZERO;
                        command.target_direction = None;
                    } else if distance > 1.0 * RAPIER_SCALE {
                        command.action_id = ActionId::Walk;
                        command.movement_direction = dir;
                        command.target_direction = Some(dir);
                    } else {
                        command.action_id = ActionId::Burning;
                        command.movement_direction = Vec2::ZERO;
                        command.target_direction = None;
                    }
                }
                _ => (),
            }
        }
    } else {
        for (_, _, unit, mut command) in enemy_q.iter_mut() {
            match unit.action_id {
                ActionId::Dead => (),
                _ => {
                    command.action_id = ActionId::Idle;
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

    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let id = unit::spawn_unit(
        SpawnUnit {
            name: "Fox",
            unit: Unit {
                dead: false,
                hp: 1,
                movement_speed: 5.0,
                action_id: ActionId::Idle,
                action_state: ActionState::Active,
                action_time: None,
                action_data: ActionData::None,
                stun: 0.0,
            },
            translation: position,
            action_ids: vec![ActionId::Idle, ActionId::Walk],
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
