use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    plugins::{
        team::Team,
        unit::{AttackModifierEvent, AttributeModifier},
        visual_effect::{VisualEffect, VisualEffectMarker},
    },
    RAPIER_SCALE,
};

use super::{action::Skill, base::BaseSkill};

pub struct AttackAuraPlugin;
impl Plugin for AttackAuraPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_system(attach)
            .add_system(collision_detection)
            .add_event::<EnterAura>()
            .add_system(enter_aura)
            .add_event::<LeaveAura>()
            .add_system(leave_aura);
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct AttackAuraAction {
    pub cooldown: f32,
}

fn attach(mut commands: Commands, q: Query<(Entity, &Skill), Added<Skill>>) {
    for (e, a) in q.iter() {
        let setting = a.action_id.setting();
        if let BaseSkill::AttackAura(base) = setting.base {
            commands
                .entity(e)
                .insert(AttackAuraAction::default())
                .insert(SpatialBundle::default())
                .insert((
                    Collider::ball(base.radius * RAPIER_SCALE),
                    ActiveEvents::COLLISION_EVENTS,
                    ActiveCollisionTypes::DYNAMIC_STATIC | ActiveCollisionTypes::KINEMATIC_STATIC,
                    Sensor,
                    RigidBody::Fixed,
                ));
        }
    }
}

fn collision_detection(
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<(&AttackAuraAction,)>,
    mut enter_events: EventWriter<EnterAura>,
    mut leave_events: EventWriter<LeaveAura>,
) {
    for collision_event in collision_events.iter() {
        info!("Received collision event: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(e1, e2, _flag) => {
                if let Ok((_act,)) = query.get(*e1) {
                    enter_events.send(EnterAura {
                        skill: *e1,
                        entity: *e2,
                    });
                }
                if let Ok((_act,)) = query.get(*e2) {
                    enter_events.send(EnterAura {
                        skill: *e2,
                        entity: *e1,
                    });
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if let Ok((_act,)) = query.get(*e1) {
                    leave_events.send(LeaveAura {
                        skill: *e1,
                        entity: *e2,
                    });
                }
                if let Ok((_act,)) = query.get(*e2) {
                    leave_events.send(LeaveAura {
                        skill: *e2,
                        entity: *e1,
                    });
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct EnterAura {
    pub skill: Entity,
    pub entity: Entity,
}
fn enter_aura(
    mut events: EventReader<EnterAura>,
    query: Query<(&AttackAuraAction, &Skill, &Parent)>,
    team_q: Query<&Team>,
    mut mod_ev: EventWriter<AttackModifierEvent>,
    mut commands: Commands,
) {
    for ev in events.iter() {
        debug!("{ev:?}");
        if let Ok((_aura, skill, parent)) = query.get(ev.skill) {
            let setting = skill.action_id.setting();
            if let BaseSkill::AttackAura(base) = setting.base {
                if let Ok([team, target_team]) = team_q.get_many([parent.get(), ev.entity]) {
                    if team.is_ally(target_team) {
                        mod_ev.send(AttackModifierEvent::Add(
                            ev.entity,
                            AttributeModifier {
                                source: ev.skill,
                                amount: 0,
                                percentage: base.percentage.get(skill.level),
                            },
                        ));

                        commands.entity(ev.entity).with_children(|commands| {
                            commands.spawn(VisualEffectMarker {
                                visual_effect: VisualEffect::SmashWave,
                                duration: None,
                                repeat: true,
                                size: Vec2 { x: 50.0, y: 50.0 },
                                auto_despawn: false,
                                pos: default(),
                            });
                        });
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct LeaveAura {
    pub skill: Entity,
    pub entity: Entity,
}
fn leave_aura(mut events: EventReader<LeaveAura>, mut mod_ev: EventWriter<AttackModifierEvent>) {
    for ev in events.iter() {
        debug!("{ev:?}");
        mod_ev.send(AttackModifierEvent::Remove(ev.entity, ev.skill));
    }
}
