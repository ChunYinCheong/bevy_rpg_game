use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

use super::{blocker::Blocker, game_world::GameObjectId, player::Player};

pub struct AreaPlugin;
impl Plugin for AreaPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_system(attach)
            .add_system(area_detection)
            .register_inspectable::<Area>()
            // .add_event::<HitEvent>()
            ;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Component, Inspectable)]
pub struct Area {
    pub event: AreaEvent,
    pub hx: f32,
    pub hy: f32,
    pub disable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Inspectable)]
pub enum AreaEvent {
    None,
    Unknown,
    EventA,
}
impl From<&str> for AreaEvent {
    fn from(s: &str) -> Self {
        match s {
            "EventA" => AreaEvent::EventA,
            "" => {
                warn!("Empty AreaEvent name!");
                AreaEvent::None
            }
            _ => {
                error!("Unknown AreaEvent name: {}", s);
                AreaEvent::Unknown
            }
        }
    }
}

pub fn spawn_area(
    commands: &mut Commands,
    position: Vec2,

    _asset_server: &mut Res<AssetServer>,
    _texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    commands
        .spawn()
        .insert_bundle(SpatialBundle {
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        })
        .insert(Name::new("Area"))
        // .insert(Area {
        //     event: AreaEvent::EventA,
        // })
        // Rapier
        // .insert(RigidBody::Fixed)
        // .insert(Sensor(true))
        // .insert(ActiveEvents::COLLISION_EVENTS)
        // .insert(Collider::cuboid(hx, hy))
        //
        .id()
}
pub fn attach(mut commands: Commands, query: Query<(Entity, &Area), Changed<Area>>) {
    for (entity, a) in query.iter() {
        commands
            .entity(entity)
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(a.hx, a.hy))
            .insert(Sensor)
            .insert(ActiveEvents::COLLISION_EVENTS);
    }
}

pub fn area_detection(
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<&Area>,
    player_query: Query<&Player>,
    mut blocker_query: Query<(&mut Blocker, &GameObjectId)>,
) {
    for collision_event in collision_events.iter() {
        info!("Received collision event: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(e1, e2, _flag) => {
                let area_event = {
                    let mut temp = None;
                    if let Ok(area) = query.get(*e1) {
                        if player_query.get(*e2).is_ok() {
                            temp = Some(area.event);
                        }
                    } else if let Ok(area) = query.get(*e2) {
                        if player_query.get(*e1).is_ok() {
                            temp = Some(area.event);
                        }
                    }
                    temp
                };
                if let Some(ae) = area_event {
                    match ae {
                        AreaEvent::EventA => {
                            info!("EventA");
                            for (mut b, id) in blocker_query.iter_mut() {
                                match id {
                                    GameObjectId::Tiled { x, y, id } => {
                                        if *x == 1 && *y == -2 {
                                            if *id == 11 || *id == 12 || *id == 13 {
                                                b.blocking = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        AreaEvent::None => {
                            info!("No AreaEvent");
                        }
                        AreaEvent::Unknown => {
                            error!("Unknown AreaEvent")
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
