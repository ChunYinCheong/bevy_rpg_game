use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionGroups;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};

use super::{
    animation::AnimationState,
    blocker::Blocker,
    chest::Chest,
    game_world::{Ecs, GameObjectId, GameObjectType},
    item::{Equipment, Inventory},
    spatial_map::SpatialMap,
    unit::Unit,
};

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_startup_system(load_save)
            .register_type::<SaveGameObjectType>()
            .add_system(save_game_object_type)
            .register_type::<SaveUnit>()
            .add_system(save_unit)
            .register_type::<SaveTransform>()
            .add_system(save_transform)
            .register_type::<ClearOnReset>()
            .add_system(save_reset)
            .register_type::<SaveBlocker>()
            .add_system(save_blocker)
            .register_type::<SaveInventory>()
            .add_system(save_inventory)
            .register_type::<SaveEquipment>()
            .add_system(save_equipment)
            .register_type::<SaveChest>()
            .add_system(save_chest)
            .register_type::<SaveCollisionGroups>()
            .add_system(save_collision_groups)
            .register_type::<SaveAnimationState>()
            .add_system(save_animation_state)
            .add_event::<WriteSaveFile>()
            .add_system(write_save_file)
            .add_event::<ClearSave>()
            .add_system(clear_save);
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Save {
    pub data: Ecs,
    pub map: SpatialMap,
}

#[derive(Debug, Resource)]
pub struct SaveBuffer(pub Save);

#[derive(Debug, Default, Component, Reflect)]
pub struct SaveGameObjectType;

#[derive(Debug, Default, Component, Reflect)]
pub struct SaveUnit;

#[derive(Debug, Default, Component, Reflect)]
pub struct SaveTransform;

pub fn save_game_object_type(
    query: Query<
        (&GameObjectType, &GameObjectId),
        (With<SaveGameObjectType>, Changed<GameObjectType>),
    >,
    mut save: ResMut<SaveBuffer>,
) {
    for (obj, id) in query.iter() {
        info!("save_game_object_type, id: {id:?}, obj: {obj:?}");
        if let Some(v) = save.0.data.objects.get(id) {
            if v == obj {
                continue;
            }
        }
        save.0.data.objects.insert(id.clone(), *obj);
    }
}

pub fn save_unit(
    query: Query<(&Unit, &GameObjectId), (With<SaveUnit>, Changed<Unit>)>,
    mut save: ResMut<SaveBuffer>,
) {
    for (unit, id) in query.iter() {
        // info!("save_unit, id: {id:?}, unit: {unit:?}");
        if let Some(v) = save.0.data.units.get(id) {
            if v == unit {
                continue;
            }
        }
        save.0.data.units.insert(id.clone(), unit.clone());
    }
}

