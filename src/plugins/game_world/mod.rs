use bevy::{prelude::*, time::Stopwatch};
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use strum::{Display, EnumIter};

use self::world_cache::{WorldCache, WorldCachePlugin};

use super::{
    animation::AnimationState,
    area::{Area, PlayerEnterEvent},
    blocker::Blocker,
    chest::Chest,
    item::{Equipment, Inventory},
    player::Hero,
    save::{ClearOnReset, ClearSave, SaveBuffer, WriteSaveFile},
    scene_editor::scene_loader::SceneRes,
    spatial_map::{CHUNK_SIZE, TILE_SIZE},
    tiled_asset::TiledAsset,
    trigger::EventTrigger,
    unit::{Unit, UnitDieEvent},
};

mod world_cache;

pub struct GameWorldPlugin;
impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugin(WorldCachePlugin)
            .register_type::<WorldChunk>()
            .register_type::<GameObjectId>()
            .register_type::<GameObjectType>()
            .add_system(follow_player)
            .add_event::<LoadPosition>()
            .add_system(load_position)
            .add_system(load_chunk)
            .add_event::<LoadObject>()
            .add_system(load_object.after(unload_object))
            .init_resource::<Loaded>()
            // .add_event::<UpdateSpatialMap>()
            // .add_system(update_spatial_map)
            .add_event::<UnloadObject>()
            .add_system(unload_object.before(load_object))
            // .add_system(auto_save)
            .add_event::<ResetEvent>()
            .add_system(reset_world.before(load_object).before(unload_object))
            .init_resource::<GameWorld>();
    }
}

/// Tile in map
pub const MAP_SIZE: i32 = 100;

#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct WorldChunkRoot;
/// (x, y)
///
/// y is following bevy transform: 1 is up, -1 is down.
/// Use -y when working with tiled.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Component, Reflect)]
pub struct WorldChunk {
    pub x: i32,
    pub y: i32,
}
impl WorldChunk {
    /// x, y should come from the Bevy Transform,
    ///
    /// y: 1 is up, -1 is down.
    pub fn new(x: f32, y: f32) -> WorldChunk {
        let mut cx = x as i32 / TILE_SIZE / CHUNK_SIZE;
        if x < 0.0 {
            cx -= 1;
        }
        let mut cy = y as i32 / TILE_SIZE / CHUNK_SIZE;
        if y < 0.0 {
            cy -= 1;
        }
        WorldChunk { x: cx, y: cy }
    }

    fn x_range(&self) -> (i32, i32) {
        (self.x * CHUNK_SIZE, self.x * CHUNK_SIZE + CHUNK_SIZE)
    }
    fn y_range(&self) -> (i32, i32) {
        (self.y * CHUNK_SIZE, self.y * CHUNK_SIZE + CHUNK_SIZE)
    }

    fn tiles(&self) -> Vec<WorldTile> {
        let (x1, x2) = self.x_range();
        let (y1, y2) = self.y_range();

        (x1..x2)
            .flat_map(|x| (y1..y2).map(move |y| (x, y)))
            .map(|(x, y)| WorldTile::new(x, y))
            .collect()
    }

    fn _tiled_file_names(&self) -> HashSet<String> {
        HashSet::from_iter(self.tiles().iter().map(|wt| wt.tiled_file_name()))
    }

    pub fn asset_load_path(&self) -> HashSet<String> {
        HashSet::from_iter(self.tiles().iter().map(|wt| wt.asset_load_path()))
    }
}

/// y is following bevy transform: 1 is up, -1 is down.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Component, Reflect)]
pub struct WorldTile {
    pub x: i32,
    pub y: i32,
}
impl WorldTile {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn transform(&self) -> Transform {
        Transform::from_xyz(
            ((TILE_SIZE / 2) + (self.x * TILE_SIZE)) as f32,
            ((TILE_SIZE / 2) + (self.y * TILE_SIZE)) as f32,
            0.0,
        )
    }

    /// Index in file, range: `0..MAP_SIZE`
    fn tiled_index(&self) -> (i32, i32) {
        (
            self.x.rem_euclid(MAP_SIZE),
            (-self.y - 1).rem_euclid(MAP_SIZE),
        )
    }

