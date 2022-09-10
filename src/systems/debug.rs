use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/* A system that displays the events. */
pub fn display_events(mut collision_events: EventReader<CollisionEvent>) {
    for collision_event in collision_events.iter() {
        debug!("Received collision event: {:?}", collision_event);
    }
}
