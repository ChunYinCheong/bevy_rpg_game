use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use std::collections::HashMap;

use crate::res::GameWorldConfig;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animation)
            .add_system(animation_changed)
            .register_inspectable::<AnimationEntity>()
            .register_inspectable::<AnimationState>()
            .register_inspectable::<AnimationIndex>();
    }
}

#[derive(Debug, Component, Inspectable)]
pub struct AnimationEntity(pub Entity);

#[derive(Debug, Component)]
pub struct AnimationSheet {
    /// <enum, (index, len)>
    pub animations: HashMap<String, (usize, usize)>,
}

#[derive(Debug, Component, Inspectable)]
pub struct AnimationState {
    pub animation: String,
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct AnimationIndex {
    pub index: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animation(
    config: Res<GameWorldConfig>,
    time: Res<Time>,
    mut anim_q: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut AnimationIndex,
        &AnimationState,
        &AnimationSheet,
    )>,
) {
    if !config.active {
        return;
    }

    for (mut timer, mut sprite, mut ai, state, sheet) in anim_q.iter_mut() {
        // info!("animation! {:?}", state);
        timer.tick(time.delta());
        if timer.finished() {
            // info!("animation timer finished! {:?}", state);
            let (offset, len) = sheet.animations.get(&state.animation).unwrap_or(&(0, 1));
            ai.index = (ai.index + 1) % len;
            sprite.index = offset + ai.index;
        }
    }
}

pub fn animation_changed(
    config: Res<GameWorldConfig>,
    mut anim_q: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &mut AnimationIndex,
            &AnimationState,
            &AnimationSheet,
        ),
        Changed<AnimationState>,
    >,
) {
    if !config.active {
        return;
    }

    for (mut timer, mut sprite, mut ai, state, sheet) in anim_q.iter_mut() {
        // info!("Changed! {:?}", state);
        timer.reset();
        let (offset, _) = sheet.animations.get(&state.animation).unwrap_or(&(0, 1));
        ai.index = 0;
        sprite.index = *offset;
    }
}