    fn tiled_file_index(&self) -> (i32, i32) {
        let (xi, yi) = (self.x, -self.y - 1);
        let mut x = xi / MAP_SIZE;
        if xi < 0 {
            x -= 1;
        }
        let mut y = yi / MAP_SIZE;
        if yi < 0 {
            y -= 1;
        }
        (x, y)
    }
    fn tiled_file_name(&self) -> String {
        let (x, y) = self.tiled_file_index();
        format!("map_x{x}_y{y}.tmx")
    }
    fn asset_load_path(&self) -> String {
        let file_name = self.tiled_file_name();
        format!("maps/world/{file_name}")
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Component, Reflect, Default,
)]
pub struct GameObjectId(pub String);

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Component, Reflect, EnumIter, Display,
)]
pub enum GameObjectType {
    None,
    Unknown,
    Player,
    Wolf,
    Spider,
    Fox,
    Shop,
    ResetPoint,
    Blocker,
    Area,
    Chest,
    Rock,
}

impl Default for GameObjectType {
    fn default() -> Self {
        GameObjectType::None
    }
}

impl From<&str> for GameObjectType {
    fn from(s: &str) -> Self {
        match s {
            "Wolf" => GameObjectType::Wolf,
            "Spider" => GameObjectType::Spider,
            "Fox" => GameObjectType::Fox,
            "Player" => GameObjectType::Player,
            "Shop" => GameObjectType::Shop,
            "ResetPoint" => GameObjectType::ResetPoint,
            "Blocker" => GameObjectType::Blocker,
            "Area" => GameObjectType::Area,
            "Chest" => GameObjectType::Chest,
            "" => {
                warn!("Empty obj_type!");
                GameObjectType::None
            }
            _ => {
                error!("Unknown obj_type: {}", s);
                GameObjectType::Unknown
            }
        }
    }
}

#[derive(Debug, Default, Resource)]
pub struct GameWorld {
    pub current_chunk: Option<WorldChunk>,
    pub loading_chunks: HashSet<WorldChunk>,
    pub loaded_chunks: HashSet<WorldChunk>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Reflect)]
#[reflect_value()]
pub struct Ecs {
    pub objects: HashMap<GameObjectId, GameObjectType>,
    pub units: HashMap<GameObjectId, Unit>,
    pub transforms: HashMap<GameObjectId, (Vec3, Quat, Vec3)>,
    pub resets: HashMap<GameObjectId, ClearOnReset>,
    pub blockers: HashMap<GameObjectId, Blocker>,
    pub areas: HashMap<GameObjectId, Area>,
    pub inventorys: HashMap<GameObjectId, Inventory>,
    pub equipments: HashMap<GameObjectId, Equipment>,
    pub chests: HashMap<GameObjectId, Chest>,
    pub enter_triggers: HashMap<GameObjectId, EventTrigger<PlayerEnterEvent>>,
    pub die_triggers: HashMap<GameObjectId, EventTrigger<UnitDieEvent>>,
    pub collision_groupss: HashMap<GameObjectId, (u32, u32)>,
    pub animation_states: HashMap<GameObjectId, AnimationState>,
}
#[derive(Debug)]
pub struct LoadObject(pub GameObjectId);

#[derive(Debug, Default, Resource)]
pub struct Loaded {
    pub objects: HashSet<GameObjectId>,
}

