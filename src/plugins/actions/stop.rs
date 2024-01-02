use bevy::prelude::*;

use crate::plugins::{
    animation::ChangeAnimation,
    movement::Movement,
    unit_action::UnitAnimation,
    unit_state::{ActionSystemLabel, UnitActionEnterActiveEvent, UnitActionEnterEvent},
};

use super::{action::Skill, base::BaseSkill};

pub struct StopPlugin;
impl Plugin for StopPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<StopAction>()
            .add_system(attach)
            .add_system(enter.label(ActionSystemLabel::Enter))
            .add_system(enter_active.label(ActionSystemLabel::EnterActive));
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct StopAction {}

fn attach(mut commands: Commands, q: Query<(Entity, &Skill), Added<Skill>>) {
    for (e, a) in q.iter() {
        let setting = a.action_id.setting();
        if let BaseSkill::Stop = setting.base {
            commands.entity(e).insert(StopAction {});
        }
    }
}

fn enter(
    mut events: EventReader<UnitActionEnterEvent>,
    query: Query<(&StopAction,)>,
    mut unit_q: Query<(&mut Movement,)>,
    mut anim_events: EventWriter<ChangeAnimation>,
) {
    for ev in events.iter() {
        if let Ok((_,)) = query.get(ev.action) {
            if let Ok((mut movement,)) = unit_q.get_mut(ev.unit) {
                movement.speed = 0.0;
                anim_events.send(ChangeAnimation {
                    entity: ev.unit,
                    name: UnitAnimation::Idle.to_string(),
                });
            }
        }
    }
}

fn enter_active(
    mut events: EventReader<UnitActionEnterActiveEvent>,
    query: Query<(&StopAction,)>,
    mut unit_q: Query<(&mut Movement,)>,
) {
    for ev in events.iter() {
        if let Ok((_,)) = query.get(ev.action) {
            debug!("Stop enter active: {:?}", ev);
            if let Ok((mut movement,)) = unit_q.get_mut(ev.unit) {
                movement.speed = 0.0;
            }
        }
    }
}
