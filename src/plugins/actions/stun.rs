use bevy::prelude::*;

use crate::plugins::{
    animation::ChangeAnimation,
    movement::Movement,
    unit_action::UnitAnimation,
    unit_state::{ActionSystemLabel, UnitActionEnterEvent},
};

use super::{action::Skill, base::BaseSkill};

pub struct StunPlugin;
impl Plugin for StunPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<StunAction>()
            .add_system(attach)
            .add_system(enter.label(ActionSystemLabel::Enter));
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct StunAction {}

fn attach(mut commands: Commands, q: Query<(Entity, &Skill), Added<Skill>>) {
    for (e, a) in q.iter() {
        let setting = a.action_id.setting();
        if let BaseSkill::Stun = setting.base {
            commands.entity(e).insert(StunAction {});
        }
    }
}

fn enter(
    mut events: EventReader<UnitActionEnterEvent>,
    query: Query<(&StunAction,)>,
    mut unit_q: Query<(&mut Movement,)>,
    mut anim_events: EventWriter<ChangeAnimation>,
) {
    for ev in events.iter() {
        if let Ok((_,)) = query.get(ev.action) {
            if let Ok((mut movement,)) = unit_q.get_mut(ev.unit) {
                movement.speed = 0.0;
                anim_events.send(ChangeAnimation {
                    entity: ev.unit,
                    name: UnitAnimation::Stun.to_string(),
                });
            }
        }
    }
}
