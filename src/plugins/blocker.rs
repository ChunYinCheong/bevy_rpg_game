use std::collections::HashMap;

use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    animation::{AnimationEntity, AnimationIndex, AnimationSheet, AnimationState, AnimationTimer},
    save::SaveBlocker,
    unit_action::UnitAnimation,
};

pub struct BlockerPlugin;
impl Plugin for BlockerPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_system(block)
            .register_inspectable::<Blocker>()
            // .add_event::<HitEvent>()
            ;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Component, Inspectable)]
pub struct Blocker {
    pub blocking: bool,
}

pub fn spawn_blocker(
    commands: &mut Commands,
    position: Vec2,

    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let animation_entity = {
        let texture_handle = asset_server.load("images/player/spritesheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 10, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    rotation: Default::default(),
                    // scale: Vec3::new(SCALE, SCALE, SCALE),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(AnimationTimer(Timer::from_seconds(0.5, true)))
            .insert(AnimationSheet {
                animations: HashMap::from([
                    (UnitAnimation::Idle.to_string(), (0, 1)),
                    (UnitAnimation::Walk.to_string(), (1, 2)),
                    (UnitAnimation::Run.to_string(), (3, 2)),
                    (UnitAnimation::Attack.to_string(), (5, 1)),
                    (UnitAnimation::Stab.to_string(), (6, 1)),
                    (UnitAnimation::BurstFire.to_string(), (7, 1)),
                    (UnitAnimation::Hook.to_string(), (8, 1)),
                ]),
            })
            .insert(AnimationState {
                animation: UnitAnimation::Idle.to_string(),
            })
            .insert(AnimationIndex::default())
            .id()
    };
    commands
        .spawn()
        .insert_bundle(SpatialBundle {
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        })
        .insert(Name::new("Blocker"))
        .insert(Blocker { blocking: false })
        // Save
        .insert(SaveBlocker)
        // Sprite
        .add_child(animation_entity)
        .insert(AnimationEntity(animation_entity))
        // Rapier
        // .insert(RigidBody::Fixed)
        // .insert(Collider::cuboid(hx, hy))
        //
        .id()
}

pub fn block(mut commands: Commands, query: Query<(Entity, &Blocker), Changed<Blocker>>) {
    for (entity, a) in query.iter() {
        if a.blocking {
            commands
                .entity(entity)
                .insert(RigidBody::Fixed)
                .insert(Collider::cuboid(32.0, 32.0))
                .insert(Visibility { is_visible: true });
        } else {
            commands
                .entity(entity)
                .remove::<RigidBody>()
                .remove::<Collider>()
                .insert(Visibility { is_visible: false });
        }
    }
}
