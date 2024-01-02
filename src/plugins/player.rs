use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy_egui::{egui, EguiContext};
use bevy_rapier2d::prelude::{QueryFilter, RapierContext};
use std::collections::HashMap;
use std::time::Duration;

use super::actions::action::Skill;
use super::actions::skill_id::SkillId;
use super::animation::AnimationData;
use super::interaction::Interacting;
use super::item::{Equipment, InventoryUiRes, OpenInventoryEvent, SwitchEquipment};
use super::save::{SaveTransform, SaveUnit};
use super::team::Team;
use super::unit_action::{UnitActions, UnitAnimation};
use crate::plugins::animation::{AnimationSheet, AnimationState};
use crate::plugins::unit::{self, SpawnUnit, Unit};
use crate::plugins::units::unit_command::UnitCommand;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_follow_player)
            // ACT
            // .add_system(update_controller)
            // .add_system(update_command)
            // .add_system(cursor_position)
            .register_type::<PlayerController>()
            // RTS
            .add_system(my_cursor_system)
            .add_system(project_point.after(my_cursor_system))
            .add_system(mouse_button_input)
            .register_type::<Hero>()
            .register_type::<RtsController>()
            // ui
            .add_system(ui_example_system);
    }
}

#[derive(Debug, Clone, Default, Component, Reflect)]
pub struct Hero {
    pub gold: i32,
}

#[derive(Debug, Default, Component, Reflect)]
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
            let window_size = Vec2::new(wnd.width(), wnd.height());

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
            command.action_id = SkillId::MoveTo;
            command.movement_direction = dir;
            command.target_direction = Some(dir.normalize());
            // }
        } else {
            command.action_id = SkillId::Idle;
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
    mut camera_q: Query<&mut Transform, (With<Camera>, Without<Hero>)>,
    player_q: Query<&GlobalTransform, (With<Hero>, Without<Camera>)>,
) {
    if let Ok(player) = player_q.get_single() {
        for mut camera in camera_q.iter_mut() {
            // camera.translation.x= player.translation.x;
            // camera.translation.z = player.translation.z + 10.0;
            camera.translation.x = player.translation().x;
            camera.translation.y = player.translation().y;
        }
    }
}

pub fn spawn_hero(
    commands: &mut Commands,
    position: Vec2,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let id = unit::spawn_unit(
        SpawnUnit {
            name: "Hero",
            unit: Unit {
                dead: false,
                hp: 100,
                hp_max: 100,
                atk: 20,
                movement_speed: 15.0,
                stun: 0.0,
            },
            team: Team::Player,
            translation: position,
            action_ids: vec![
                SkillId::Stun,
                SkillId::Dead,
                SkillId::Idle,
                SkillId::MoveTo,
                SkillId::Attack,
            ],
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
                        UnitAnimation::Move.to_string(),
                        AnimationData {
                            start: 1,
                            len: 2,
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
        .insert(Hero::default())
        // .insert(PlayerController::default())
        .insert(RtsController::default())
        .insert(Interacting { target: None })
        // Save
        .insert(SaveUnit)
        .insert(SaveTransform)
        //
        .insert(Name::new(format!("Hero ({id:?})")))
        .id()
}

#[derive(Debug, Default, Component, Reflect)]
pub struct RtsController {
    pub cursor_pos: Vec2,
    pub pointing_unit: Option<Entity>,
}

fn my_cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut rts_query: Query<&mut RtsController>,
) {
    // get the camera info and transform
    if let Ok((camera, camera_transform)) = q_camera.get_single() {
        // get the window that the camera is displaying to (or the primary window)
        let wnd = if let RenderTarget::Window(id) = camera.target {
            wnds.get(id).unwrap()
        } else {
            wnds.get_primary().unwrap()
        };

        // check if the cursor is inside the window and get its position
        if let Some(screen_pos) = wnd.cursor_position() {
            // get the size of the window
            let window_size = Vec2::new(wnd.width(), wnd.height());

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
            if let Ok(mut rts) = rts_query.get_single_mut() {
                rts.cursor_pos = world_pos;
            }
        }
    }
}

fn project_point(
    rapier_context: Res<RapierContext>,
    mut rts_query: Query<&mut RtsController>,
    unit_q: Query<Entity, With<Unit>>,
) {
    if let Ok(mut rts) = rts_query.get_single_mut() {
        // let point = Vec2::new(1.0, 2.0);
        let point = rts.cursor_pos;
        // let solid = true;
        let filter = QueryFilter::default();
        // warn!("TODO: filter to unit only");
        // TODO: filter to unit only

        // if let Some((entity, projection)) = rapier_context.project_point(&point, solid, filter) {
        //     // The collider closest to the point has this `handle`.
        //     println!(
        //         "Projected point on entity {:?}. Point projection: {}",
        //         entity, projection.point
        //     );
        //     println!(
        //         "Point was inside of the collider shape: {}",
        //         projection.is_inside
        //     );
        // }
        let mut unit = None;
        rapier_context.intersections_with_point(point, filter, |entity| {
            // Callback called on each collider with a shape containing the point.
            // println!("The entity {:?} contains the point.", entity);
            if unit_q.get(entity).is_ok() {
                unit = Some(entity);
                false
            } else {
                // Return `false` instead if we want to stop searching for other colliders containing this point.
                true
            }
        });
        rts.pointing_unit = unit;
    }
}

fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&RtsController, &mut UnitCommand)>,
) {
    if let Ok((controller, mut uc)) = query.get_single_mut() {
        if buttons.just_pressed(MouseButton::Right) {
            uc.action_id = SkillId::MoveTo;
            uc.target_position = Some(controller.cursor_pos);
        }
        if keys.just_pressed(KeyCode::A) || buttons.just_pressed(MouseButton::Right) {
            if controller.pointing_unit.is_some() {
                uc.action_id = SkillId::Attack;
                uc.target_position = Some(controller.cursor_pos);
                uc.target_unit = controller.pointing_unit;
            }
        }

        if keys.just_pressed(KeyCode::Q) {
            uc.action_id = SkillId::IceSpear;
            uc.target_position = Some(controller.cursor_pos);
            uc.target_unit = controller.pointing_unit;
        }
        if keys.just_pressed(KeyCode::W) {
            uc.action_id = SkillId::BurstFire;
            uc.target_position = Some(controller.cursor_pos);
            uc.target_unit = controller.pointing_unit;
        }
        if keys.just_pressed(KeyCode::E) {
            uc.action_id = SkillId::GhostLight;
            uc.target_position = Some(controller.cursor_pos);
            uc.target_unit = controller.pointing_unit;
        }

        // if buttons.just_released(MouseButton::Left) {}
        // if buttons.pressed(MouseButton::Right) {}
        // // we can check multiple at once with `.any_*`
        // if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        //     // Either the left or the right button was just pressed
        // }
    }
}

