#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_rapier2d::prelude::*;
use tracing_subscriber::{filter, prelude::*, EnvFilter};

mod components;
mod events;
mod plugins;
mod res;
mod states;
mod systems;
mod utils;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Title,
    Level,
    EscMenu,
}

fn fps_system(mut windows: ResMut<Windows>, diagnostics: Res<Diagnostics>) {
    let window = windows.get_primary_mut().unwrap();
    // println!("Window size was: {},{}", window.width(), window.height());

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            window.set_title(format!("FPS: {average:.0}"));
        }
    } else {
        window.set_title("FPS: None".to_string());
    }
}

pub const RAPIER_SCALE: f32 = 50.0;
pub const INTERACT_GROUP: Group = Group::GROUP_1;
pub const UNIT_GROUP: Group = Group::GROUP_2;
pub const ALL_GROUP: Group = Group::ALL;
pub const NONE_GROUP: Group = Group::NONE;

// fn editor() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_plugin(EguiPlugin)
//         // Inspector
//         .add_plugin(WorldInspectorPlugin::new())
//         // .add_plugin(InspectorPlugin::<SceneRes>::new())
//         // Editor
//         .add_plugin(plugins::scene_editor::scene_loader::SceneLoaderPlugin)
//         .add_plugin(plugins::scene_editor::editor::EditorPlugin)
//         .add_plugin(EditorComponentPlugin)
//         .register_editor_component::<Transform>()
//         // Camera
//         .add_plugin(PanCamPlugin::default())
//         .add_startup_system(|mut commands: Commands| {
//             commands
//                 .spawn(Camera2dBundle::default())
//                 .insert(PanCam::default());
//         })
//         // Tile Map
//         .add_plugin(plugins::tiled_asset::TiledAssetPlugin)
//         .add_plugin(plugins::tile_map::TileMapPlugin)
//         .add_system(
//             |camera: Query<(&Camera, &GlobalTransform), Changed<GlobalTransform>>,
//              mut load_map_events: EventWriter<plugins::tile_map::LoadMap>| {
//                 for (camera, tran) in camera.iter() {
//                     if !camera.is_active {
//                         continue;
//                     }

//                     let v = tran.translation();
//                     let chunk = plugins::game_world::WorldChunk::new(v.x, v.y);
//                     // debug!("v: {v:?} ,chunk: {chunk:?}");
//                     // Draw tile map
//                     load_map_events.send_batch(
//                         chunk
//                             .asset_load_path()
//                             .into_iter()
//                             .map(plugins::tile_map::LoadMap),
//                     );
//                     break;
//                 }
//             },
//         )
//         .run();
// }

