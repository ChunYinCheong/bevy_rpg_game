use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

use super::{
    animation::{AnimationData, AnimationSheet, AnimationState},
    save::SaveBlocker,
    unit_action::UnitAnimation,
};

pub struct BlockerPlugin;
impl Plugin for BlockerPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_system(attach)
            .register_type::<Blocker>()
            // .add_event::<HitEvent>()
            ;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Component, Reflect, Default)]
pub struct Blocker {
    pub blocking: bool,
    pub hx: f32,
    pub hy: f32,
}

pub fn spawn_blocker(
    commands: &mut Commands,
    position: Vec2,

    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let animation_entity = {
        let texture_handle = asset_server.load("images/player/spritesheet.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 10, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
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
        // .spawn()
        .entity(animation_entity)
        .insert(SpatialBundle {
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        })
        .insert(Name::new("Blocker"))
        // Save
        .insert(SaveBlocker)
        // Rapier
        // .insert(RigidBody::Fixed)
        // .insert(Collider::cuboid(hx, hy))
        //
        .id()
}

fn attach(mut commands: Commands, query: Query<(Entity, &Blocker), Changed<Blocker>>) {
    for (entity, a) in query.iter() {
        if a.blocking {
            commands
                .entity(entity)
                .insert(RigidBody::Fixed)
                // .insert(Collider::cuboid(a.hx, a.hy))
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
