use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_rapier2d::prelude::CollisionEvent;
use rand::prelude::*;

use crate::plugins::{
    actions::{action::Skill, skill_id::SkillId},
    item::ItemId,
    player::Hero,
    unit_action::UnitActions,
};

#[derive(Debug, Clone, Default, Component, Reflect)]
pub struct ShopSlot {
    pub price: i32,
    pub inside: bool,
    pub icon: Option<String>,

    pub name: String,
    pub desc: String,
}

#[derive(Debug, Clone, Default, Component, Reflect)]
#[reflect_value()]
pub struct SlotItem {
    pub item: Option<ItemId>,
}

#[derive(Debug, Clone, Default, Component, Reflect)]
#[reflect_value()]
pub struct SlotAction {
    pub action_id: Option<SkillId>,
}

const ITEMS: [ItemId; 3] = [ItemId::Sword, ItemId::HpPotion, ItemId::MpPotion];
const SKILLS: [SkillId; 9] = [
    SkillId::DeadFinger,
    SkillId::Thunder,
    SkillId::LifeDrain,
    // SkillId::LifeSteal,
    SkillId::Diffusion,
    SkillId::FrostBall,
    SkillId::SmashWave,
    SkillId::HealAura,
    SkillId::AttackAura,
    SkillId::SpeedAura,
];

#[derive(Debug, Clone, Default)]
pub struct RefreshShopEvent;
pub fn refresh_shop(
    mut events: EventReader<RefreshShopEvent>,
    mut sa_q: Query<&mut SlotAction>,
    mut si_q: Query<&mut SlotItem>,
) {
    for _ in events.iter() {
        for mut sa in sa_q.iter_mut() {
            let mut rng = thread_rng();
            sa.action_id = SKILLS.choose(&mut rng).copied();
        }
        for mut si in si_q.iter_mut() {
            let mut rng = thread_rng();
            si.item = ITEMS.choose(&mut rng).copied();
        }
    }
}

pub fn shop_slot_changed(
    asset_server: Res<AssetServer>,
    mut items_q: Query<(&mut Handle<Image>, &ShopSlot), Changed<ShopSlot>>,
) {
    for (mut image, slot) in items_q.iter_mut() {
        // slot.icon
        // image
        match &slot.icon {
            Some(path) => {
                *image = asset_server.load(path);
            }
            None => {
                // default image?
                *image =
                    asset_server.load("images/particlePack_1.1/PNG (Transparent)/circle_01.png");
            }
        }
    }
}

pub fn slot_item_changed(mut items_q: Query<(&mut ShopSlot, &SlotItem), Changed<SlotItem>>) {
    for (mut slot, i) in items_q.iter_mut() {
        if let Some(item) = i.item {
            let s = item.setting();
            slot.icon = Some(s.icon);
        } else {
            // clear the sprite / show empty
            slot.icon = None;
        }
    }
}

pub fn slot_action_changed(mut items_q: Query<(&mut ShopSlot, &SlotAction), Changed<SlotAction>>) {
    for (mut slot, i) in items_q.iter_mut() {
        if let Some(item) = i.action_id {
            let s = item.setting();
            slot.name = s.name.to_string();
            slot.desc = s.desc.to_string();
            slot.icon = Some(s.icon.to_string());
        } else {
            // clear the sprite / show empty
            slot.icon = None;
        }
    }
}

pub fn slot_detection(
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<&mut ShopSlot>,
    player_query: Query<&Hero>,
) {
    for collision_event in collision_events.iter() {
        // info!("Received collision event: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(e1, e2, _flag) => {
                if let Ok(mut slot) = query.get_mut(*e1) {
                    if player_query.get(*e2).is_ok() {
                        slot.inside = true;
                    }
                } else if let Ok(mut slot) = query.get_mut(*e2) {
                    if player_query.get(*e1).is_ok() {
                        slot.inside = true;
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if let Ok(mut slot) = query.get_mut(*e1) {
                    if player_query.get(*e2).is_ok() {
                        slot.inside = false;
                    }
                } else if let Ok(mut slot) = query.get_mut(*e2) {
                    if player_query.get(*e1).is_ok() {
                        slot.inside = false;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ShopSlotBuyEvent {
    pub slot: Entity,
}

pub fn slot_ui(
    mut egui_context: ResMut<EguiContext>,
    mut query: Query<(Entity, &mut ShopSlot)>,
    mut events: EventWriter<ShopSlotBuyEvent>,
    // mut buy_events: EventWriter<BuyEvent>,
) {
    for (e, mut s) in query.iter_mut() {
        if s.inside {
            egui::Window::new(format!("{e:?}"))
                .title_bar(false)
                .collapsible(false)
                .min_height(100.0)
                .min_width(100.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(egui_context.ctx_mut(), |ui| {
                    ui.label(&s.name);
                    ui.label(&s.desc);

                    ui.horizontal(|ui| {
                        if ui.button("Buy").clicked() {
                            s.inside = false;
                            events.send(ShopSlotBuyEvent { slot: e });
                        }
                        if ui.button("Close").clicked() {
                            s.inside = false;
                        }
                    });
                });
        }
    }
}

pub fn buy_action(
    mut events: EventReader<ShopSlotBuyEvent>,
    mut query: Query<(&mut ShopSlot, &mut SlotAction)>,
    mut write_events: EventWriter<HeroGetAction>,
) {
    for ev in events.iter() {
        if let Ok((mut slot, mut action)) = query.get_mut(ev.slot) {
            if let Some(action_id) = action.action_id {
                slot.name = "".to_string();
                slot.desc = "".to_string();
                slot.icon = None;
                slot.price = 0;
                write_events.send(HeroGetAction { action_id });
                action.action_id = None;
            }
        }
    }
}

#[derive(Debug)]
pub struct HeroGetAction {
    pub action_id: SkillId,
}
pub fn hero_get_action(
    mut events: EventReader<HeroGetAction>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut UnitActions), With<Hero>>,
    mut action_q: Query<&mut Skill>,
) {
    for ev in events.iter() {
        if let Ok((hero_id, mut ua)) = query.get_single_mut() {
            let mut found = false;
            for a in ua.actions.iter() {
                if let Ok(mut action) = action_q.get_mut(*a) {
                    if action.action_id == ev.action_id {
                        action.level += 1;
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                let id = commands
                    .spawn(Skill {
                        action_id: ev.action_id,
                        level: 1,
                        ..Default::default()
                    })
                    .id();
                ua.actions.push(id);
                commands.entity(hero_id).add_child(id);
            }
        }
    }
}