fn follow_player(player_q: Query<&Transform, With<Hero>>, mut events: EventWriter<LoadPosition>) {
    if let Ok(position) = player_q.get_single() {
        let t = position.translation;
        let x = t.x;
        let y = t.y;
        events.send(LoadPosition(x, y));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LoadPosition(pub f32, pub f32);
fn load_position(
    mut events: EventReader<LoadPosition>,
    mut commands: Commands,
    mut game_world: ResMut<GameWorld>,
    mut cache: ResMut<WorldCache>,
    chunk_q: Query<(Entity, &WorldChunk)>,
    transform_query: Query<(Entity, &Transform), With<GameObjectId>>,
    mut unload_events: EventWriter<UnloadObject>,
    // mut load_map_events: EventWriter<LoadMap>,
) {
    for ev in events.iter() {
        // info!("{ev:?}");
        let LoadPosition(x, y) = *ev;

        // Chunk index
        let chunk = WorldChunk::new(x, y);
        if game_world.current_chunk != Some(chunk) {
            info!("LoadPosition, {ev:?}, chunk: {chunk:?}");
            game_world.current_chunk = Some(chunk);
        }

        // define unload range
        let x1 = chunk.x - 2;
        let x2 = chunk.x + 2;
        let y1 = chunk.y - 2;
        let y2 = chunk.y + 2;
        // remove out of range chunk from loading
        game_world
            .loading_chunks
            .retain(|c| x1 <= c.x && c.x <= x2 && y1 <= c.y && c.y <= y2);
        // remove loaded chunk
        game_world
            .loaded_chunks
            .retain(|c| x1 <= c.x && c.x <= x2 && y1 <= c.y && c.y <= y2);
        // Unload chunk
        for (entity, _chunk) in chunk_q
            .iter()
            .filter(|(_, c)| !(x1 <= c.x && c.x <= x2 && y1 <= c.y && c.y <= y2))
        {
            // info!("Unload chunk: {:?}", chunk);
            commands.entity(entity).despawn_recursive();
        }

        // unload object out of range
        let px1 = (x1 * TILE_SIZE * CHUNK_SIZE) as f32;
        let px2 = ((x2 + 1) * TILE_SIZE * CHUNK_SIZE) as f32;
        let py1 = (y1 * TILE_SIZE * CHUNK_SIZE) as f32;
        let py2 = ((y2 + 1) * TILE_SIZE * CHUNK_SIZE) as f32;
        let events = transform_query
            .iter()
            .filter(|(_, t)| {
                let x = t.translation.x;
                let y = t.translation.y;
                x < px1 || x > px2 || y < py1 || y > py2
            })
            .map(|(e, _)| UnloadObject(e))
            .collect::<Vec<_>>();
        if !events.is_empty() {
            info!("Unload: {:?}", events);
            info!("x: {x}, y: {y}, py1: {py1}, py2: {py2}, y1: {y1}, y2: {y2}");
            unload_events.send_batch(events.into_iter());
        }

        // Load range
        // let x1 = chunk.x - 1;
        // let x2 = chunk.x + 1;
        // let y1 = chunk.y - 1;
        // let y2 = chunk.y + 1;
        // Load new chunk
        for x in x1..=x2 {
            for y in y1..=y2 {
                let chunk = WorldChunk { x, y };
                if !game_world.loaded_chunks.contains(&chunk) {
                    // info!("Load new chunk: {chunk:?}");
                    game_world.loading_chunks.insert(chunk);
                    // Cache
                    cache.load(chunk);
                    // Draw tile map
                    todo!();
                    // load_map_events.send_batch(chunk.asset_load_path().into_iter().map(LoadMap));
                }
            }
        }
    }
}

fn load_chunk(
    mut commands: Commands,
    mut game_world: ResMut<GameWorld>,
    cache: Res<WorldCache>,
    tileds: Res<Assets<TiledAsset>>,
    mut events: EventWriter<LoadObject>,
    chunks_q: Query<Entity, With<WorldChunkRoot>>,
    save: Res<SaveBuffer>,
    editor: Res<SceneRes>,
) {
    let mut loaded = vec![];
    let chunk_root = match chunks_q.get_single() {
        Ok(e) => e,
        Err(err) => match err {
            bevy::ecs::query::QuerySingleError::NoEntities(_) => commands
                .spawn(SpatialBundle::default())
                .insert(WorldChunkRoot)
                .insert(Name::new("WorldChunkRoot"))
                .id(),
            bevy::ecs::query::QuerySingleError::MultipleEntities(_) => todo!(),
        },
    };
    // check res and get tiled file asset
    for &chunk in game_world.loading_chunks.iter() {
        if !cache.ready(&chunk) {
            continue;
        }
        // info!("Ready to load: {chunk:?}");
        loaded.push(chunk);

        let chunk_entity = commands
            .spawn(SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.1),
                ..Default::default()
            })
            .insert(chunk)
            .insert(Name::new(format!("{chunk:?}")))
            .id();
        commands.entity(chunk_root).add_child(chunk_entity);
        // Tile Collider
        trace!("Start creating tile collider for chunk: {chunk:?}");
        for world_tile in chunk.tiles() {
            // info!("Create collider for tile: {world_tile:?}");
            if let Some(handle) = cache
                .tiled_handles
                .get(&world_tile.asset_load_path())
                .expect("file should be loaded")
            {
                // info!("asset_load_path: {:?}", world_tile.asset_load_path());
                // info!("handle: {handle:?}");
                let tiled = tileds.get(handle).unwrap();
                let (x, y) = world_tile.tiled_index();
                for layer in tiled.map.layers() {
                    match layer.layer_type() {
                        tiled::LayerType::TileLayer(layer) => match layer {
                            tiled::TileLayer::Finite(data) => {
                                if let Some(layer_tile) = data.get_tile(x, y) {
                                    if let Some(tile) = layer_tile.get_tile() {
                                        if let Some(_collision) = &tile.collision {
                                            commands.entity(chunk_entity).with_children(|child| {
                                                child
                                                    .spawn(SpatialBundle {
                                                        transform: world_tile.transform(),
                                                        ..Default::default()
                                                    })
                                                    .insert(Name::new("Tile Collider"))
                                                    .insert(world_tile)
                                                    .insert(RigidBody::Fixed)
                                                    .insert(Collider::cuboid(
                                                        TILE_SIZE as f32 / (2.0),
                                                        TILE_SIZE as f32 / (2.0),
                                                    ));
                                            });
                                        }
                                    }
                                }
                            }
                            tiled::TileLayer::Infinite(_) => todo!(),
                        },
                        tiled::LayerType::ObjectLayer(_) => {}
                        tiled::LayerType::ImageLayer(_) => todo!(),
                        tiled::LayerType::GroupLayer(_) => todo!(),
                    }
                }
            }
        }
        // Game Object
        let v = cache.get_objects(&save, &editor, &(chunk.x, chunk.y));
        if !v.is_empty() {
            events.send_batch(v.iter().map(|id| LoadObject(id.clone())));
        }
    }

    for chunk in loaded {
        game_world.loading_chunks.remove(&chunk);
        game_world.loaded_chunks.insert(chunk);
    }
}

