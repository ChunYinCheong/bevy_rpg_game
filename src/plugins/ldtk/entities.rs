use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::plugins::{
    area::{Area, PlayerEnterEvent},
    blocker::Blocker,
    chest::Chest,
    game_world::GameObjectId,
    item::ItemId,
    rogue::{
        rogue::{BackTo, StartTeleport, StartTeleportTarget},
        shop::{ShopSlot, SlotAction, SlotItem},
    },
    save::SaveBuffer,
    trigger::{EventTrigger, TriggerAction},
};

pub fn process_my_entity(
    mut commands: Commands,
    entity_query: Query<(Entity, &Transform, &EntityInstance), Added<EntityInstance>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    save: Res<SaveBuffer>,
) {
    for (entity, transform, entity_instance) in entity_query.iter() {
        debug!("process_my_entity: {}", entity_instance.identifier);
        match entity_instance.identifier.as_str() {
            "Player" => {
                let id = crate::plugins::player::spawn_hero(
                    &mut commands,
                    Vec2::ZERO,
                    &asset_server,
                    &mut texture_atlases,
                );
                commands
                    .entity(entity)
                    .insert(Worldly {
                        entity_iid: entity_instance.iid.clone(),
                    })
                    .add_child(id);
            }
            "Wolf" => {
                let id = crate::plugins::wolf::spawn_wolf(
                    &mut commands,
                    Vec2::ZERO,
                    &asset_server,
                    &mut texture_atlases,
                );
                commands
                    .entity(id)
                    .insert(GameObjectId(entity_instance.iid.clone()));
                commands.entity(entity).add_child(id);
            }
            "Spider" => {
                let id = crate::plugins::spider::spawn_spider(
                    &mut commands,
                    Vec2::ZERO,
                    &asset_server,
                    &mut texture_atlases,
                );

                commands
                    .entity(id)
                    .insert(GameObjectId(entity_instance.iid.clone()));
                commands.entity(entity).add_child(id);
            }
            "Fox" => {
                let id = crate::plugins::fox::spawn_fox(
                    &mut commands,
                    Vec2::ZERO,
                    &asset_server,
                    &mut texture_atlases,
                );

                commands
                    .entity(id)
                    .insert(GameObjectId(entity_instance.iid.clone()));
                commands.entity(entity).add_child(id);
            }
            "Shop" => {
                let id = crate::plugins::shop::spawn_shop(
                    &mut commands,
                    Vec2::ZERO,
                    &asset_server,
                    &mut texture_atlases,
                );

                commands
                    .entity(id)
                    .insert(GameObjectId(entity_instance.iid.clone()));
                commands.entity(entity).add_child(id);
            }
            "ResetPoint" => {
                let id = crate::plugins::reset_point::spawn_reset_point(
                    &mut commands,
                    Vec2::ZERO,
                    &asset_server,
                    &mut texture_atlases,
                );

                commands
                    .entity(id)
                    .insert(GameObjectId(entity_instance.iid.clone()));
                commands.entity(entity).add_child(id);
            }
            "Blocker" => {
                let id = crate::plugins::blocker::spawn_blocker(
                    &mut commands,
                    Vec2::ZERO,
                    &asset_server,
                    &mut texture_atlases,
                );

                let blocking = entity_instance
                    .field_instances
                    .iter()
                    .find(|f| f.identifier == *"blocking")
                    .unwrap();
                let blocking = if let FieldValue::Bool(blocking) = &blocking.value {
                    *blocking
                } else {
                    error!("blocking is not bool field!");
                    false
                };
                let blocking = save
                    .0
                    .data
                    .blockers
                    .get(&GameObjectId(entity_instance.iid.clone()))
                    .map(|b| b.blocking)
                    .unwrap_or(blocking);
                commands.entity(id).insert(Blocker {
                    hx: (entity_instance.width / 2) as f32,
                    hy: (entity_instance.height / 2) as f32,
                    blocking,
                });

                commands
                    .entity(id)
                    .insert(GameObjectId(entity_instance.iid.clone()));
                commands.entity(entity).add_child(id);
            }
            "Area" => {
                let id = crate::plugins::area::spawn_area(
                    &mut commands,
                    Vec2::ZERO,
                    &asset_server,
                    &mut texture_atlases,
                );
                commands.entity(id).insert(Area {
                    hx: (entity_instance.width / 2) as f32,
                    hy: (entity_instance.height / 2) as f32,
                    disable: false,
                });

                let actions = entity_instance
                    .field_instances
                    .iter()
                    .find(|f| f.identifier == *"actions")
                    .unwrap();
                let actions_targets = entity_instance
                    .field_instances
                    .iter()
                    .find(|f| f.identifier == *"actions_targets")
                    .unwrap();

                if let FieldValue::Enums(actions) = &actions.value {
                    if let FieldValue::EntityRefs(targets) = &actions_targets.value {
                        let mut trigger_actions = vec![];
                        for (i, action) in actions.iter().enumerate() {
                            let action = action.as_ref().unwrap();
                            let entity_iid =
                                targets.get(i).unwrap().as_ref().unwrap().entity_iid.clone();
                            trigger_actions.push(TriggerAction::new(action, entity_iid));
                        }
                        debug!("trigger_actions: {:?}", trigger_actions);
                        commands
                            .entity(id)
                            .insert(EventTrigger::<PlayerEnterEvent> {
                                actions: trigger_actions,
                                ..default()
                            });
                    }
                }

                commands
                    .entity(id)
                    .insert(GameObjectId(entity_instance.iid.clone()));
                commands.entity(entity).add_child(id);
            }
            "Chest" => {
                let id = crate::plugins::chest::spawn_chest(
                    &mut commands,
                    Vec2::ZERO,
                    &asset_server,
                    &mut texture_atlases,
                );
                let item = entity_instance
                    .field_instances
                    .iter()
                    .find(|f| f.identifier == *"item_id")
                    .unwrap();
                if let FieldValue::Enum(Some(s)) = &item.value {
                    //
                    let item_id = ItemId::from(s.as_str());
                    let opened = false;
                    commands.entity(id).insert(Chest { item_id, opened });
                }

                commands
                    .entity(id)
                    .insert(GameObjectId(entity_instance.iid.clone()));
                commands.entity(entity).add_child(id);
            }
            "Rock" => {
                let id = crate::plugins::rock::spawn_rock(
                    &mut commands,
                    Vec2::ZERO,
                    &asset_server,
                    &mut texture_atlases,
                );

                commands
                    .entity(id)
                    .insert(GameObjectId(entity_instance.iid.clone()));
                commands.entity(entity).add_child(id);
            }
            "StartTeleport" => {
                let texture_handle = asset_server.load("images/player/spritesheet.png");
                let texture_atlas = TextureAtlas::from_grid(
                    texture_handle,
                    Vec2::new(64.0, 64.0),
                    10,
                    1,
                    None,
                    None,
                );
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                commands
                    .entity(entity)
                    .insert(Name::new(format!("StartTeleport ({entity:?})")))
                    .insert(StartTeleport)
                    .insert(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        transform: *transform,
                        ..Default::default()
                    })
                    .insert(Area {
                        hx: (entity_instance.width / 2) as f32,
                        hy: (entity_instance.height / 2) as f32,
                        disable: false,
                    })
                    // Rapier
                    // .insert(RigidBody::Fixed)
                    // .insert(Sensor)
                    // .insert(ActiveEvents::COLLISION_EVENTS)
                    // .insert(Collider::cuboid(32.0, 32.0))
                    ;
            }
            "StartTeleportTarget" => {
                commands
                    .entity(entity)
                    .insert(Name::new(format!("StartTeleportTarget ({entity:?})")))
                    .insert(StartTeleportTarget);
            }
            "BackTo" => {
                commands
                    .entity(entity)
                    .insert(Name::new(format!("BackTo ({entity:?})")))
                    .insert(BackTo);
            }
            "ShopItem" => {
                commands
                    .entity(entity)
                    .insert(Name::new(format!("Shop Slot Item ({entity:?})")))
                    .insert(ShopSlot::default())
                    .insert(SlotItem::default())
                    .insert(SpriteBundle {
                        transform: *transform,
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(64.0, 64.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    // Rapier
                    .insert(RigidBody::Fixed)
                    // collider is scaled by ldtk plugins
                    .insert(Collider::cuboid(32.0, 32.0))
                    .insert(Sensor)
                    .insert(ActiveEvents::COLLISION_EVENTS);
            }
            "ShopAction" => {
                commands
                    .entity(entity)
                    .insert(Name::new(format!("Shop Slot Action ({entity:?})")))
                    .insert(ShopSlot::default())
                    .insert(SlotAction::default())
                    .insert(SpriteBundle {
                        transform: *transform,
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(64.0, 64.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    // Rapier
                    .insert(RigidBody::Fixed)
                    // collider is scaled by ldtk plugins
                    .insert(Collider::cuboid(32.0, 32.0))
                    .insert(Sensor)
                    .insert(ActiveEvents::COLLISION_EVENTS);
            }
            _ => {
                warn!("Unknown entity: {}", entity_instance.identifier)
            }
        }
    }
}
