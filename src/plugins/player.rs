use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use std::collections::HashMap;
use std::time::Duration;

use super::animation::AnimationData;
use super::item::{Equipment, InventoryUiRes, OpenInventoryEvent, SwitchEquipment};
use super::save::{SaveTransform, SaveUnit};
use super::unit_action::{ActionData, ActionId, UnitAnimation};
use super::unit_state::{ActionState, UnitCommand};
use crate::plugins::animation::{AnimationSheet, AnimationState};
use crate::plugins::unit::{self, SpawnUnit, Unit};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_controller)
            .add_system(update_command)
            .add_system(camera_follow_player)
            .add_system(cursor_position)
            .register_inspectable::<Player>()
            .register_inspectable::<PlayerController>();
    }
}

#[derive(Debug, Component, Inspectable)]
pub struct Player {}

#[derive(Debug, Default, Component, Inspectable)]
pub struct PlayerController {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,

    pub main_attack: bool,
    pub special_attack: bool,

    pub switch_weapon_1: bool,
    pub switch_weapon_2: bool,

    pub mouse_pos: Vec2,

    pub open_inventory: bool,
}

fn cursor_position(
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut cont: Query<&mut PlayerController>,
    inventory: Res<InventoryUiRes>,
) {
    if inventory.show {
        return;
    }
    if let Ok((camera, camera_transform)) = q_camera.get_single() {
        // get the window that the camera is displaying to (or the primary window)
        let wnd = if let RenderTarget::Window(id) = camera.target {
            windows.get(id).unwrap()
        } else {
            windows.get_primary().unwrap()
        };

        // check if the cursor is inside the window and get its position
        if let Some(screen_pos) = wnd.cursor_position() {
            // get the size of the window
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();

            // eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
            for mut c in cont.iter_mut() {
                c.mouse_pos = world_pos;
            }
        }
    }
}

pub fn update_controller(
    mut query: Query<&mut PlayerController>,
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut events: EventWriter<OpenInventoryEvent>,
    inventory: Res<InventoryUiRes>,
    mut switch_events: EventWriter<SwitchEquipment>,
) {
    for mut controller in query.iter_mut() {
        if inventory.show {
            controller.down = false;
            controller.up = false;
            controller.right = false;
            controller.left = false;

            controller.main_attack = false;
            controller.special_attack = false;
        } else {
            controller.down = keys.pressed(KeyCode::S);
            controller.up = keys.pressed(KeyCode::W);
            controller.right = keys.pressed(KeyCode::D);
            controller.left = keys.pressed(KeyCode::A);

            controller.main_attack = buttons.pressed(MouseButton::Left);
            controller.special_attack = buttons.pressed(MouseButton::Right);

            controller.switch_weapon_1 = keys.pressed(KeyCode::Key1);
            controller.switch_weapon_2 = keys.pressed(KeyCode::Key2);

            if controller.switch_weapon_1 {
                switch_events.send(SwitchEquipment { slot: 0 });
            }
            if controller.switch_weapon_2 {
                switch_events.send(SwitchEquipment { slot: 1 });
            }
        }
        controller.open_inventory = keys.just_pressed(KeyCode::Tab);
        if controller.open_inventory {
            events.send_default();
        }
    }
}

pub fn update_command(
    mut query: Query<(
        &PlayerController,
        &mut UnitCommand,
        &Equipment,
        &GlobalTransform,
    )>,
) {
    for (controller, mut command, equipment, tran) in query.iter_mut() {
        let mut dir = Vec2::ZERO;
        if controller.down {
            dir.y -= 1.0;
        }
        if controller.up {
            dir.y += 1.0;
        }
        if controller.right {
            dir.x += 1.0;
        }
        if controller.left {
            dir.x -= 1.0;
        }
        if dir.length_squared() > 0.0 {
            // if controller.sprint_just_pressed {
            //     command.action_id = ActionId::Walk;
            //     command.target_direction = dir.normalize();
            // } else {
            command.action_id = ActionId::Walk;
            command.movement_direction = dir;
            command.target_direction = Some(dir.normalize());
            // }
        } else {
            command.action_id = ActionId::Idle;
            command.movement_direction = Vec2::ZERO;
            command.target_direction = Some(dir.normalize());
        }

        command.target_position = Some(controller.mouse_pos);
        command.target_direction = Some(controller.mouse_pos - tran.translation().truncate());

        if controller.main_attack {
            match equipment.weapons[equipment.current].setting().kind {
                super::item::ItemKind::None => (),
                super::item::ItemKind::Weapon(w) => {
                    command.action_id = w.main_action_id;
                }
                super::item::ItemKind::Consume => (),
            }
        }
        if controller.special_attack {
            match equipment.weapons[equipment.current].setting().kind {
                super::item::ItemKind::None => (),
                super::item::ItemKind::Weapon(w) => {
                    command.action_id = w.sub_action_id;
                }
                super::item::ItemKind::Consume => (),
            }
        }
    }
}

pub fn camera_follow_player(
    mut camera_q: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_q: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    if let Ok(player) = player_q.get_single() {
        for mut camera in camera_q.iter_mut() {
            // camera.translation.x= player.translation.x;
            // camera.translation.z = player.translation.z + 10.0;
            camera.translation.x = player.translation.x;
            camera.translation.y = player.translation.y;
        }
    }
}

pub fn spawn_player(
    commands: &mut Commands,
    position: Vec2,
    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let id = unit::spawn_unit(
        SpawnUnit {
            name: "Player",
            unit: Unit {
                dead: false,
                hp: 10,
                movement_speed: 15.0,
                action_id: ActionId::Idle,
                action_state: ActionState::Active,
                action_time: None,
                action_data: ActionData::None,
                stun: 0.0,
            },
            translation: position,
            action_ids: vec![ActionId::Idle, ActionId::Walk],
            texture_path: "images/player/spritesheet.png",
            texture_columns: 10,
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
                        UnitAnimation::Run.to_string(),
                        AnimationData {
                            start: 3,
                            len: 2,
                            frame_time: Duration::from_millis(500),
                            repeat: true,
                        },
                    ),
                    (
                        UnitAnimation::Attack.to_string(),
                        AnimationData {
                            start: 5,
                            len: 1,
                            frame_time: Duration::from_millis(500),
                            repeat: true,
                        },
                    ),
                    (
                        UnitAnimation::Stab.to_string(),
                        AnimationData {
                            start: 6,
                            len: 1,
                            frame_time: Duration::from_millis(500),
                            repeat: true,
                        },
                    ),
                    (
                        UnitAnimation::BurstFire.to_string(),
                        AnimationData {
                            start: 7,
                            len: 1,
                            frame_time: Duration::from_millis(500),
                            repeat: true,
                        },
                    ),
                    (
                        UnitAnimation::Hook.to_string(),
                        AnimationData {
                            start: 8,
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
        .insert(Player {})
        .insert(PlayerController::default())
        // Save
        .insert(SaveUnit)
        .insert(SaveTransform)
        .id()
}
