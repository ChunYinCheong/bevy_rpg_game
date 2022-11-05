use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

use super::{
    animation::{AnimationData, AnimationSheet, AnimationState},
    game_world::GameObjectType,
    unit::{self, SpawnUnit, Unit},
    unit_action::{ActionData, ActionId, UnitAnimation},
    unit_state::ActionState,
};

pub struct RockPlugin;
impl Plugin for RockPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_inspectable::<Rock>();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Component, Inspectable)]
pub struct Rock;

pub fn spawn_rock(
    commands: &mut Commands,
    position: Vec2,

    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let id = unit::spawn_unit(
        SpawnUnit {
            name: "Rock",
            unit: Unit {
                dead: false,
                hp: 1,
                movement_speed: 0.0,
                action_id: ActionId::Idle,
                action_state: ActionState::Active,
                action_time: None,
                action_data: ActionData::None,
                stun: 0.0,
            },
            translation: position,
            action_ids: vec![ActionId::Idle],
            texture_path: "images/rock/rock.png",
            texture_columns: 2,
            texture_rows: 1,
            animation_sheet: AnimationSheet {
                animations: HashMap::from([
                    (
                        UnitAnimation::Idle.to_string(),
                        AnimationData {
                            start: 0,
                            len: 1,
                            frame_time: Duration::from_millis(500),
                            repeat: false,
                        },
                    ),
                    (
                        UnitAnimation::Dead.to_string(),
                        AnimationData {
                            start: 1,
                            len: 1,
                            frame_time: Duration::from_millis(500),
                            repeat: false,
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
        .insert(GameObjectType::Rock)
        .insert(Rock)
        .insert(RigidBody::Fixed);
    id
}
