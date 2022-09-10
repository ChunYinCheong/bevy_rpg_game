use std::collections::HashMap;

use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::plugins::animation::{AnimationEntity, AnimationIndex, AnimationSheet, AnimationState};
use crate::plugins::player::Player;
use crate::plugins::unit::{KillReward, Unit};
use crate::res::GameWorldConfig;
use crate::utils::SPRITE_SCALE;
use crate::RAPIER_SCALE;

use super::animation::AnimationTimer;
use super::game_world::GameObjectType;
use super::save::ClearOnReset;
use super::unit::{self, SpawnUnit};
use super::unit_action::{ActionData, ActionId, UnitAnimation};
use super::unit_state::{ActionState, UnitCommand};

pub struct WolfPlugin;

impl Plugin for WolfPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(wolf_ai)
            .register_inspectable::<WolfAi>()
            .register_inspectable::<Wolf>();
    }
}

#[derive(Debug, Component, Inspectable)]
pub struct Wolf {}

#[derive(Debug, Component, Inspectable)]
pub struct WolfAi {}

pub(crate) fn wolf_ai(
    mut enemy_q: Query<
        (Entity, &Transform, &Unit, &mut UnitCommand),
        (With<WolfAi>, Without<Player>),
    >,
    player_q: Query<&Transform, (With<Player>, Without<WolfAi>)>,
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
                    let wolf_pos = pos.translation.truncate();
                    let distance = player_pos.distance(wolf_pos);
                    let dir = player_pos - wolf_pos;
                    // info!("{player_pos} | {wolf_pos} | {distance} | {dir}");
                    let dir = dir.normalize_or_zero();
                    if distance > 5.0 * RAPIER_SCALE {
                        command.action_id = ActionId::Walk;
                        command.target_direction = dir;
                    } else {
                        command.action_id = ActionId::WolfAttack;
                        command.target_direction = dir;
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

pub fn spawn_wolf(
    commands: &mut Commands,
    position: Vec2,

    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let animation_entity = {
        let texture_handle = asset_server.load("images/wolf/spritesheet.png");
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
                    (UnitAnimation::Attack.to_string(), (3, 1)),
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
            name: "Wolf",
            unit: Unit {
                dead: false,
                hp: 2,
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
        .insert(Wolf {})
        .insert(WolfAi {})
        .insert(GameObjectType::Wolf)
        .insert(ClearOnReset)
        .insert(KillReward { exp: 10, money: 10 });
    id
}
