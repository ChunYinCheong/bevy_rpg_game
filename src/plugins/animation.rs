use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

use crate::res::GameWorldConfig;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animation)
            .add_event::<ChangeAnimation>()
            .add_system(change_animation)
            .register_inspectable::<AnimationState>();
    }
}

#[derive(Debug, Component)]
pub struct AnimationSheet {
    /// <name, (index, len)>
    pub animations: HashMap<String, AnimationData>,
}

#[derive(Debug)]
pub struct AnimationData {
    /// start offset
    pub start: usize,
    pub len: usize,
    pub frame_time: Duration,
    pub repeat: bool,
}

#[derive(Debug, Clone, Component, Inspectable, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnimationState {
    pub name: String,
    pub index: usize,
    // pub timer: Timer,
    pub duration: Duration,
}

pub fn animation(
    config: Res<GameWorldConfig>,
    time: Res<Time>,
    mut anim_q: Query<(
        &mut TextureAtlasSprite,
        &mut AnimationState,
        &AnimationSheet,
    )>,
) {
    if !config.active {
        return;
    }

    for (mut sprite, mut state, sheet) in anim_q.iter_mut() {
        // info!("animation! {:?}", state);
        if let Some(animation) = sheet.animations.get(&state.name) {
            state.duration += time.delta();
            if state.duration >= animation.frame_time {
                state.duration -= animation.frame_time;
                if animation.repeat {
                    state.index = (state.index + 1) % animation.len;
                    sprite.index = animation.start + state.index;
                } else {
                    state.index = animation.len - 1;
                    sprite.index = animation.start + state.index;
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ChangeAnimation {
    pub entity: Entity,
    pub name: String,
}
pub fn change_animation(
    mut events: EventReader<ChangeAnimation>,
    mut anim_q: Query<(
        &mut TextureAtlasSprite,
        &mut AnimationState,
        &AnimationSheet,
    )>,
) {
    for ev in events.iter() {
        // info!("{ev:?}");
        if let Ok((mut sprite, mut state, sheet)) = anim_q.get_mut(ev.entity) {
            state.name = ev.name.clone();
            state.duration = Duration::ZERO;
            state.index = 0;
            if let Some(animation) = sheet.animations.get(&state.name) {
                sprite.index = animation.start;
            } else {
                error!(
                    "Animation not found, Animation Name: {}, Entity: {:?}",
                    &state.name, ev.entity
                );
                sprite.index = 0;
            }
        }
    }
}