fn load_object(
    mut events: EventReader<LoadObject>,
    mut commands: Commands,
    cache: Res<WorldCache>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut loaded: ResMut<Loaded>,
    save: Res<SaveBuffer>,
    editor: Res<SceneRes>,
) {
    for ev in events.iter() {
        // info!("LoadObject: {ev:?}");
        let id = &ev.0;

        if loaded.objects.contains(id) {
            info!("LoadObject: {ev:?}, object already loaded");
            continue;
        } else {
            info!("LoadObject: {ev:?}, start loading object");
            loaded.objects.insert(id.clone());
        }
        let entity = match cache.get_object_type(&save, &editor, id) {
            Some(o) => {
                let pos = cache.get_transform(&save, &editor, id).unwrap();
                let pos = (pos.0.x, pos.0.y).into();
                info!("LoadObject: {ev:?}, object load pos: {pos:?}");
                match o {
                    GameObjectType::None => continue,
                    GameObjectType::Unknown => {
                        error!("Unknown obj_type: {:?}", o);
                        continue;
                    }
                    GameObjectType::Player => crate::plugins::player::spawn_hero(
                        &mut commands,
                        pos,
                        &asset_server,
                        &mut texture_atlases,
                    ),
                    GameObjectType::Wolf => crate::plugins::wolf::spawn_wolf(
                        &mut commands,
                        pos,
                        &asset_server,
                        &mut texture_atlases,
                    ),
                    GameObjectType::Spider => crate::plugins::spider::spawn_spider(
                        &mut commands,
                        pos,
                        &asset_server,
                        &mut texture_atlases,
                    ),
                    GameObjectType::Fox => crate::plugins::fox::spawn_fox(
                        &mut commands,
                        pos,
                        &asset_server,
                        &mut texture_atlases,
                    ),
                    GameObjectType::Shop => crate::plugins::shop::spawn_shop(
                        &mut commands,
                        pos,
                        &asset_server,
                        &mut texture_atlases,
                    ),
                    GameObjectType::ResetPoint => crate::plugins::reset_point::spawn_reset_point(
                        &mut commands,
                        pos,
                        &asset_server,
                        &mut texture_atlases,
                    ),
                    GameObjectType::Blocker => crate::plugins::blocker::spawn_blocker(
                        &mut commands,
                        pos,
                        &asset_server,
                        &mut texture_atlases,
                    ),
                    GameObjectType::Area => crate::plugins::area::spawn_area(
                        &mut commands,
                        pos,
                        &asset_server,
                        &mut texture_atlases,
                    ),
                    GameObjectType::Chest => crate::plugins::chest::spawn_chest(
                        &mut commands,
                        pos,
                        &asset_server,
                        &mut texture_atlases,
                    ),
                    GameObjectType::Rock => crate::plugins::rock::spawn_rock(
                        &mut commands,
                        pos,
                        &asset_server,
                        &mut texture_atlases,
                    ),
                }
            }
            None => {
                error!("Missing obj_type for object: LoadObject: {ev:?}");
                continue;
            }
        };
        info!("LoadObject: {ev:?}, object loaded, {entity:?}, {id:?}");

        if let Some(u) = cache.get_unit(&save, &editor, id) {
            commands.entity(entity).insert(u.clone());
        }
        if let Some(u) = cache.get_blocker(&save, &editor, id) {
            commands.entity(entity).insert(u.clone());
        }
        if let Some(u) = cache.get_area(&save, &editor, id) {
            commands.entity(entity).insert(u.clone());
        }
        if let Some(u) = cache.get_inventory(&save, &editor, id) {
            commands.entity(entity).insert(u.clone());
        }
        if let Some(u) = cache.get_equipment(&save, &editor, id) {
            commands.entity(entity).insert(u.clone());
        }
        if let Some(u) = cache.get_chest(&save, &editor, id) {
            commands.entity(entity).insert(u.clone());
        }
        if let Some(u) = cache.get_enter_trigger(&save, &editor, id) {
            commands.entity(entity).insert(u.clone());
        }
        if let Some(u) = cache.get_die_trigger(&save, &editor, id) {
            commands.entity(entity).insert(u.clone());
        }
        if let Some((memberships, filters)) = cache.get_collision_groups(&save, &editor, id) {
            commands.entity(entity).insert(CollisionGroups::new(
                Group::from_bits_truncate(memberships),
                Group::from_bits_truncate(filters),
            ));
        }
        if let Some(u) = cache.get_animation_state(&save, &editor, id) {
            commands.entity(entity).insert(u.clone());
        }
        commands.entity(entity).insert(id.clone());
    }
}

