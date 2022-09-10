use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::game_world::GameObjectId;

/// Tile height and width
pub const TILE_SIZE: i32 = 64;
/// Number of tile in world chunk
pub const CHUNK_SIZE: i32 = 8;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SpatialMap {
    map: HashMap<(i32, i32), Vec<GameObjectId>>,
    r_map: HashMap<GameObjectId, (i32, i32)>,
}

impl SpatialMap {
    pub fn get_objects(&self, chunk: &(i32, i32)) -> Option<&Vec<GameObjectId>> {
        self.map.get(chunk)
    }
    pub fn get_chunk(&self, id: &GameObjectId) -> Option<&(i32, i32)> {
        self.r_map.get(id)
    }

    pub fn update(&mut self, id: GameObjectId, pos: (f32, f32)) {
        if let Some(map_key) = self.r_map.remove(&id) {
            if let Some(v) = self.map.get_mut(&map_key) {
                v.retain(|i| *i != id);
            }
        }

        let x = pos.0 as i32 / TILE_SIZE / CHUNK_SIZE;
        let y = pos.1 as i32 / TILE_SIZE / CHUNK_SIZE;
        let key = (x, y);
        let vec = self.map.entry(key).or_default();
        vec.push(id);
        self.r_map.insert(id, key);
    }

    pub fn remove(&mut self, id: &GameObjectId) {
        if let Some(map_key) = self.r_map.remove(&id) {
            if let Some(v) = self.map.get_mut(&map_key) {
                v.retain(|i| i != id);
            }
        }
    }
}
