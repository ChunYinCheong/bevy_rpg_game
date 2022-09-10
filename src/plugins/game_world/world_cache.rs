use super::super::spatial_map::SpatialMap;
use super::super::tiled_asset::TiledAsset;
use super::super::unit::Unit;
use super::Ecs;
use super::GameObjectId;
use super::GameObjectType;
use super::WorldChunk;
use crate::plugins::area::Area;
use crate::plugins::area::AreaEvent;
use crate::plugins::blocker::Blocker;
use crate::plugins::game_world::MAP_SIZE;
use crate::plugins::item::Equipment;
use crate::plugins::item::Inventory;
use crate::plugins::save::SaveBuffer;
use crate::plugins::spatial_map::TILE_SIZE;
use bevy::prelude::*;
use std;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct WorldCachePlugin;
impl Plugin for WorldCachePlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .init_resource::<WorldCache>()
            .add_system(load_cache);
    }
}

#[derive(Debug, Default)]
pub struct WorldCache {
    pub map: SpatialMap,
    pub map_ecs: Ecs,
    pub loading_chunk: HashSet<WorldChunk>,
    pub loaded_chunk: HashSet<WorldChunk>,
    pub chunk_tiled_files: HashMap<WorldChunk, HashSet<Handle<TiledAsset>>>,

    pub loading_tileds: HashSet<Handle<TiledAsset>>,
    pub loaded_tileds: HashSet<Handle<TiledAsset>>,
    /// All handles, include both loaded and loading.
    ///
    /// key is asset_load_path.
    /// if option is none mean no file exists
    pub tiled_handles: HashMap<String, Option<Handle<TiledAsset>>>,
}

impl WorldCache {
    pub fn load(&mut self, chunk: WorldChunk) {
        if !self.loaded_chunk.contains(&chunk) {
            self.loading_chunk.insert(chunk);
        }
    }

    pub fn ready(&self, chunk: &WorldChunk) -> bool {
        match self.chunk_tiled_files.get(chunk) {
            Some(files) => {
                // info!("ready, chunk: {chunk:?} ,files: {files:?}");
                // info!("ready, self.loading_tileds: {:?}", &self.loading_tileds);
                return self.loading_tileds.is_disjoint(files);
            }
            None => return false,
        }
    }

    pub fn get_objects(&self, save: &SaveBuffer, chunk: &(i32, i32)) -> Vec<GameObjectId> {
        let mut results = save.0.map.get_objects(chunk).cloned().unwrap_or_default();
        if let Some(objs) = self.map.get_objects(chunk) {
            let mut ids = objs
                .iter()
                .filter(|id| !results.contains(*id) && save.0.map.get_chunk(id) != Some(chunk))
                .cloned()
                .collect();
            results.append(&mut ids);
        }
        results
    }

    pub fn get_object_type(&self, save: &SaveBuffer, id: &GameObjectId) -> Option<GameObjectType> {
        save.0
            .data
            .objects
            .get(id)
            .or(self.map_ecs.objects.get(id))
            .cloned()
    }
    pub fn get_unit(&self, save: &SaveBuffer, id: &GameObjectId) -> Option<Unit> {
        save.0
            .data
            .units
            .get(id)
            .or(self.map_ecs.units.get(id))
            .cloned()
    }
    pub fn get_transform(
        &self,
        save: &SaveBuffer,
        id: &GameObjectId,
    ) -> Option<(Vec3, Quat, Vec3)> {
        save.0
            .data
            .transforms
            .get(id)
            .or(self.map_ecs.transforms.get(id))
            .cloned()
    }
    pub fn get_blocker(&self, save: &SaveBuffer, id: &GameObjectId) -> Option<Blocker> {
        save.0
            .data
            .blockers
            .get(id)
            .or(self.map_ecs.blockers.get(id))
            .cloned()
    }
    pub fn get_area(&self, save: &SaveBuffer, id: &GameObjectId) -> Option<Area> {
        save.0
            .data
            .areas
            .get(id)
            .or(self.map_ecs.areas.get(id))
            .cloned()
    }
    pub fn get_inventory(&self, save: &SaveBuffer, id: &GameObjectId) -> Option<Inventory> {
        save.0
            .data
            .inventorys
            .get(id)
            .or(self.map_ecs.inventorys.get(id))
            .cloned()
    }
    pub fn get_equipment(&self, save: &SaveBuffer, id: &GameObjectId) -> Option<Equipment> {
        save.0
            .data
            .equipments
            .get(id)
            .or(self.map_ecs.equipments.get(id))
            .cloned()
    }
}