#[derive(Debug)]
pub struct UnloadObject(pub Entity);
fn unload_object(
    mut events: EventReader<UnloadObject>,
    mut commands: Commands,
    mut loaded: ResMut<Loaded>,
    id_query: Query<&GameObjectId>,
    mut write_events: EventWriter<WriteSaveFile>,
) {
    let mut empty = true;
    for ev in events.iter() {
        empty = false;
        // info!("UnloadObject, ev: {ev:?}");
        let entity = ev.0;

        // Remove from loaded
        if let Ok(id) = id_query.get(entity) {
            let exist = loaded.objects.remove(id);
            info!("UnloadObject, ev: {ev:?}, id: {id:?}, loaded: {exist}");
        } else {
            info!("UnloadObject, ev: {ev:?}, object does not exist");
            continue;
        }

        // Despawn
        commands.entity(ev.0).despawn_recursive();
    }

    if !empty {
        write_events.send(WriteSaveFile);
    }
}

pub fn auto_save(
    mut stopwatch: Local<Stopwatch>,
    time: Res<Time>,
    mut write_events: EventWriter<WriteSaveFile>,
) {
    stopwatch.tick(time.delta());
    if stopwatch.elapsed_secs() >= 1.0 {
        // info!("Auto save...");
        stopwatch.reset();
        // Write to file
        write_events.send(WriteSaveFile);
    }
}

pub struct ResetEvent;
fn reset_world(
    mut events: EventReader<ResetEvent>,
    mut commands: Commands,
    chunk_q: Query<(Entity, &WorldChunk)>,
    transform_query: Query<Entity, With<GameObjectId>>,
    mut unload_events: EventWriter<UnloadObject>,
    mut clear_events: EventWriter<ClearSave>,
    mut load_events: EventWriter<LoadObject>,
    mut game_world: ResMut<GameWorld>,
) {
    for _ in events.iter() {
        info!("ResetEvent");

        // Unload all chunk
        game_world.loading_chunks.clear();
        game_world.loaded_chunks.clear();
        for (entity, _) in chunk_q.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // Unload all object
        let events = transform_query.iter().map(UnloadObject);
        unload_events.send_batch(events);

        // Clear save for reset object
        clear_events.send(ClearSave);

        // Reload player
        load_events.send(LoadObject(GameObjectId("player".into())));
    }
}
