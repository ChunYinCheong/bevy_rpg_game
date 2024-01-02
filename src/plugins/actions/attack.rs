use core::str;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    plugins::{
        animation::ChangeAnimation,
        damage::DamageEvent,
        movement::Movement,
        team::Team,
        unit::Unit,
        unit_action::{UnitActions, UnitAnimation},
        unit_state::{
            ActionSystemLabel, UnitActionEnterActiveEvent, UnitActionEnterEvent, UnitState,
        },
        visual_effect::{VisualEffect, VisualEffectMarker},
    },
    RAPIER_SCALE,
};

use super::{action::Skill, base::BaseSkill};

pub struct AttackPlugin;
impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<AttackAction>()
            .add_system(attach)
            .add_system(enter.label(ActionSystemLabel::Enter))
            .add_system(enter_active.label(ActionSystemLabel::EnterActive))
            .add_event::<AttackEvent>()
            .add_system(attack_event);
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct AttackAction {}

fn attach(mut commands: Commands, q: Query<(Entity, &Skill), Added<Skill>>) {
    for (e, a) in q.iter() {
        let setting = a.action_id.setting();
        if let BaseSkill::Attack = setting.base {
            commands.entity(e).insert(AttackAction {});
        }
    }
}

fn enter(
    mut events: EventReader<UnitActionEnterEvent>,
    query: Query<(&AttackAction, &Skill)>,
    mut unit_q: Query<(&Unit, &UnitState, &mut Movement, &GlobalTransform)>,
    mut anim_events: EventWriter<ChangeAnimation>,
) {
    for ev in events.iter() {
        if let Ok((_, _)) = query.get(ev.action) {
            if let Ok((_u, us, mut movement, gt)) = unit_q.get_mut(ev.unit) {
                movement.speed = 0.0;
                if let Some(pos) = us.command.as_ref().and_then(|c| c.target_position) {
                    let dir = pos - gt.translation().truncate();
                    movement.face = Some(dir);
                }
                anim_events.send(ChangeAnimation {
                    entity: ev.unit,
                    name: UnitAnimation::Attack.to_string(),
                });
            }
        }
    }
}

