use bevy::prelude::*;

use crate::components::lifespan::Lifespan;

pub mod debug;
pub mod game;

pub fn lifespan_countdown(
    mut lifespan_q: Query<(Entity, &mut Lifespan)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut d) in lifespan_q.iter_mut() {
        d.duration -= time.delta_seconds();
        if d.duration <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
