use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
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
            window.set_title(format!("FPS: {:.0}", average));
        }
    } else {
        window.set_title("FPS: None".to_string());
    }
}

pub const RAPIER_SCALE: f32 = 50.0;
pub const INTERACT_GROUP: u32 = 0b0001;
pub const UNIT_GROUP: u32 = 0b0010;

fn main() {
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
        .add_plugins_with(DefaultPlugins, |plugins| {
            plugins.disable::<bevy::log::LogPlugin>()
        })
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
        .add_plugin(EguiPlugin)
        // Inspector
        .add_plugin(InspectableRapierPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<components::bullet::Bullet>()
        .register_inspectable::<components::lifespan::Lifespan>()
        .register_inspectable::<plugins::movement::Movement>()
        // Plugins
        .add_plugin(plugins::game_world::GameWorldPlugin)
        .add_plugin(plugins::save::SavePlugin)
        .add_plugin(plugins::tiled_asset::TiledAssetPlugin)
        .add_plugin(plugins::tile_map::TileMapPlugin)
        .add_system(plugins::movement::update_movement)
        .add_plugin(plugins::unit::UnitPlugin)
        .add_plugin(plugins::unit_state::UnitStatePlugin)
        .add_plugin(plugins::unit_action::UnitActionPlugin)
        .add_plugin(plugins::wolf::WolfPlugin)
        .add_plugin(plugins::player::PlayerPlugin)
        .add_plugin(plugins::spider::SpiderPlugin)
        .add_plugin(plugins::animation::AnimationPlugin)
        .add_plugin(plugins::fox::FoxPlugin)
        .add_plugin(plugins::damage::DamagePlugin)
        .add_plugin(plugins::knockback::KnockbackPlugin)
        .add_plugin(plugins::hook::HookPlugin)
        .add_plugin(plugins::interaction::InteractionPlugin)
        .add_plugin(plugins::shop::ShopPlugin)
        .add_plugin(plugins::item::ItemPlugin)
        .add_plugin(plugins::reset_point::ResetPointPlugin)
        .add_plugin(plugins::blocker::BlockerPlugin)
        .add_plugin(plugins::area::AreaPlugin)
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
        .add_system_set(SystemSet::on_enter(AppState::Level).with_system(states::level::setup))
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
