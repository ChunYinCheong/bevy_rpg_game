use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    plugins::{
        animation::ChangeAnimation,
        movement::Movement,
        team::Team,
        unit::Unit,
        unit_action::UnitAnimation,
        unit_state::{ActionSystemLabel, UnitActionActiveUpdateEvent, UnitActionEnterEvent},
        units::unit_command::UnitCommand,
    },
    RAPIER_SCALE,
};

use super::{action::Skill, base::BaseSkill, skill_id::SkillId};

pub struct IdlePlugin;
impl Plugin for IdlePlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // .add_system(range_detection)
            .add_system(attach)
            .add_system(enter.label(ActionSystemLabel::Enter))
            // .add_system(active_update.label(ActionSystemLabel::ActiveUpdate))
            ;
    }
}

pub fn range_detection(
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<(&mut IdleAction, &Parent)>,
    unit_q: Query<(&Unit, &Team)>,
) {
    for collision_event in collision_events.iter() {
        // info!("Received collision event: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(e1, e2, _flag) => {
                if let Ok((mut idle, parent)) = query.get_mut(*e1) {
                    if parent.get() != *e2 {
                        if let Ok((unit, team)) = unit_q.get(*e2) {
                            if !unit.dead {
                                if let Ok((_, parent_team)) = unit_q.get(parent.get()) {
                                    if team != parent_team {
                                        idle.units.push(*e2);
                                    }
                                }
                            }
                        }
                    }
                }
                if let Ok((mut idle, parent)) = query.get_mut(*e2) {
                    if parent.get() != *e1 {
                        if let Ok((unit, team)) = unit_q.get(*e1) {
                            if !unit.dead {
                                if let Ok((_, parent_team)) = unit_q.get(parent.get()) {
                                    if team != parent_team {
                                        idle.units.push(*e1);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if let Ok((mut idle, _)) = query.get_mut(*e1) {
                    idle.units.retain(|e| e != e2);
                }
                if let Ok((mut idle, _)) = query.get_mut(*e2) {
                    idle.units.retain(|e| e != e1);
                }
            }
        }
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct IdleAction {
    pub units: Vec<Entity>,
}

fn attach(mut commands: Commands, q: Query<(Entity, &Skill), Added<Skill>>) {
    for (e, a) in q.iter() {
        let setting = a.action_id.setting();
        if let BaseSkill::Idle = setting.base {
            commands
                .entity(e)
                .insert(IdleAction::default())
                // .insert(Collider::ball(2.5 * RAPIER_SCALE))
                // .insert(ActiveEvents::COLLISION_EVENTS)
                // .insert(ActiveCollisionTypes::all())
                // .insert(Sensor)
                ;
        }
    }
}

fn enter(
    mut events: EventReader<UnitActionEnterEvent>,
    query: Query<(&IdleAction,)>,
    mut unit_q: Query<(&mut Movement,)>,
    mut anim_events: EventWriter<ChangeAnimation>,
) {
    for ev in events.iter() {
        if let Ok((_,)) = query.get(ev.action) {
            if let Ok((mut movement,)) = unit_q.get_mut(ev.unit) {
                // debug!("enter: {:?}", ev);
                movement.speed = 0.0;
                anim_events.send(ChangeAnimation {
                    entity: ev.unit,
                    name: UnitAnimation::Idle.to_string(),
                });
            }
        }
    }
}

pub fn active_update(
    mut events: EventReader<UnitActionActiveUpdateEvent>,
    mut query: Query<(&IdleAction, &Skill)>,
    mut unit_q: Query<(&mut UnitCommand, &GlobalTransform)>,
) {
    for ev in events.iter() {
        if let Ok((idle, _)) = query.get_mut(ev.action) {
            // debug!("Idle: {:?}", ev);
            if !idle.units.is_empty() {
                if let Ok((mut uc, _)) = unit_q.get_mut(ev.unit) {
                    // uc.action_id = SkillId::Attack;
                    // uc.target_unit = idle.units.get(0).cloned();
                }
            }
        }
    }
}