fn load_cache(
    mut cache: ResMut<WorldCache>,
    asset_server: Res<AssetServer>,
    tileds: Res<Assets<TiledAsset>>,
) {
    {
        // Tiled map file
        let cache = &mut *cache;
        let mut loaded = vec![];
        for &chunk in cache.loading_chunk.iter() {
            if !cache.chunk_tiled_files.contains_key(&chunk) {
                // info!("Adding tiled map file to load for {chunk:?}");
                let mut hs = HashSet::new();
                // info!(
                //     "{chunk:?} chunk.asset_load_path(): {:?}",
                //     chunk.asset_load_path()
                // );
                chunk.asset_load_path().into_iter().for_each(|path| {
                    if !cache.tiled_handles.contains_key(&path) {
                        // info!("New tiled path: {path:?}");
                        // First time
                        if PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
                            .join("assets")
                            .join(path.clone())
                            .exists()
                        {
                            // insert handle
                            // debug!("asset_server.load: {path:?}");
                            let handle = asset_server.load(&path);
                            cache
                                .tiled_handles
                                .insert(path.clone(), Some(handle.clone()));
                            // add to loading
                            // info!("Tiled handle added to loading: {path:?}");
                            cache.loading_tileds.insert(handle.clone());
                            hs.insert(handle);
                        } else {
                            // No file, just insert None
                            // debug!("Tiled path do not exists: {path:?}");
                            cache.tiled_handles.insert(path.clone(), None);
                        }
                    } else {
                        if let Some(handle) = cache.tiled_handles.get(&path).unwrap() {
                            hs.insert(handle.clone());
                        }
                    }
                });
                // info!("cache.chunk_tiled_files.insert: {chunk:?}, {hs:?}");
                cache.chunk_tiled_files.insert(chunk, hs);
            }
            let handles = cache.chunk_tiled_files.get(&chunk).unwrap();
            match asset_server.get_group_load_state(handles.iter().map(|h| h.id)) {
                bevy::asset::LoadState::Loaded => {
                    // info!("Tiled map file for {chunk:?} are loaded now");
                    loaded.push(chunk);

                    for handle in handles.iter() {
                        // info!("Tiled handle remove from loading");
                        if !cache.loading_tileds.remove(handle) {
                            // info!("Tiled handle is not exists in loading");
                            continue;
                        }
                        cache.loaded_tileds.insert(handle.clone());
                        let tiled_asset = tileds.get(handle).unwrap();
                        let tiled_map = &tiled_asset.map;
                        // info!("Init tiled_asset: {:?},{:?}", tiled_asset.x, tiled_asset.y);
                        for layer in tiled_map.layers() {
                            match layer.layer_type() {
                                tiled::LayerType::ObjectLayer(layer) => {
                                    for object in layer.objects() {
                                        let id = GameObjectId::Tiled {
                                            x: tiled_asset.x,
                                            y: tiled_asset.y,
                                            id: object.id(),
                                        };
                                        let x = object.x
                                            + (tiled_asset.x as i32 * TILE_SIZE * MAP_SIZE) as f32;
                                        let y = -object.y
                                            - (tiled_asset.y as i32 * TILE_SIZE * MAP_SIZE) as f32;
                                        let (x, y) = match &object.shape {
                                            tiled::ObjectShape::Rect { width, height } => {
                                                cache.map_ecs.areas.insert(
                                                    id,
                                                    Area {
                                                        event: AreaEvent::from(
                                                            object.name.as_str(),
                                                        ),
                                                        hx: width / 2.0,
                                                        hy: height / 2.0,
                                                        disable: false,
                                                    },
                                                );
                                                (x + width / 2.0, y - height / 2.0)
                                            }
                                            tiled::ObjectShape::Point(_, _) => (x, y),
                                            _ => todo!(),
                                        };
                                        info!("Updating cache for object in map: {id:?}, obj_type: {}", object.obj_type);
                                        cache.map.update(id, (x, y));

                                        cache
                                            .map_ecs
                                            .objects
                                            .insert(id, object.obj_type.as_str().into());
                                        cache
                                            .map_ecs
                                            .transforms
                                            .insert(id, ((x, y, 0.0).into(), default(), default()));
                                    }
                                }
                                tiled::LayerType::TileLayer(_) => {}
                                tiled::LayerType::ImageLayer(_) => {}
                                tiled::LayerType::GroupLayer(_) => {}
                            }
                        }
                    }
                }
                bevy::asset::LoadState::Failed => {
                    todo!()
                }
                _ => (),
                // bevy::asset::LoadState::NotLoaded => todo!(),
                // bevy::asset::LoadState::Loading => todo!(),
                // bevy::asset::LoadState::Unloaded => todo!(),
            }
        }
        for chunk in loaded {
            cache.loading_chunk.remove(&chunk);
            cache.loaded_chunk.insert(chunk);
        }
    }
}
