use bevy::prelude::*;

use crate::plugins::{
    animation::ChangeAnimation,
    movement::Movement,
    unit::Unit,
    unit_action::UnitAnimation,
    unit_state::{ActionSystemLabel, UnitActionActiveUpdateEvent, UnitActionEnterEvent, UnitState},
    units::unit_command::UnitCommand,
};

use super::{action::Skill, base::BaseSkill, skill_id::SkillId};

pub struct MoveToPlugin;
impl Plugin for MoveToPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_system(attach)
            .add_system(enter.label(ActionSystemLabel::Enter))
            .add_system(active_update.label(ActionSystemLabel::ActiveUpdate));
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct MoveToAction;

fn attach(mut commands: Commands, q: Query<(Entity, &Skill), Added<Skill>>) {
    for (e, a) in q.iter() {
        let setting = a.action_id.setting();
        if let BaseSkill::MoveTo = setting.base {
            commands.entity(e).insert(MoveToAction);
        }
    }
}

fn enter(
    mut events: EventReader<UnitActionEnterEvent>,
    query: Query<(&MoveToAction,)>,
    mut unit_q: Query<(&mut Movement, &Unit)>,
    mut anim_events: EventWriter<ChangeAnimation>,
) {
    for ev in events.iter() {
        if let Ok((_,)) = query.get(ev.action) {
            if let Ok((mut movement, unit)) = unit_q.get_mut(ev.unit) {
                movement.speed = unit.movement_speed;
                anim_events.send(ChangeAnimation {
                    entity: ev.unit,
                    name: UnitAnimation::Move.to_string(),
                });
            }
        }
    }
}

pub fn active_update(
    mut events: EventReader<UnitActionActiveUpdateEvent>,
    mut query: Query<(&MoveToAction, &Skill)>,
    mut unit_q: Query<(
        &Unit,
        &mut UnitCommand,
        &GlobalTransform,
        &mut Movement,
        &UnitState,
    )>,
) {
    for ev in events.iter() {
        if let Ok((_, _)) = query.get_mut(ev.action) {
            // debug!("MoveTo: {:?}", ev);
            if let Ok((u, mut command, position, mut movement, us)) = unit_q.get_mut(ev.unit) {
                if let Some(pos) = us.command.as_ref().and_then(|c| c.target_position) {
                    let dir = pos - position.translation().truncate();
                    // debug!("dir.length_squared(): {}", dir.length_squared());
                    if dir.length_squared() > 100.0 {
                        movement.face = Some(dir);
                        movement.direction = dir;
                        movement.speed = u.movement_speed;
                    } else {
                        movement.speed = 0.0;
                        command.action_id = SkillId::Idle;
                    }
                }
            }
        }
    }
}
