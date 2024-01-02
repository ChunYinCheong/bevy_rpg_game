use bevy::prelude::*;

use crate::components::lifespan::Lifespan;

pub struct VisualEffectPlugin;
impl Plugin for VisualEffectPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<VisualEffectMarker>()
            .register_type::<VisualEffect>()
            .register_type::<AnimationIndices>()
            .register_type::<AnimationTimer>()
            .register_type::<AnimationRepeat>()
            .add_system(animate_sprite)
            .add_system(visual_effect_marker);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
pub enum VisualEffect {
    FrostBall,
    SmashWave,
    Diffusion,
    DeadFinger,
    Heal,
}

#[derive(Component, Reflect)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut, Reflect)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut, Reflect)]
struct AnimationRepeat(bool);

#[derive(Component, Reflect)]
struct DespawnAnimationWhenEnd;

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &AnimationRepeat,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        Option<&DespawnAnimationWhenEnd>,
        Entity,
    )>,
    mut commands: Commands,
) {
    for (indices, repeat, mut timer, mut sprite, despawn, entity) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if sprite.index >= indices.last {
                if repeat.0 {
                    sprite.index = indices.first;
                } else if despawn.is_some() {
                    commands.entity(entity).despawn_recursive();
                }
            } else {
                sprite.index += 1;
            }
        }
    }
}

#[derive(Debug, Clone, Reflect, Component)]
pub struct VisualEffectMarker {
    pub visual_effect: VisualEffect,
    pub pos: Vec2,
    pub size: Vec2,
    pub repeat: bool,
    pub auto_despawn: bool,
    pub duration: Option<f32>,
}

fn visual_effect_marker(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    query: Query<(Entity, &VisualEffectMarker, Option<&Transform>), Changed<VisualEffectMarker>>,
) {
    for (entity, marker, transform) in query.iter() {
        let (texture_atlas_handle, animation_indices) = match marker.visual_effect {
            VisualEffect::FrostBall => {
                let texture_handle = asset_server.load("images/Classic/Preview.png");
                let texture_atlas = TextureAtlas::from_grid(
                    texture_handle,
                    Vec2::new(125.0, 150.0),
                    6,
                    5,
                    None,
                    None,
                );
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                let animation_indices = AnimationIndices { first: 0, last: 5 };
                (texture_atlas_handle, animation_indices)
            }
            VisualEffect::SmashWave => {
                let texture_handle = asset_server.load("images/Classic/Preview.png");
                let texture_atlas = TextureAtlas::from_grid(
                    texture_handle,
                    Vec2::new(125.0, 150.0),
                    6,
                    5,
                    None,
                    None,
                );
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                let animation_indices = AnimationIndices { first: 6, last: 11 };
                (texture_atlas_handle, animation_indices)
            }
            VisualEffect::Diffusion => {
                let texture_handle = asset_server.load("images/Classic/Preview.png");
                let texture_atlas = TextureAtlas::from_grid(
                    texture_handle,
                    Vec2::new(125.0, 150.0),
                    6,
                    5,
                    None,
                    None,
                );
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                let animation_indices = AnimationIndices {
                    first: 12,
                    last: 17,
                };
                (texture_atlas_handle, animation_indices)
            }
            VisualEffect::DeadFinger => {
                let texture_handle = asset_server.load("images/Classic/Preview.png");
                let texture_atlas = TextureAtlas::from_grid(
                    texture_handle,
                    Vec2::new(125.0, 150.0),
                    6,
                    5,
                    None,
                    None,
                );
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                let animation_indices = AnimationIndices {
                    first: 18,
                    last: 23,
                };
                (texture_atlas_handle, animation_indices)
            }
            VisualEffect::Heal => {
                let texture_handle = asset_server.load("images/Classic/Preview.png");
                let texture_atlas = TextureAtlas::from_grid(
                    texture_handle,
                    Vec2::new(125.0, 150.0),
                    6,
                    5,
                    None,
                    None,
                );
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                let animation_indices = AnimationIndices {
                    first: 24,
                    last: 29,
                };
                (texture_atlas_handle, animation_indices)
            }
        };

        commands.entity(entity).insert((
            Name::new(format!(
                "VisualEffect {:?} ({entity:?})",
                marker.visual_effect
            )),
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite {
                    index: animation_indices.first,
                    custom_size: Some(marker.size),
                    ..Default::default()
                },
                transform: transform
                    .cloned()
                    .unwrap_or(Transform::from_translation(marker.pos.extend(1.0))),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            AnimationRepeat(marker.repeat),
        ));

        if let Some(duration) = marker.duration {
            commands.entity(entity).insert(Lifespan { duration });
        }

        if marker.auto_despawn {
            commands.entity(entity).insert(DespawnAnimationWhenEnd);
        }
    }
}
