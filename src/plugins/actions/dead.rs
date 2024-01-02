use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    plugins::{
        animation::ChangeAnimation,
        movement::Movement,
        unit_action::UnitAnimation,
        unit_state::{ActionSystemLabel, UnitActionEnterEvent},
    },
    NONE_GROUP,
};

use super::{action::Skill, base::BaseSkill};

pub struct DeadPlugin;
impl Plugin for DeadPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<DeadAction>()
            .add_system(attach)
            .add_system(enter.label(ActionSystemLabel::Enter));
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct DeadAction {}

fn attach(mut commands: Commands, q: Query<(Entity, &Skill), Added<Skill>>) {
    for (e, a) in q.iter() {
        let setting = a.action_id.setting();
        if let BaseSkill::Dead = setting.base {
            commands.entity(e).insert(DeadAction {});
        }
    }
}

fn enter(
    mut events: EventReader<UnitActionEnterEvent>,
    query: Query<(&DeadAction,)>,
    mut unit_q: Query<(&mut Movement, &mut CollisionGroups)>,
    mut anim_events: EventWriter<ChangeAnimation>,
) {
    for ev in events.iter() {
        if let Ok((_,)) = query.get(ev.action) {
            if let Ok((mut movement, mut flag)) = unit_q.get_mut(ev.unit) {
                movement.speed = 0.0;
                anim_events.send(ChangeAnimation {
                    entity: ev.unit,
                    name: UnitAnimation::Dead.to_string(),
                });
                flag.memberships = NONE_GROUP;
                flag.filters = NONE_GROUP;
            }
        }
    }
}
