use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    plugins::{
        damage::HealEvent,
        team::Team,
        unit::Unit,
        unit_state::{PassiveUpdateLabel, UnitPassiveUpdateEvent},
        visual_effect::{VisualEffect, VisualEffectMarker},
    },
    RAPIER_SCALE,
};

use super::{action::Skill, base::BaseSkill};

pub struct HealAuraPlugin;
impl Plugin for HealAuraPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_system(attach)
            .add_system(passive_update.label(PassiveUpdateLabel));
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct HealAuraAction {
    pub cooldown: f32,
}

fn attach(mut commands: Commands, q: Query<(Entity, &Skill), Added<Skill>>) {
    for (e, a) in q.iter() {
        let setting = a.action_id.setting();
        if let BaseSkill::HealAura(base) = setting.base {
            commands.entity(e).insert(HealAuraAction::default());
        }
    }
}

pub fn passive_update(
    mut events: EventReader<UnitPassiveUpdateEvent>,
    mut query: Query<(&mut HealAuraAction, &Skill)>,
    unit_q: Query<(&Unit, &Team, &GlobalTransform)>,
    owner_q: Query<(&Team, &GlobalTransform)>,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    mut heal_ev: EventWriter<HealEvent>,
    mut commands: Commands,
) {
    for ev in events.iter() {
        if let Ok((mut act, skill)) = query.get_mut(ev.skill) {
            act.cooldown -= time.delta_seconds();
            if act.cooldown > 0.0 {
                continue;
            } else {
                act.cooldown = 1.0;
            }
            // debug!("passive_update: {ev:?}, {act:?}");
            let setting = skill.action_id.setting();
            if let BaseSkill::HealAura(base) = setting.base {
                if let Ok((team, gt)) = owner_q.get(ev.unit) {
                    let shape = Collider::ball(base.radius * RAPIER_SCALE);
                    let shape_pos = gt.translation().truncate();
                    let shape_rot = 0.0;
                    let filter = QueryFilter::default();
                    rapier_context.intersections_with_shape(
                        shape_pos,
                        shape_rot,
                        &shape,
                        filter,
                        |entity| {
                            if let Ok((unit, t, gt)) = unit_q.get(entity) {
                                if team.is_ally(t) {
                                    let heal = unit.hp_max * base.percentage.get(skill.level) / 100;
                                    heal_ev.send(HealEvent {
                                        unit: entity,
                                        source_unit: Some(ev.unit),
                                        heal,
                                    });

                                    commands.spawn(VisualEffectMarker {
                                        visual_effect: VisualEffect::Heal,
                                        duration: Some(0.5),
                                        repeat: false,
                                        size: Vec2 { x: 50.0, y: 50.0 },
                                        auto_despawn: false,
                                        pos: gt.translation().truncate(),
                                    });
                                }
                            }
                            true
                        },
                    );
                }
            }
        }
    }
}
