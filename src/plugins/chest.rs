use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

use crate::{ALL_GROUP, INTERACT_GROUP};

use super::{
    animation::{AnimationData, AnimationSheet, AnimationState, ChangeAnimation},
    item::{Inventory, ItemId},
    player::Hero,
    save::SaveChest,
    unit_action::UnitAnimation,
};

pub struct ChestPlugin;
impl Plugin for ChestPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // .add_system(block)
            .register_type::<Chest>()
            .add_event::<ChestEvent>()
            .add_system(open_chest)
            .add_system(update_sprite);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Component, Reflect, Default)]
pub struct Chest {
    pub opened: bool,
    pub item_id: ItemId,
}

pub fn spawn_chest(
    commands: &mut Commands,
    position: Vec2,

    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let animation_entity = {
        let texture_handle = asset_server.load("images/chest/chest.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 2, 1, None, None);
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
                animations: HashMap::from([
                    (
                        UnitAnimation::Idle.to_string(),
                        AnimationData {
                            start: 0,
                            len: 1,
                            frame_time: Duration::from_millis(500),
                            repeat: false,
                        },
                    ),
                    (
                        UnitAnimation::Walk.to_string(),
                        AnimationData {
                            start: 1,
                            len: 1,
                            frame_time: Duration::from_millis(500),
                            repeat: false,
                        },
                    ),
                ]),
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
            transform: Transform::from_translation(position.extend(1.0)),
            ..Default::default()
        })
        .insert(Name::new("Chest"))
        .insert(Chest {
            opened: false,
            item_id: ItemId::None,
        })
        // Interact
        .insert(super::interaction::Interaction::Chest)
        // Save
        .insert(SaveChest)
        // Rapier
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(32.0, 32.0))
        .insert(CollisionGroups::new(INTERACT_GROUP, ALL_GROUP))
        //
        .id()
}

#[derive(Debug)]
pub struct ChestEvent {
    pub chest: Entity,
}
pub fn open_chest(
    mut events: EventReader<ChestEvent>,
    mut chest_q: Query<&mut Chest>,
    mut player_q: Query<&mut Inventory, With<Hero>>,
) {
    for ev in events.iter() {
        info!("{ev:?}");
        if let Ok(mut chest) = chest_q.get_mut(ev.chest) {
            if !chest.opened {
                info!("{ev:?}, open chest: {chest:?}");
                chest.opened = true;
                if let Ok(mut inventory) = player_q.get_single_mut() {
                    match inventory.items.get_mut(&chest.item_id) {
                        Some(qty) => {
                            *qty += 1;
                        }
                        None => {
                            inventory.items.insert(chest.item_id, 1);
                        }
                    }
                }
            }
        }
    }
}

fn update_sprite(
    chest_q: Query<(Entity, &Chest), Changed<Chest>>,
    mut anim_events: EventWriter<ChangeAnimation>,
) {
    for (entity, chest) in chest_q.iter() {
        anim_events.send(ChangeAnimation {
            entity,
            name: if chest.opened {
                UnitAnimation::Walk.to_string()
            } else {
                UnitAnimation::Idle.to_string()
            },
        });
    }
}
