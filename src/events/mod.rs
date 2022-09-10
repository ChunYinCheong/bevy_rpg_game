use bevy::prelude::Entity;

#[derive(Debug)]
pub struct BulletHitEvent {
    pub bullet: Entity,
    pub other: Entity,
}
