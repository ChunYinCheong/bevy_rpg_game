use std::{collections::HashSet, path::PathBuf};

use bevy::{math::ivec3, prelude::*};
use bevy_simple_tilemap::prelude::*;

use crate::plugins::{game_world::MAP_SIZE, spatial_map::TILE_SIZE};

use super::tiled_asset::TiledAsset;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugin(SimpleTileMapPlugin)
            .init_resource::<TileMapRes>()
            .add_event::<LoadMap>()
            .add_system(load_map)
            .add_system(load_pending.after(load_map));
    }
}

#[derive(Debug, Default)]
pub struct TileMapRes {
    pub pending: HashSet<Handle<TiledAsset>>,
    pub loaded: HashSet<Handle<TiledAsset>>,
}

#[derive(Debug)]
pub struct LoadMap(pub String);
pub fn load_map(
    mut events: EventReader<LoadMap>,
    mut res: ResMut<TileMapRes>,
    asset_server: Res<AssetServer>,
) {
    for e in events.iter() {
        // debug!("{e:?}");
        if PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("assets")
            .join(e.0.clone())
            .exists()
        {
            let handle: Handle<TiledAsset> = asset_server.load(&e.0);
            if !res.loaded.contains(&handle) {
                res.pending.insert(handle);
            }
        }
    }
}

pub fn load_pending(
    mut res: ResMut<TileMapRes>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tileds: Res<Assets<TiledAsset>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut loaded = vec![];
    for handle in res.pending.iter() {
        if let Some(asset) = tileds.get(handle) {
            loaded.push(handle.clone());

            let map = &asset.map;
            let mut tilesets = vec![];
            for tileset in map.tilesets() {
                if let Some(image) = &tileset.image {
                    let texture_handle = asset_server.load(image.source.clone());
                    let texture_atlas = TextureAtlas::from_grid(
                        texture_handle,
                        Vec2::new(tileset.tile_width as f32, tileset.tile_height as f32),
                        tileset.columns as usize,
                        (image.height / tileset.tile_height as i32) as usize,
                    );
                    let texture_atlas_handle = texture_atlases.add(texture_atlas);
                    tilesets.push(Some(texture_atlas_handle));
                } else {
                    tilesets.push(None);
                }
            }

            let mut tiles = vec![];

            for layer in map.layers() {
                match layer.layer_type() {
                    tiled::LayerType::TileLayer(layer) => match layer {
                        tiled::TileLayer::Finite(data) => {
                            for y in 0..data.height() {
                                for x in 0..data.width() {
                                    if let Some(layer_tile) = data.get_tile(x as i32, y as i32) {
                                        let id = layer_tile.id();
                                        // let index = layer_tile.tileset_index();
                                        if let Some(_tile) = layer_tile.get_tile() {
                                            tiles.push((
                                                ivec3(x as i32, -(y as i32), 0),
                                                Some(Tile {
                                                    sprite_index: id,
                                                    ..Default::default()
                                                }),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                        tiled::TileLayer::Infinite(_) => {}
                    },
                    tiled::LayerType::ObjectLayer(_) => {}
                    tiled::LayerType::ImageLayer(_) => {}
                    tiled::LayerType::GroupLayer(_) => {}
                }
            }

            let mut tilemap = TileMap::default();
            tilemap.set_tiles(tiles);

            // Set up tilemap
            let x = (TILE_SIZE / 2) + (asset.x as i32) * TILE_SIZE * MAP_SIZE;
            let y = -(TILE_SIZE / 2) - (asset.y as i32) * TILE_SIZE * MAP_SIZE;
            let texture_atlas_handle = tilesets.get(0).unwrap().as_ref().unwrap().clone();
            let tilemap_bundle = TileMapBundle {
                tilemap,
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                ..Default::default()
            };

            // Spawn tilemap
            commands
                .spawn_bundle(tilemap_bundle)
                .insert(Name::new("TileMapBundle"));
        }
    }
    for handle in loaded {
        res.pending.remove(&handle);
        res.loaded.insert(handle);
    }
}
