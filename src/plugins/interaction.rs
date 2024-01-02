use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{utils, ALL_GROUP, INTERACT_GROUP, RAPIER_SCALE};

use super::{
    chest::ChestEvent,
    player::Hero,
    shop::{OpenShopEvent, Shop},
};

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<Interaction>()
            .add_event::<InteractEvent>()
            .add_system(check_interact)
            .add_system(interact_event)
            .register_type::<InteractArea>()
            .register_type::<InteractObject>()
            .register_type::<Interacting>()
            .add_system(interacting_detection)
            .add_system(show_interacting)
            .add_event::<InteractionEvent>()
            .add_system(unit_interacting);
    }
}

#[derive(Debug, Component, Reflect)]
pub enum Interaction {
    None,
    Shop,
    Talk,
    ResetPoint,
    Chest,
}

impl Default for Interaction {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug)]
pub struct InteractEvent {
    pub entity: Entity,
}

fn check_interact(
    rapier_context: Res<RapierContext>,

    player_q: Query<&GlobalTransform, With<Hero>>,
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
    let groups = CollisionGroups::new(ALL_GROUP, INTERACT_GROUP);
    warn!("InteractionGroups need to fix!");

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
    // mut reset_events: EventWriter<ResetEvent>,
    mut chest_events: EventWriter<ChestEvent>,
) {
    for ev in events.iter() {
        info!("{ev:?}");
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
                    todo!();
                    // reset_events.send(ResetEvent);
                }
                Interaction::Chest => {
                    chest_events.send(ChestEvent { chest: ev.entity });
                }
            }
        }
    }
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct InteractArea {
    pub focused: bool,
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct InteractObject;

#[derive(Debug, Clone, Component, Reflect)]
pub struct Interacting {
    pub target: Option<Entity>,
}

pub fn interacting_detection(
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<&mut InteractArea>,
    mut player_query: Query<&mut Interacting>,
) {
    for collision_event in collision_events.iter() {
        // info!("Received collision event: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(e1, e2, _flag) => {
                if let Ok(mut area) = query.get_mut(*e1) {
                    if let Ok(mut p) = player_query.get_mut(*e2) {
                        if p.target != Some(*e1) {
                            p.target = Some(*e1);
                        }
                        area.focused = true;
                    }
                } else if let Ok(mut area) = query.get_mut(*e2) {
                    if let Ok(mut p) = player_query.get_mut(*e1) {
                        if p.target != Some(*e2) {
                            p.target = Some(*e2);
                        }
                        area.focused = true;
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if let Ok(mut area) = query.get_mut(*e1) {
                    if let Ok(mut p) = player_query.get_mut(*e2) {
                        if p.target == Some(*e1) {
                            p.target = None;
                        }
                        area.focused = false;
                    }
                } else if let Ok(mut area) = query.get_mut(*e2) {
                    if let Ok(mut p) = player_query.get_mut(*e1) {
                        if p.target == Some(*e2) {
                            p.target = None;
                        }
                        area.focused = false;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct InteractionSprite;
pub fn show_interacting(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &InteractArea, Option<&Children>), Changed<InteractArea>>,
    sprite_q: Query<(&Parent, &InteractionSprite)>,
) {
    for (entity, a, children) in query.iter() {
        if a.focused {
            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn(Text2dBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "E".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 24.0,
                                    color: Color::BLUE,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(InteractionSprite);
            });
        } else {
            if let Some(children) = children {
                for &child in children.iter() {
                    if sprite_q.get(child).is_ok() {
                        commands.entity(child).despawn_recursive();
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct InteractionEvent {
    pub unit: Entity,
    pub target: Entity,
}
pub fn unit_interacting(
    keys: Res<Input<KeyCode>>,
    unit_query: Query<(Entity, &Interacting)>,
    mut events: EventWriter<InteractionEvent>,
) {
    if !keys.just_pressed(KeyCode::E) {
        return;
    }
    for (e, p) in unit_query.iter() {
        if let Some(target) = p.target {
            events.send(InteractionEvent { unit: e, target });
        }
    }
}
