use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{player::Player, unit_action::ActionId};

pub struct ItemPlugin;
impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<Inventory>()
            .register_inspectable::<Equipment>()
            .init_resource::<InventoryUiRes>()
            .add_system(inventory_ui)
            .add_event::<OpenInventoryEvent>()
            .add_system(open_inventory_ui)
            .add_event::<EquipEvent>()
            .add_system(equip)
            .add_event::<SwitchEquipment>()
            .add_system(switch);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Component, Inspectable)]
pub struct Inventory {
    pub items: HashMap<ItemId, i32>,
    pub money: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Inspectable)]
pub enum ItemId {
    None,
    HpPotion,
    MpPotion,
    Sword,
    Spear,
}
impl Default for ItemId {
    fn default() -> Self {
        Self::None
    }
}
impl ItemId {
    pub fn setting(&self) -> ItemaSetting {
        match self {
            ItemId::None => ItemaSetting {
                name: "None".to_string(),
                kind: ItemKind::None,
            },
            ItemId::HpPotion => ItemaSetting {
                name: "HpPotion".to_string(),
                kind: ItemKind::Consume,
            },
            ItemId::MpPotion => ItemaSetting {
                name: "MpPotion".to_string(),
                kind: ItemKind::Consume,
            },
            ItemId::Sword => ItemaSetting {
                name: "Sword".to_string(),
                kind: ItemKind::Weapon(Weapon {
                    main_action_id: ActionId::Attack,
                    sub_action_id: ActionId::ForbiddenArray,
                }),
            },
            ItemId::Spear => ItemaSetting {
                name: "Spear".to_string(),
                kind: ItemKind::Weapon(Weapon {
                    main_action_id: ActionId::Stab,
                    sub_action_id: ActionId::IceSpear,
                }),
            },
        }
    }
}

#[derive(Debug, Default, Clone, Component, Inspectable)]
pub struct ItemaSetting {
    pub name: String,
    pub kind: ItemKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, Inspectable)]
pub enum ItemKind {
    None,
    Weapon(Weapon),
    Consume,
}
impl Default for ItemKind {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Inspectable)]
pub struct Weapon {
    pub main_action_id: ActionId,
    pub sub_action_id: ActionId,
}

#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Inspectable, Component,
)]
pub struct Equipment {
    pub current: usize,
    pub weapons: [ItemId; 3],
}

#[derive(Debug, Default)]
pub struct OpenInventoryEvent;
fn open_inventory_ui(
    mut events: EventReader<OpenInventoryEvent>,
    mut shop: ResMut<InventoryUiRes>,
) {
    for _ev in events.iter() {
        shop.show = !shop.show;
    }
}

#[derive(Debug, Default)]
pub struct InventoryUiRes {
    pub show: bool,
}
pub fn inventory_ui(
    mut egui_context: ResMut<EguiContext>,
    mut inventory_ui: ResMut<InventoryUiRes>,
    query: Query<(&Inventory, &Equipment), With<Player>>,
    mut events: EventWriter<EquipEvent>,
    mut switch_events: EventWriter<SwitchEquipment>,
) {
    if let Ok((inventory, equipment)) = query.get_single() {
        let inventory_ui = &mut *inventory_ui;
        let show = &mut inventory_ui.show;
        egui::Window::new("Item")
            .open(show)
            .collapsible(false)
            .vscroll(true)
            .hscroll(true)
            .resizable(true)
            .show(egui_context.ctx_mut(), |ui| {
                // ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label("Equipment");
                egui::TopBottomPanel::top("equipment").show_inside(ui, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.scope(|ui| {
                            if equipment.current == 0 {
                                ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
                            }
                            let w = equipment.weapons[0];
                            if ui.button(w.setting().name).clicked() {
                                if equipment.current != 0 {
                                    switch_events.send(SwitchEquipment { slot: 0 });
                                }
                            };
                        });
                        ui.separator();
                        ui.scope(|ui| {
                            if equipment.current == 1 {
                                ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
                            }
                            let w = equipment.weapons[1];
                            if ui.button(w.setting().name).clicked() {
                                if equipment.current != 1 {
                                    switch_events.send(SwitchEquipment { slot: 1 });
                                }
                            };
                        });
                        ui.separator();
                        ui.scope(|ui| {
                            if equipment.current == 2 {
                                ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
                            }
                            let w = equipment.weapons[2];
                            if ui.button(w.setting().name).clicked() {
                                if equipment.current != 2 {
                                    switch_events.send(SwitchEquipment { slot: 2 });
                                }
                            };
                        });
                    });
                });
                ui.separator();
                ui.label("Inventory");
                egui::Grid::new("some_unique_id")
                    .striped(true)
                    .spacing([16.0, 16.0])
                    .show(ui, |ui| {
                        for (k, v) in inventory.items.iter() {
                            let item = k.setting();
                            ui.label(item.name);
                            ui.label(v.to_string());
                            match item.kind {
                                ItemKind::None => (),
                                ItemKind::Weapon(_) => {
                                    if ui.button("Equip").clicked() {
                                        events.send(EquipEvent {
                                            slot: equipment.current,
                                            item_id: *k,
                                        });
                                    }
                                }
                                ItemKind::Consume => {
                                    if ui.button("Use").clicked() {
                                        // events.send(EquipEvent {
                                        //     slot: inventory_ui.focus,
                                        //     item_id: *k,
                                        // });
                                    }
                                }
                            }
                            ui.end_row();
                        }
                    });
                // });
            });
    }
}

pub struct EquipEvent {
    pub slot: usize,
    pub item_id: ItemId,
}
pub fn equip(
    mut events: EventReader<EquipEvent>,
    mut query: Query<(&mut Equipment,), With<Player>>,
) {
    for ev in events.iter() {
        if let Ok((mut equipment,)) = query.get_single_mut() {
            equipment.weapons[ev.slot] = ev.item_id;
        }
    }
}

pub struct SwitchEquipment {
    pub slot: usize,
}
pub fn switch(
    mut events: EventReader<SwitchEquipment>,
    mut query: Query<(&mut Equipment,), With<Player>>,
) {
    for ev in events.iter() {
        if let Ok((mut equipment,)) = query.get_single_mut() {
            equipment.current = ev.slot;
        }
    }
}
