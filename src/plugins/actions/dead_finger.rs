use bevy::prelude::*;

use crate::plugins::{
    animation::ChangeAnimation,
    damage::DamageEvent,
    movement::Movement,
    unit_action::UnitAnimation,
    unit_state::{ActionSystemLabel, UnitActionEnterActiveEvent, UnitActionEnterEvent, UnitState},
    visual_effect::{VisualEffect, VisualEffectMarker},
};

use super::{action::Skill, base::BaseSkill};

pub struct DeadFingerPlugin;
impl Plugin for DeadFingerPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<DeadFingerAction>()
            .add_system(attach)
            .add_system(enter.label(ActionSystemLabel::Enter))
            .add_system(enter_active.label(ActionSystemLabel::EnterActive));
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct DeadFingerAction {}

fn attach(mut commands: Commands, q: Query<(Entity, &Skill), Added<Skill>>) {
    for (e, a) in q.iter() {
        let setting = a.action_id.setting();
        if let BaseSkill::DeadFinger(_) = setting.base {
            commands.entity(e).insert(DeadFingerAction {});
        }
    }
}

fn enter(
    mut events: EventReader<UnitActionEnterEvent>,
    query: Query<(&DeadFingerAction,)>,
    mut unit_q: Query<(&mut Movement,)>,
    mut anim_events: EventWriter<ChangeAnimation>,
) {
    for ev in events.iter() {
        if let Ok((_,)) = query.get(ev.action) {
            if let Ok((mut movement,)) = unit_q.get_mut(ev.unit) {
                movement.speed = 0.0;
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
    query: Query<(&DeadFingerAction, &Skill)>,
    unit_q: Query<(&UnitState,)>,
    pos_q: Query<&GlobalTransform>,

    mut damage_ev: EventWriter<DamageEvent>,
    mut commands: Commands,
) {
    for ev in events.iter() {
        if let Ok((_, skill)) = query.get(ev.action) {
            debug!("DeadFinger enter active: {:?}", ev);
            let setting = skill.action_id.setting();
            if let BaseSkill::DeadFinger(base) = setting.base {
                if let Ok((us,)) = unit_q.get(ev.unit) {
                    if let Some(uc) = &us.command {
                        if let Some(target) = uc.target_unit {
                            if let Ok(gt) = pos_q.get(target) {
                                commands.spawn(VisualEffectMarker {
                                    visual_effect: VisualEffect::DeadFinger,
                                    duration: Some(0.5),
                                    repeat: false,
                                    size: Vec2 { x: 50.0, y: 50.0 },
                                    auto_despawn: false,
                                    pos: gt.translation().truncate(),
                                });
                            }
                            damage_ev.send(DamageEvent {
                                unit: target,
                                source_unit: Some(ev.unit),
                                damage: base.damage.get(skill.level),
                            });
                        }
                    }
                }
            }
        }
    }
}