pub fn save_transform(
    query: Query<(&Transform, &GameObjectId), (With<SaveTransform>, Changed<Transform>)>,
    mut save: ResMut<SaveBuffer>,
) {
    for (transform, id) in query.iter() {
        // info!("id: {id:?}, transform: {transform:?}");
        if let Some((translation, rotation, scale)) = save.0.data.transforms.get(id) {
            if &transform.translation == translation
                && &transform.rotation == rotation
                && &transform.scale == scale
            {
                continue;
            }
        }
        // info!("Transform Changed: id: {id:?}, transform: {transform:?}");
        save.0.data.transforms.insert(
            id.clone(),
            (transform.translation, transform.rotation, transform.scale),
        );
        save.0
            .map
            .update(id.clone(), transform.translation.truncate().into())
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct SaveBlocker;
pub fn save_blocker(
    query: Query<(&Blocker, &GameObjectId), (With<SaveBlocker>, Changed<Blocker>)>,
    mut save: ResMut<SaveBuffer>,
) {
    for (obj, id) in query.iter() {
        info!("save_blocker, id: {id:?}, obj: {obj:?}");
        if let Some(v) = save.0.data.blockers.get(id) {
            if v == obj {
                continue;
            }
        }
        save.0.data.blockers.insert(id.clone(), obj.clone());
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct SaveInventory;
pub fn save_inventory(
    query: Query<(&Inventory, &GameObjectId), (With<SaveInventory>, Changed<Inventory>)>,
    mut save: ResMut<SaveBuffer>,
) {
    for (obj, id) in query.iter() {
        info!("save_inventory, id: {id:?}, obj: {obj:?}");
        if let Some(v) = save.0.data.inventorys.get(id) {
            if v == obj {
                continue;
            }
        }
        save.0.data.inventorys.insert(id.clone(), obj.clone());
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct SaveEquipment;
pub fn save_equipment(
    query: Query<(&Equipment, &GameObjectId), (With<SaveEquipment>, Changed<Equipment>)>,
    mut save: ResMut<SaveBuffer>,
) {
    for (obj, id) in query.iter() {
        info!("save_equipment, id: {id:?}, obj: {obj:?}");
        if let Some(v) = save.0.data.equipments.get(id) {
            if v == obj {
                continue;
            }
        }
        save.0.data.equipments.insert(id.clone(), obj.clone());
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct SaveChest;
pub fn save_chest(
    query: Query<(&Chest, &GameObjectId), (With<SaveChest>, Changed<Chest>)>,
    mut save: ResMut<SaveBuffer>,
) {
    for (obj, id) in query.iter() {
        info!("save_chest, id: {id:?}, obj: {obj:?}");
        if let Some(v) = save.0.data.chests.get(id) {
            if v == obj {
                continue;
            }
        }
        save.0.data.chests.insert(id.clone(), obj.clone());
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct SaveCollisionGroups;
pub fn save_collision_groups(
    query: Query<
        (&CollisionGroups, &GameObjectId),
        (With<SaveCollisionGroups>, Changed<CollisionGroups>),
    >,
    mut save: ResMut<SaveBuffer>,
) {
    for (obj, id) in query.iter() {
        info!("save_collision_groups, id: {id:?}, obj: {obj:?}");
        if let Some(v) = save.0.data.collision_groupss.get(id) {
            if v.0 == obj.memberships.bits() && v.1 == obj.filters.bits() {
                continue;
            }
        }
        save.0
            .data
            .collision_groupss
            .insert(id.clone(), (obj.memberships.bits(), obj.filters.bits()));
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct SaveAnimationState;
pub fn save_animation_state(
    query: Query<
        (&AnimationState, &GameObjectId),
        (With<SaveAnimationState>, Changed<AnimationState>),
    >,
    mut save: ResMut<SaveBuffer>,
) {
    for (obj, id) in query.iter() {
        // info!("save_animation_state, id: {id:?}, obj: {obj:?}");
        if let Some(v) = save.0.data.animation_states.get(id) {
            if v == obj {
                continue;
            }
        }
        save.0.data.animation_states.insert(id.clone(), obj.clone());
    }
}

pub fn load_save(mut commands: Commands) {
    {
        // Save template
        let data = Save::default();
        let pretty = ron::ser::PrettyConfig::new()
            // .depth_limit(2)
            .separate_tuple_members(true)
            .enumerate_arrays(true);
        let s = ron::ser::to_string_pretty(&data, pretty).expect("Serialization failed");
        // info!("s: {s:?}");
        let path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("assets/save/save_template.ron");
        std::fs::write(path, s).expect("Unable to write file");
    }

    // Read save file
    let path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        // .join(e.0);
        .join("assets/save/save.ron");
    let f = File::open(path).expect("Failed opening file");
    let save: Save = match ron::de::from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load Save: {e}");

            std::process::exit(1);
        }
    };
    // info!("Save: {save:?}");
    commands.insert_resource(SaveBuffer(save.clone()));
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Component, Reflect,
)]
pub struct ClearOnReset;
pub fn save_reset(
    query: Query<(&ClearOnReset, &GameObjectId), Changed<ClearOnReset>>,
    mut save: ResMut<SaveBuffer>,
) {
    for (reset, id) in query.iter() {
        info!("save_reset, id: {id:?}, ClearOnReset: {ClearOnReset:?}");
        if let Some(v) = save.0.data.resets.get(id) {
            if v == reset {
                continue;
            }
        }
        save.0.data.resets.insert(id.clone(), reset.clone());
    }
}

pub struct ClearSave;
fn clear_save(
    mut events: EventReader<ClearSave>,
    query: Query<&GameObjectId, With<ClearOnReset>>,
    mut cache: ResMut<SaveBuffer>,
    mut write_events: EventWriter<WriteSaveFile>,
) {
    for _ in events.iter() {
        info!("Clear Save");
        for id in query.iter() {
            cache.0.data.objects.remove(id);
            cache.0.data.transforms.remove(id);
            cache.0.data.units.remove(id);
            cache.0.data.resets.remove(id);
            cache.0.map.remove(id);
        }
        let keys = cache.0.data.resets.keys().cloned().collect::<Vec<_>>();
        for id in keys.iter() {
            cache.0.data.objects.remove(id);
            cache.0.data.transforms.remove(id);
            cache.0.data.units.remove(id);
            cache.0.data.resets.remove(id);
            cache.0.map.remove(id);
        }
        write_events.send(WriteSaveFile);
    }
}

pub struct WriteSaveFile;
pub fn write_save_file(mut events: EventReader<WriteSaveFile>, cache: Res<SaveBuffer>) {
    let mut skip = false;
    for _ in events.iter() {
        if skip {
            continue;
        }
        skip = true;
        // info!("Write save to file...");
        let pretty = ron::ser::PrettyConfig::new()
            // .depth_limit(2)
            .separate_tuple_members(true)
            .enumerate_arrays(true);
        let s = ron::ser::to_string_pretty(&cache.0, pretty).expect("Serialization failed");
        // info!("s: {s:?}");
        let path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("assets/save/save.ron");
        std::fs::write(path, s).expect("Unable to write file");
    }
}
