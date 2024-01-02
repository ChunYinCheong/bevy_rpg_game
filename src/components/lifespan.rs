use bevy::prelude::*;

#[derive(Debug, Component, Reflect)]
pub struct Lifespan {
    pub duration: f32,
}
