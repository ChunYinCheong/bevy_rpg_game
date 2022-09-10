use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;

use crate::utils::Knockback;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_system(hit_detection)
            .register_inspectable::<HitBox>()
            .add_event::<HitEvent>();
    }
}

#[derive(Debug, Component, Inspectable)]
pub struct HitBox {
    pub source: Entity,
    pub damage: i32,
    pub hit_stun: f32,
    pub knockback: Knockback,
}

#[derive(Debug)]
pub struct HitEvent {
    pub source: Entity,
    pub victim: Entity,
    pub source_collider: Entity,
    pub damage: i32,
    pub hit_stun: f32,
    pub knockback: Knockback,
}

pub fn hit_detection(
    mut collision_events: EventReader<CollisionEvent>,
    mut events: EventWriter<HitEvent>,
    query: Query<&HitBox>,
) {
    for collision_event in collision_events.iter() {
        info!("Received collision event: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(e1, e2, _flag) => {
                let r = {
                    let mut temp = None;
                    if let Ok(hit_box) = query.get(*e1) {
                        if *e2 != hit_box.source {
                            temp = Some((hit_box, *e2, *e1));
                        }
                    } else if let Ok(hit_box) = query.get(*e2) {
                        if *e1 != hit_box.source {
                            temp = Some((hit_box, *e1, *e2));
                        }
                    }
                    temp
                };
                if let Some((hit_box, victim, source_collider)) = r {
                    events.send(HitEvent {
                        source: hit_box.source,
                        victim,
                        source_collider,
                        damage: hit_box.damage,
                        hit_stun: hit_box.hit_stun,
                        knockback: hit_box.knockback,
                    });
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
