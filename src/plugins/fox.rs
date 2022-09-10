use std::collections::HashMap;

use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use super::animation::AnimationTimer;
use super::game_world::GameObjectType;
use super::unit::{self, SpawnUnit};
use super::unit_action::{ActionData, ActionId, UnitAnimation};
use super::unit_state::{ActionState, UnitCommand};
use crate::plugins::animation::{AnimationEntity, AnimationIndex, AnimationSheet, AnimationState};
use crate::plugins::player::Player;
use crate::plugins::unit::{KillReward, Unit};
use crate::res::GameWorldConfig;
use crate::utils::SPRITE_SCALE;
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
                        command.target_direction = dir;
                    } else if distance > 4.0 * RAPIER_SCALE {
                        command.action_id = ActionId::GhostLight;
                    } else if distance > 1.0 * RAPIER_SCALE {
                        command.action_id = ActionId::Walk;
                        command.target_direction = dir;
                    } else {
                        command.action_id = ActionId::Burning;
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
    let animation_entity = {
        let texture_handle = asset_server.load("images/fox/spritesheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 5, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    rotation: Default::default(),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, SPRITE_SCALE),
                },
                ..Default::default()
            })
            .insert(AnimationTimer(Timer::from_seconds(0.5, true)))
            .insert(AnimationSheet {
                animations: HashMap::from([
                    (UnitAnimation::Idle.to_string(), (0, 1)),
                    (UnitAnimation::Walk.to_string(), (1, 2)),
                    // (UnitAnimation::Attack.to_string(), (3, 1)),
                    (UnitAnimation::Dead.to_string(), (4, 1)),
                ]),
            })
            .insert(AnimationState {
                animation: UnitAnimation::Idle.to_string(),
            })
            .insert(AnimationIndex::default())
            .id()
    };

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
        },
        commands,
    );

    commands
        .entity(id)
        .add_child(animation_entity)
        .insert(AnimationEntity(animation_entity))
        .insert(Fox {})
        .insert(FoxAi {})
        .insert(GameObjectType::Fox)
        .insert(KillReward { exp: 10, money: 10 });
    id
}
