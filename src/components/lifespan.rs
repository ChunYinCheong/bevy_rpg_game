use bevy::prelude::Component;
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Component, Inspectable)]
pub struct Lifespan {
    pub duration: f32,
}
