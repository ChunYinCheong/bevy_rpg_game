use bevy::prelude::*;
use std::{fs::File, path::PathBuf};

use crate::plugins::{
    game_world::{Ecs, GameObjectId},
    spatial_map::SpatialMap,
};

pub struct SceneLoaderPlugin;
impl Plugin for SceneLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .init_resource::<SceneRes>()
            .add_startup_system(load_scene);
    }
}

#[derive(Debug, Default, Resource, Reflect)]
pub struct SceneRes {
    pub ecs: Ecs,
    pub map: SpatialMap,
}

fn load_scene(mut commands: Commands) {
    let mut res = SceneRes::default();
    let path =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("assets/editor/objects/");
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let object_id = GameObjectId(entry.file_name().into_string().unwrap());
            for entry in path.read_dir().unwrap() {
                let entry = entry.unwrap();
                let compoment_type = entry.file_name();

                let path = entry.path();
                let f = File::open(&path).expect("Failed opening file");

                match compoment_type.to_str().unwrap() {
                    "Transform.ron" => {
                        match ron::de::from_reader(f) {
                            Ok(x) => {
                                res.ecs.transforms.insert(object_id.clone(), x);
                                res.map.update(object_id.clone(), x.0.truncate().into())
                            }
                            Err(e) => {
                                println!("Failed to load Component({compoment_type:?}): {e}");
                                std::process::exit(1);
                            }
                        };
                    }
                    "GameObjectType.ron" => {
                        match ron::de::from_reader(f) {
                            Ok(x) => {
                                res.ecs.objects.insert(object_id.clone(), x);
                            }
                            Err(e) => {
                                println!("Failed to load Component({compoment_type:?}): {e}");
                                std::process::exit(1);
                            }
                        };
                    }
                    "Unit.ron" => {
                        match ron::de::from_reader(f) {
                            Ok(x) => {
                                res.ecs.units.insert(object_id.clone(), x);
                            }
                            Err(e) => {
                                println!("Failed to load Component({compoment_type:?}): {e}");
                                std::process::exit(1);
                            }
                        };
                    }
                    "ClearOnReset.ron" => {
                        match ron::de::from_reader(f) {
                            Ok(x) => {
                                res.ecs.resets.insert(object_id.clone(), x);
                            }
                            Err(e) => {
                                println!("Failed to load Component({compoment_type:?}): {e}");
                                std::process::exit(1);
                            }
                        };
                    }
                    "Blocker.ron" => {
                        match ron::de::from_reader(f) {
                            Ok(x) => {
                                res.ecs.blockers.insert(object_id.clone(), x);
                            }
                            Err(e) => {
                                println!("Failed to load Component({compoment_type:?}): {e}");
                                std::process::exit(1);
                            }
                        };
                    }
                    "Area.ron" => {
                        match ron::de::from_reader(f) {
                            Ok(x) => {
                                res.ecs.areas.insert(object_id.clone(), x);
                            }
                            Err(e) => {
                                println!("Failed to load Component({compoment_type:?}): {e}");
                                std::process::exit(1);
                            }
                        };
                    }
                    "Inventory.ron" => {
                        match ron::de::from_reader(f) {
                            Ok(x) => {
                                res.ecs.inventorys.insert(object_id.clone(), x);
                            }
                            Err(e) => {
                                println!("Failed to load Component({compoment_type:?}): {e}");
                                std::process::exit(1);
                            }
                        };
                    }
                    "Equipment.ron" => {
                        match ron::de::from_reader(f) {
                            Ok(x) => {
                                res.ecs.equipments.insert(object_id.clone(), x);
                            }
                            Err(e) => {
                                println!("Failed to load Component({compoment_type:?}): {e}");
                                std::process::exit(1);
                            }
                        };
                    }
                    "Chest.ron" => {
                        match ron::de::from_reader(f) {
                            Ok(x) => {
                                res.ecs.chests.insert(object_id.clone(), x);
                            }
                            Err(e) => {
                                println!("Failed to load Component({compoment_type:?}): {e}");
                                std::process::exit(1);
                            }
                        };
                    }
                    "EventTrigger_PlayerEnterEvent.ron" => {
                        match ron::de::from_reader(f) {
                            Ok(x) => {
                                res.ecs.enter_triggers.insert(object_id.clone(), x);
                            }
                            Err(e) => {
                                println!("Failed to load Component({compoment_type:?}): {e}");
                                std::process::exit(1);
                            }
                        };
                    }
                    "EventTrigger_UnitDieEvent.ron" => {
                        match ron::de::from_reader(f) {
                            Ok(x) => {
                                res.ecs.die_triggers.insert(object_id.clone(), x);
                            }
                            Err(e) => {
                                println!("Failed to load Component({compoment_type:?}): {e}");
                                std::process::exit(1);
                            }
                        };
                    }
                    "CollisionGroups.ron" => {
                        match ron::de::from_reader(f) {
                            Ok(x) => {
                                res.ecs.collision_groupss.insert(object_id.clone(), x);
                            }
                            Err(e) => {
                                println!("Failed to load Component({compoment_type:?}): {e}");
                                std::process::exit(1);
                            }
                        };
                    }
                    "AnimationState.ron" => {
                        match ron::de::from_reader(f) {
                            Ok(x) => {
                                res.ecs.animation_states.insert(object_id.clone(), x);
                            }
                            Err(e) => {
                                println!("Failed to load Component({compoment_type:?}): {e}");
                                std::process::exit(1);
                            }
                        };
                    }
                    _ => {
                        error!("Unknown type: {compoment_type:?}");
                    }
                }
            }
        }
    }

    commands.insert_resource(res);
}
