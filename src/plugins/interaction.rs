use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;

use crate::{utils, INTERACT_GROUP, RAPIER_SCALE};

use super::{
    game_world::ResetEvent,
    player::Player,
    shop::{OpenShopEvent, Shop},
};

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_inspectable::<Interaction>()
            .add_system(check_interact)
            .add_system(interact_event)
            .add_event::<InteractEvent>();
    }
}

#[derive(Debug, Component, Inspectable)]
pub enum Interaction {
    None,
    Shop,
    Talk,
    ResetPoint,
}

impl Default for Interaction {
    fn default() -> Self {
        Self::None
    }
}

pub struct InteractEvent {
    pub entity: Entity,
}

fn check_interact(
    rapier_context: Res<RapierContext>,

    player_q: Query<&GlobalTransform, With<Player>>,
    mut events: EventWriter<InteractEvent>,
    keys: Res<Input<KeyCode>>,
) {
    if !keys.just_pressed(KeyCode::E) {
        return;
    }
    let player = player_q.get_single();
    if player.is_err() {
        return;
    }
    let player = player.unwrap();

    let forward = utils::get_forward_global(player);
    let point = player.translation().truncate() + forward * 0.75 * RAPIER_SCALE;
    let solid = true;
    let groups = InteractionGroups::new(u32::MAX, INTERACT_GROUP);
    let filter = QueryFilter {
        groups: Some(groups),
        ..Default::default()
    };

    if let Some((entity, projection)) = rapier_context.project_point(point, solid, filter) {
        // dbg!(entity, projection, point, player.translation);
        if projection.is_inside {
            events.send(InteractEvent { entity });
        }
    }
}

fn interact_event(
    mut events: EventReader<InteractEvent>,
    interactions: Query<&Interaction>,
    shop_query: Query<&Shop>,
    mut shop_events: EventWriter<OpenShopEvent>,
    mut reset_events: EventWriter<ResetEvent>,
) {
    for ev in events.iter() {
        if let Ok(interact) = interactions.get(ev.entity) {
            match interact {
                Interaction::None => (),
                Interaction::Shop => {
                    if let Ok(shop) = shop_query.get(ev.entity) {
                        shop_events.send(OpenShopEvent {
                            shop_items: shop.items.clone(),
                        });
                    }
                }
                Interaction::Talk => todo!(),
                Interaction::ResetPoint => {
                    reset_events.send(ResetEvent);
                }
            }
        }
    }
}
