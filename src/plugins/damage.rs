use bevy::prelude::*;

use crate::utils::Knockback;

use super::{
    actions::skill_id::SkillId,
    hit::HitEvent,
    unit::{Unit, UnitDieEvent},
    unit_state::{ChangeActionRequest, UnitState},
};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<OnHitDamage>()
            .add_event::<HitDamageEvent>()
            .add_system(on_hit_damage)
            .add_event::<DamageEvent>()
            .add_system(damage_event)
            .add_event::<HealEvent>()
            .add_system(heal_event);
    }
}

#[derive(Debug, Component, Reflect)]
pub struct OnHitDamage {
    pub source: Entity,
    pub damage: i32,
    pub hit_stun: f32,
    pub knockback: Knockback,
}

#[derive(Debug)]
pub struct HitDamageEvent {
    pub source: Entity,
    pub victim: Entity,
    pub source_collider: Entity,
    pub damage: i32,
    pub hit_stun: f32,
    pub knockback: Knockback,
}

pub fn on_hit_damage(
    mut events: EventReader<HitEvent>,
    mut sends: EventWriter<HitDamageEvent>,
    query: Query<&OnHitDamage>,
) {
    for ev in events.iter() {
        if let Ok(hit_box) = query.get(ev.hit_entity) {
            let e = HitDamageEvent {
                source: hit_box.source,
                victim: ev.target_entity,
                source_collider: ev.hit_entity,
                damage: hit_box.damage,
                hit_stun: hit_box.hit_stun,
                knockback: hit_box.knockback,
            };
            info!("Send HitDamageEvent: {e:?}");
            sends.send(e);
        }
    }
}

#[derive(Debug)]
pub struct DamageEvent {
    pub unit: Entity,
    pub source_unit: Option<Entity>,
    pub damage: i32,
}
fn damage_event(
    mut events: EventReader<DamageEvent>,
    mut query: Query<&mut Unit>,
    mut die_events: EventWriter<UnitDieEvent>,
    mut change_events: EventWriter<ChangeActionRequest>,
) {
    for ev in events.iter() {
        if let Ok(mut unit) = query.get_mut(ev.unit) {
            unit.hp -= ev.damage;
            if unit.hp <= 0 && !unit.dead {
                unit.dead = true;
                die_events.send(UnitDieEvent(ev.unit));
                change_events.send(ChangeActionRequest {
                    action_id: SkillId::Dead,
                    command: default(),
                    entity: ev.unit,
                });
            }
        }
    }
}

#[derive(Debug)]
pub struct HealEvent {
    pub unit: Entity,
    pub source_unit: Option<Entity>,
    pub heal: i32,
}
fn heal_event(mut events: EventReader<HealEvent>, mut query: Query<(&mut Unit, &UnitState)>) {
    for ev in events.iter() {
        if let Ok((mut unit, us)) = query.get_mut(ev.unit) {
            if !unit.dead && us.action_id != SkillId::Dead {
                if unit.hp < unit.hp_max {
                    unit.hp += ev.heal;
                    if unit.hp > unit.hp_max {
                        unit.hp = unit.hp_max;
                    }
                }
            }
        }
    }
}
