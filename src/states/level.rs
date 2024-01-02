use bevy::prelude::*;

use crate::plugins::game_world::{GameObjectId, LoadObject};

#[derive(Resource)]
pub struct LevelRoot(Entity);

pub fn setup(mut commands: Commands, mut load_events: EventWriter<LoadObject>) {
    let root_entity = commands
        .spawn(SpatialBundle::default())
        .insert(Name::new("Level Root"))
        .id();
    commands.insert_resource(LevelRoot(root_entity));

    {
        // Camera
        let child = commands.spawn(Camera2dBundle::default()).id();
        commands.entity(root_entity).add_child(child);
    }
    {
        // Load player
        load_events.send(LoadObject(GameObjectId("player".into())));
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
