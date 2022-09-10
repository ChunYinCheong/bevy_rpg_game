use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

use crate::{INTERACT_GROUP, RAPIER_SCALE};

use super::{
    animation::{AnimationEntity, AnimationIndex, AnimationSheet, AnimationState, AnimationTimer},
    interaction::Interaction,
    item::{Inventory, ItemId},
    player::Player,
    unit_action::UnitAnimation,
};

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_inspectable::<Shop>()
            .add_system(open_shop)
            .add_system(shop_ui)
            .add_system(buy_item)
            .add_event::<OpenShopEvent>()
            .add_event::<BuyEvent>()
            .init_resource::<ShopRes>();
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Shop {
    pub items: Vec<ShopItem>,
}

#[derive(Debug, Default, Clone, Inspectable)]
pub struct ShopItem {
    pub item_id: ItemId,
    pub price: i32,
}

#[derive(Debug, Default)]
pub struct ShopRes {
    pub show: bool,
    pub shop_items: Vec<ShopItem>,
}

#[derive(Debug, Default)]
pub struct OpenShopEvent {
    pub shop_items: Vec<ShopItem>,
}

#[derive(Debug)]
pub struct BuyEvent {
    pub buyer: Entity,
    pub item_id: ItemId,
    pub price: i32,
}

pub fn spawn_shop(
    commands: &mut Commands,
    position: Vec2,
    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let animation_entity = {
        let texture_handle = asset_server.load("images/player/spritesheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 10, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    rotation: Default::default(),
                    // scale: Vec3::new(SCALE, SCALE, SCALE),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(AnimationTimer(Timer::from_seconds(0.5, true)))
            .insert(AnimationSheet {
                animations: HashMap::from([
                    (UnitAnimation::Idle.to_string(), (0, 1)),
                    (UnitAnimation::Walk.to_string(), (1, 2)),
                    (UnitAnimation::Run.to_string(), (3, 2)),
                    (UnitAnimation::Attack.to_string(), (5, 1)),
                    (UnitAnimation::Stab.to_string(), (6, 1)),
                    (UnitAnimation::BurstFire.to_string(), (7, 1)),
                    (UnitAnimation::Hook.to_string(), (8, 1)),
                ]),
            })
            .insert(AnimationState {
                animation: UnitAnimation::Idle.to_string(),
            })
            .insert(AnimationIndex::default())
            .id()
    };
    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        })
        .insert(Name::from("Shop"))
        .insert(RigidBody::Fixed)
        .insert(Collider::ball(0.5 * RAPIER_SCALE))
        .insert(CollisionGroups::new(INTERACT_GROUP, u32::MAX))
        .insert(Shop {
            items: vec![
                ShopItem {
                    item_id: ItemId::HpPotion,
                    price: 10,
                },
                ShopItem {
                    item_id: ItemId::MpPotion,
                    price: 20,
                },
                ShopItem {
                    item_id: ItemId::HpPotion,
                    price: 30,
                },
                ShopItem {
                    item_id: ItemId::HpPotion,
                    price: 40,
                },
            ],
        })
        .insert(Interaction::Shop)
        .add_child(animation_entity)
        .insert(AnimationEntity(animation_entity))
        .id()
}

fn shop_ui(
    mut egui_context: ResMut<EguiContext>,
    mut shop: ResMut<ShopRes>,
    mut buy_events: EventWriter<BuyEvent>,
    player: Query<Entity, With<Player>>,
) {
    let mut show = shop.show;
    egui::Window::new("Shop")
        .open(&mut show)
        .collapsible(false)
        .vscroll(true)
        .hscroll(true)
        // .default_pos([0.0, 0.0])
        // .min_width(600.0)
        // .min_height(400.0)
        // .default_size([600.0, 600.0])
        .resizable(true)
        // .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(egui_context.ctx_mut(), |ui| {
            egui::Grid::new("some_unique_id")
                .striped(true)
                .spacing([16.0, 16.0])
                .show(ui, |ui| {
                    for item in shop.shop_items.iter() {
                        // ui.label(&item.item_id);
                        ui.label("Name");
                        ui.label(item.price.to_string());
                        if ui.button("Buy!").clicked() {
                            if let Ok(entity) = player.get_single() {
                                buy_events.send(BuyEvent {
                                    buyer: entity,
                                    item_id: item.item_id,
                                    price: item.price,
                                });
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    shop.show = show;
}

fn open_shop(mut events: EventReader<OpenShopEvent>, mut shop: ResMut<ShopRes>) {
    for ev in events.iter() {
        shop.shop_items = ev.shop_items.clone();
        shop.show = true;
    }
}

fn buy_item(mut buy_events: EventReader<BuyEvent>, mut inventory_query: Query<&mut Inventory>) {
    for ev in buy_events.iter() {
        if let Ok(mut inventory) = inventory_query.get_mut(ev.buyer) {
            if inventory.money > ev.price {
                inventory.money -= ev.price;
                match inventory.items.get_mut(&ev.item_id) {
                    Some(qty) => {
                        *qty += 1;
                    }
                    None => {
                        inventory.items.insert(ev.item_id, 1);
                    }
                }
            }
        }
    }
}
