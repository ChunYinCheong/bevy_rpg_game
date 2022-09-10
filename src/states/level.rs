use bevy::prelude::*;

use crate::plugins::game_world::LoadPosition;

pub struct LevelRoot(Entity);

pub fn setup(mut commands: Commands, mut load_events: EventWriter<LoadPosition>) {
    let root_entity = commands
        .spawn_bundle(SpatialBundle::default())
        .insert(Name::new("Level Root"))
        .id();
    commands.insert_resource(LevelRoot(root_entity));

    {
        // Camera
        let child = commands.spawn_bundle(Camera2dBundle::default()).id();
        commands.entity(root_entity).add_child(child);
    }
    {
        // Init game world
        load_events.send(LoadPosition(2720.0, 1200.0));
    }
}

pub fn cleanup(mut commands: Commands, root: Res<LevelRoot>) {
    commands.entity(root.0).despawn_recursive();
}

pub fn handle_input(
    mut keys: ResMut<Input<KeyCode>>,
    // mut commands: Commands,
    // mut app_state: ResMut<State<AppState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        // commands.insert_resource(LevelConfig {
        //     level: "Test".into(),
        // });
        // app_state.push(AppState::EscMenu).unwrap();
        keys.reset(KeyCode::Escape);
    }
}
