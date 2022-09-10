use std::collections::HashMap;

use bevy::prelude::*;
// use bevy_inspector_egui::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{INTERACT_GROUP, RAPIER_SCALE};

use super::{
    animation::{AnimationEntity, AnimationIndex, AnimationSheet, AnimationState, AnimationTimer},
    interaction::Interaction,
    unit_action::UnitAnimation,
};

pub struct ResetPointPlugin;

impl Plugin for ResetPointPlugin {
    fn build(&self, _app: &mut App) {
        // app
        //
        // .add_system(hit_detection)
        // .register_inspectable::<Hook>()
        // .register_inspectable::<Hooked>()
        // .add_system(on_hit)
        // .add_system(hook)
        // .add_event::<HitEvent>()
        // ;
    }
}

pub fn spawn_reset_point(
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
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        })
        .insert(Name::from("Reset Point"))
        .insert(RigidBody::Fixed)
        .insert(Collider::ball(0.5 * RAPIER_SCALE))
        .insert(CollisionGroups::new(INTERACT_GROUP, u32::MAX))
        .insert(Interaction::ResetPoint)
        .add_child(animation_entity)
        .insert(AnimationEntity(animation_entity))
        .id()
}