fn main() {
    // let editor_mode = env::var("GAME_EDITOR")
    //     .map(|s| s == "true")
    //     .unwrap_or(false);
    // // let editor_mode = true;
    // if editor_mode {
    //     return editor();
    // }
    // let s = ron::ser::to_string(&plugins::game_world::GameObjectType::Blocker).unwrap();
    // println!("{s}");
    // return;

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);
    tracing_subscriber::registry()
        // .with(EnvFilter::from_default_env())
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(EnvFilter::from_env("BEVY_LOG"))
                .with_filter(filter::filter_fn(|metadata| {
                    !metadata.target().starts_with("bevy_rpg")
                })),
        )
        .with(fmt_layer.with_filter(EnvFilter::from_env("GAME_LOG")))
        .init();

    App::new()
        .add_state(AppState::Title)
        // .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins.build().disable::<bevy::log::LogPlugin>())
        // FPS
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // FPS
        .add_system(fps_system)
        // Rapier
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            RAPIER_SCALE,
        ))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, 0.0),
            ..Default::default()
        })
        // LDTK
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Identifier("Level_27".into()))
        // .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .register_ldtk_int_cell::<plugins::ldtk::components::WallBundle>(1)
        // .register_ldtk_int_cell::<components::LadderBundle>(2)
        // .register_ldtk_int_cell::<components::WallBundle>(3)
        .add_system(plugins::ldtk::entities::process_my_entity)
        .add_system(plugins::ldtk::systems::spawn_wall_collision)
        .add_system(plugins::ldtk::systems::update_level_selection)
        // Egui
        .add_plugin(EguiPlugin)
        // Inspector
        .add_plugin(InspectableRapierPlugin)
        .add_plugin(WorldInspectorPlugin)
        .register_type::<components::bullet::Bullet>()
        .register_type::<components::lifespan::Lifespan>()
        .register_type::<plugins::movement::Movement>()
        // Plugins
        // .add_plugin(plugins::game_world::GameWorldPlugin)
        .add_plugin(plugins::save::SavePlugin)
        // .add_plugin(plugins::tiled_asset::TiledAssetPlugin)
        // .add_plugin(plugins::tile_map::TileMapPlugin)
        .add_system(plugins::movement::update_movement)
        .add_plugin(plugins::unit::UnitPlugin)
        .add_plugin(plugins::unit_state::UnitStatePlugin)
        .add_plugin(plugins::unit_action::UnitActionPlugin)
        .add_plugin(plugins::actions::action::ActionPlugin)
        .add_plugin(plugins::wolf::WolfPlugin)
        .add_plugin(plugins::player::PlayerPlugin)
        .add_plugin(plugins::spider::SpiderPlugin)
        .add_plugin(plugins::animation::AnimationPlugin)
        .add_plugin(plugins::fox::FoxPlugin)
        .add_plugin(plugins::hit::HitPlugin)
        .add_plugin(plugins::damage::DamagePlugin)
        .add_plugin(plugins::knockback::KnockbackPlugin)
        .add_plugin(plugins::hook::HookPlugin)
        .add_plugin(plugins::interaction::InteractionPlugin)
        .add_plugin(plugins::shop::ShopPlugin)
        .add_plugin(plugins::item::ItemPlugin)
        .add_plugin(plugins::reset_point::ResetPointPlugin)
        .add_plugin(plugins::blocker::BlockerPlugin)
        .add_plugin(plugins::area::AreaPlugin)
        .add_plugin(plugins::chest::ChestPlugin)
        // .add_plugin(plugins::scene_editor::editor::EditorPlugin)
        // .add_plugin(plugins::scene_editor::scene_loader::SceneLoaderPlugin)
        .add_plugin(plugins::trigger::TriggerPlugin)
        .add_plugin(plugins::rogue::rogue::RoguePlugin)
        .add_plugin(plugins::visual_effect::VisualEffectPlugin)
        // utils
        .add_system(utils::sync_melee)
        // debug
        .add_system(systems::debug::display_events)
        // Duration
        .add_system(systems::lifespan_countdown)
        // Pause game
        .insert_resource(res::GameWorldConfig { active: true })
        .add_system_to_stage(CoreStage::PreUpdate, systems::game::pause_game)
        // .add_event::<events::BulletHitEvent>()
        // AppState
        // Title
        .add_system_set(SystemSet::on_update(AppState::Title).with_system(states::title::main_menu))
        // In game
        // .add_system_set(SystemSet::on_enter(AppState::Level).with_system(states::level::setup))
        .add_system_set(SystemSet::on_enter(AppState::Level).with_system(
            |mut commands: Commands, asset_server: Res<AssetServer>| {
                commands.spawn(Camera2dBundle::default());

                commands.spawn(LdtkWorldBundle {
                    ldtk_handle: asset_server.load("maps/test.ldtk"),
                    ..Default::default()
                });
            },
        ))
        .add_system_set(
            SystemSet::on_update(AppState::Level)
                .with_system(states::level::handle_input)
                .with_system(plugins::game_world::auto_save),
        )
        .add_system_set(SystemSet::on_exit(AppState::Level).with_system(states::level::cleanup))
        // Esc
        .add_system_set(
            SystemSet::on_update(AppState::EscMenu).with_system(states::esc_menu::esc_menu),
        )
        .run();
}
