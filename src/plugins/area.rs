use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

use super::player::Hero;

pub struct AreaPlugin;
impl Plugin for AreaPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_system(attach)
            .add_system(area_detection)
            .register_type::<Area>()
            .add_event::<PlayerEnterEvent>();
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Component, Reflect, Default)]
pub struct Area {
    pub hx: f32,
    pub hy: f32,
    pub disable: bool,
}

pub fn spawn_area(
    commands: &mut Commands,
    position: Vec2,

    _asset_server: &Res<AssetServer>,
    _texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let id = commands
        .spawn(SpatialBundle {
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        })
        // .insert(Name::new("Area"))
        // Rapier
        // .insert(RigidBody::Fixed)
        // .insert(Sensor(true))
        // .insert(ActiveEvents::COLLISION_EVENTS)
        // .insert(Collider::cuboid(hx, hy))
        //
        .id();
    commands
        .entity(id)
        .insert(Name::new(format!("Area ({id:?})")));
    id
}
fn attach(mut commands: Commands, query: Query<(Entity, &Area), Changed<Area>>) {
    for (entity, _a) in query.iter() {
        commands
            .entity(entity)
            .insert(RigidBody::Fixed)
            // area collider is scaled by ldtk plugins
            .insert(Collider::cuboid(32.0, 32.0))
            // .insert(Collider::cuboid(a.hx , a.hy))
            .insert(Sensor)
            .insert(ActiveEvents::COLLISION_EVENTS);
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct PlayerEnterEvent(pub Entity);

impl Default for PlayerEnterEvent {
    fn default() -> Self {
        Self(Entity::from_raw(0))
    }
}
pub fn area_detection(
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<&Area>,
    player_query: Query<&Hero>,
    mut ev: EventWriter<PlayerEnterEvent>,
) {
    for collision_event in collision_events.iter() {
        // info!("Received collision event: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(e1, e2, _flag) => {
                if let Ok(area) = query.get(*e1) {
                    if !area.disable && player_query.get(*e2).is_ok() {
                        ev.send(PlayerEnterEvent(*e1));
                    }
                } else if let Ok(area) = query.get(*e2) {
                    if !area.disable && player_query.get(*e1).is_ok() {
                        ev.send(PlayerEnterEvent(*e2));
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