fn ui_example_system(
    mut egui_context: ResMut<EguiContext>,
    query: Query<(&Unit, &UnitActions, &Hero), With<Hero>>,
    action_query: Query<&Skill>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((unit, ua, hero)) = query.get_single() {
        egui::Window::new("Hello")
            .title_bar(false)
            .resizable(false)
            .fixed_pos(egui::pos2(0.0, 0.0))
            .show(egui_context.ctx_mut(), |ui| {
                ui.label("Hp");

                let progress = unit.hp as f32 / unit.hp_max as f32;
                let progress_bar = egui::ProgressBar::new(progress)
                    .desired_width(400.0)
                    .text(format!("{} / {}", unit.hp, unit.hp_max,));
                ui.add(progress_bar);

                ui.label("Mp");
                let progress = 0.0 / 100.0;
                let progress_bar = egui::ProgressBar::new(progress)
                    .desired_width(400.0)
                    // .text("progress bar text")
                    .show_percentage();
                ui.add(progress_bar);

                ui.label(format!("Atk: {}", unit.atk));

                ui.label(format!("Gold: {}", hero.gold));
            });

        // Filter out action: Move, Idle, Attack, etc
        let actions = ua
            .actions
            .iter()
            .filter(|&&e| {
                if let Ok(a) = action_query.get(e) {
                    a.action_id != SkillId::Idle
                        && a.action_id != SkillId::MoveTo
                        && a.action_id != SkillId::Attack
                        && a.action_id != SkillId::Dead
                        && a.action_id != SkillId::Stun
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();

        let mut uis = vec![];
        // for i in 0..5 {
        for i in 0..actions.len() {
            // Show action icon, level, short cut, cooldown(remain)
            // Show tooltip when hover, show action name, desc, level, shortcut, cooldown(setting)
            match actions.get(i) {
                Some(&&act) => {
                    if let Ok(action) = action_query.get(act) {
                        let setting = action.action_id.setting();

                        let texture_handle = asset_server.load(setting.icon);
                        let egui_id = egui_context.add_image(texture_handle.clone());
                        uis.push((
                            setting.name.to_string(),
                            action.level.to_string(),
                            setting.desc.to_string(),
                            egui_id,
                        ));
                    } else {
                        error!("Cannot get action for ui");
                        // Show Empty
                        let texture_handle = asset_server
                            .load("images/particlePack_1.1/PNG (Transparent)/circle_01.png");
                        let egui_id = egui_context.add_image(texture_handle.clone());
                        uis.push((String::new(), String::new(), String::new(), egui_id));
                    }
                }
                None => {
                    // Show Empty
                    let texture_handle = asset_server
                        .load("images/particlePack_1.1/PNG (Transparent)/circle_01.png");
                    let egui_id = egui_context.add_image(texture_handle.clone());
                    uis.push((String::new(), String::new(), String::new(), egui_id));
                }
            }
        }

        egui::Window::new("World")
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_BOTTOM, [0.0, 0.0])
            .show(egui_context.ctx_mut(), |ui| {
                egui::Grid::new("id_source")
                    .spacing([40.0, 4.0])
                    .show(ui, |ui| {
                        for (i, (name, level, desc, image)) in uis.iter().enumerate() {
                            ui.vertical_centered(|ui| {
                                ui.image(*image, [64.0, 64.0]).on_hover_ui(|ui| {
                                    ui.label(format!("{} Lv {} \n{}", name, level, desc));
                                });
                                ui.label(name);
                            });
                        }
                    });
            });
    }
}
