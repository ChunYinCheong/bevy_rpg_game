use bevy::prelude::*;
use std::{collections::HashMap, time::Duration};
// use bevy_inspector_egui::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{INTERACT_GROUP, RAPIER_SCALE};

use super::{
    animation::{AnimationData, AnimationSheet, AnimationState},
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
        let texture_handle = asset_server.load("images/reset_point/reset_point.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 2, 1);
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
            .insert(AnimationSheet {
                animations: HashMap::from([(
                    UnitAnimation::Idle.to_string(),
                    AnimationData {
                        start: 0,
                        len: 1,
                        frame_time: Duration::from_millis(500),
                        repeat: true,
                    },
                )]),
            })
            .insert(AnimationState {
                name: UnitAnimation::Idle.to_string(),
                index: 0,
                duration: Duration::ZERO,
            })
            .id()
    };
    commands
        .entity(animation_entity)
        .insert_bundle(SpatialBundle {
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        })
        .insert(Name::from("Reset Point"))
        .insert(RigidBody::Fixed)
        .insert(Collider::ball(0.5 * RAPIER_SCALE))
        .insert(CollisionGroups::new(INTERACT_GROUP, u32::MAX))
        .insert(Interaction::ResetPoint)
        .id()
}