fn enter_active(
    mut events: EventReader<UnitActionEnterActiveEvent>,
    mut query: Query<(&mut AttackAction,)>,
    unit_q: Query<(&Unit, &UnitState)>,
    mut sends: EventWriter<AttackEvent>,
) {
    for ev in events.iter() {
        if let Ok((mut _a,)) = query.get_mut(ev.action) {
            debug!("Attack enter active: {:?}", ev);
            if let Ok((_u, us)) = unit_q.get(ev.unit) {
                if let Some(uc) = &us.command {
                    if let Some(target) = uc.target_unit {
                        // let e = HitDamageEvent {
                        //     source: ev.unit,
                        //     victim: target,
                        //     source_collider: ev.unit,
                        //     damage: u.atk,
                        //     hit_stun: 0.0,
                        //     knockback: Knockback::None,
                        // };
                        // info!("Send HitDamageEvent: {e:?}");
                        // sends.send(e);
                        let e = AttackEvent {
                            attacker: ev.unit,
                            target,
                        };
                        info!("Send HitDamageEvent: {e:?}");
                        sends.send(e);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub target: Entity,
}

fn attack_event(
    mut events: EventReader<AttackEvent>,
    mut damage_ev: EventWriter<DamageEvent>,
    rapier_context: Res<RapierContext>,
    unit_q: Query<(&Unit, &UnitActions, &GlobalTransform, &Team)>,
    skill_q: Query<&Skill>,
    team_q: Query<&Team>,
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();
    for ev in events.iter() {
        if let Ok([attacker, target]) = unit_q.get_many([ev.attacker, ev.target]) {
            let mut damage = attacker.0.atk;
            let mut actions: Vec<_> = attacker
                .1
                .actions
                .iter()
                .flat_map(|entity| skill_q.get(*entity))
                .filter(|action| {
                    let settting = action.action_id.setting();
                    matches!(
                        settting.base,
                        BaseSkill::LifeSteal
                            | BaseSkill::CriticalHit
                            | BaseSkill::Diffusion(_)
                            | BaseSkill::FrostBall(_)
                            | BaseSkill::SmashWave(_)
                    )
                })
                .collect();
            actions.sort_by_key(|action| {
                let settting = action.action_id.setting();
                match settting.base {
                    BaseSkill::LifeSteal => 10,
                    BaseSkill::CriticalHit => 1,
                    BaseSkill::Diffusion(_) => 9,
                    BaseSkill::FrostBall(_) => 1,
                    BaseSkill::SmashWave(_) => 1,
                    _ => 0,
                }
            });
            for skill in actions {
                let settting = skill.action_id.setting();
                match settting.base {
                    BaseSkill::LifeSteal => {
                        todo!();
                    }
                    BaseSkill::CriticalHit => {
                        let percent = 5;
                        if rng.gen_ratio(percent, 100) {
                            damage *= 2;
                        }
                    }
                    BaseSkill::Diffusion(d) => {
                        let radius = d.radius * RAPIER_SCALE;
                        let pos = target.2.translation().truncate();

                        commands.spawn(VisualEffectMarker {
                            visual_effect: VisualEffect::Diffusion,
                            duration: Some(0.5),
                            repeat: false,
                            size: Vec2::new(radius * 2.0, radius * 2.0),
                            auto_despawn: false,
                            pos,
                        });

                        let shape = Collider::ball(radius);
                        let shape_pos = pos;
                        let shape_rot = 0.0;
                        let filter = QueryFilter::default();
                        rapier_context.intersections_with_shape(
                            shape_pos,
                            shape_rot,
                            &shape,
                            filter,
                            |entity| {
                                if entity != ev.target {
                                    if let Ok(team) = team_q.get(entity) {
                                        if attacker.3.is_enemy(team) {
                                            damage_ev.send(DamageEvent {
                                                unit: entity,
                                                source_unit: Some(ev.attacker),
                                                damage: damage * d.percentage.get(skill.level)
                                                    / 100,
                                            });
                                        }
                                    }
                                }
                                true
                            },
                        );
                    }
                    BaseSkill::FrostBall(fb) => {
                        // if rng.gen_bool(0.05) {}
                        // let percent = 50;
                        if rng.gen_ratio(fb.chance, 100) {
                            let radius = fb.radius * RAPIER_SCALE;
                            let pos = target.2.translation().truncate();

                            commands.spawn(VisualEffectMarker {
                                visual_effect: VisualEffect::FrostBall,
                                duration: Some(0.5),
                                repeat: false,
                                size: Vec2::new(radius * 2.0, radius * 2.0),
                                auto_despawn: false,
                                pos,
                            });

                            let shape = Collider::ball(radius);
                            let shape_pos = pos;
                            let shape_rot = 0.0;
                            let filter = QueryFilter::default();
                            rapier_context.intersections_with_shape(
                                shape_pos,
                                shape_rot,
                                &shape,
                                filter,
                                |entity| {
                                    if let Ok(team) = team_q.get(entity) {
                                        if attacker.3.is_enemy(team) {
                                            damage_ev.send(DamageEvent {
                                                unit: entity,
                                                source_unit: Some(ev.attacker),
                                                damage: fb.damage.get(skill.level),
                                            });
                                        }
                                    }
                                    true
                                },
                            );
                        }
                    }
                    BaseSkill::SmashWave(sw) => {
                        if rng.gen_ratio(sw.chance, 100) {
                            let radius = sw.radius * RAPIER_SCALE;
                            let pos = attacker.2.translation().truncate();

                            commands.spawn(VisualEffectMarker {
                                visual_effect: VisualEffect::SmashWave,
                                duration: Some(0.5),
                                repeat: false,
                                size: Vec2::new(radius * 2.0, radius * 2.0),
                                auto_despawn: false,
                                pos,
                            });

                            let shape = Collider::ball(radius);
                            let shape_pos = pos;
                            let shape_rot = 0.0;
                            let filter = QueryFilter::default();
                            rapier_context.intersections_with_shape(
                                shape_pos,
                                shape_rot,
                                &shape,
                                filter,
                                |entity| {
                                    if let Ok(team) = team_q.get(entity) {
                                        if attacker.3.is_enemy(team) {
                                            damage_ev.send(DamageEvent {
                                                unit: entity,
                                                source_unit: Some(ev.attacker),
                                                damage: sw.damage.get(skill.level),
                                            });
                                        }
                                    }
                                    true
                                },
                            );
                        }
                    }
                    _ => {
                        error!("Missing filter / handle in Attack");
                    }
                }
            }

            damage_ev.send(DamageEvent {
                unit: ev.target,
                source_unit: Some(ev.attacker),
                damage,
            });
        }
    }
}
