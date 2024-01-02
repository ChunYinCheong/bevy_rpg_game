use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    plugins::actions::{action::Skill, base::BaseSkill, skill_id::SkillId},
    res::GameWorldConfig,
};

use super::{
    unit_action::UnitActions,
    unit_command::{planning, UnitCommand},
};

pub struct UnitStatePlugin;
impl Plugin for UnitStatePlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // .add_system(debug.after(state_transition))
            .register_type::<UnitCommand>()
            .register_type::<UnitState>()
            .add_system(planning.before(change_action_events))
            .add_event::<ChangeActionRequest>()
            .add_event::<StateTransitionRequest>()
            .add_system(change_action_events.before(state_transition))
            .add_event::<UnitActionEnterEvent>()
            .add_event::<UnitActionEnterActiveEvent>()
            .add_system(
                state_transition
                    .before(PassiveUpdateLabel)
                    .before(ActionSystemLabel::Enter)
                    .before(ActionSystemLabel::EnterActive)
                    .before(ActionSystemLabel::ActiveUpdate)
                    .before(ActionSystemLabel::ExitActive)
                    .before(ActionSystemLabel::Exit),
            )
            .add_event::<UnitActionActiveUpdateEvent>()
            .add_system(
                unit_state_update
                    .before(PassiveUpdateLabel)
                    .before(ActionSystemLabel::Enter)
                    .before(ActionSystemLabel::EnterActive)
                    .before(ActionSystemLabel::ActiveUpdate)
                    .before(ActionSystemLabel::ExitActive)
                    .before(ActionSystemLabel::Exit),
            )
            .add_event::<UnitPassiveUpdateEvent>()
            .add_system(
                unit_passive_update
                    .before(PassiveUpdateLabel)
                    .before(ActionSystemLabel::Enter)
                    .before(ActionSystemLabel::EnterActive)
                    .before(ActionSystemLabel::ActiveUpdate)
                    .before(ActionSystemLabel::ExitActive)
                    .before(ActionSystemLabel::Exit),
            )
            // Ordering for action system
            // PassiveUpdate / ActiveUpdate => ExitActive => Exit => Enter => EnterActive
            .add_system(
                (|| {
                    // debug!(" => ActiveUpdate");
                })
                .before(PassiveUpdateLabel)
                .before(ActionSystemLabel::ActiveUpdate),
            )
            .add_system(
                (|| {
                    // debug!("ActiveUpdate");
                })
                .label(PassiveUpdateLabel)
                .label(ActionSystemLabel::ActiveUpdate),
            )
            .add_system(
                (|| {
                    // debug!("ActiveUpdate => ExitActive");
                })
                .after(PassiveUpdateLabel)
                .after(ActionSystemLabel::ActiveUpdate)
                .before(ActionSystemLabel::ExitActive),
            )
            .add_system(
                (|| {
                    // debug!("ExitActive");
                })
                .label(ActionSystemLabel::ExitActive),
            )
            .add_system(
                (|| {
                    // debug!("ExitActive => Exit");
                })
                .after(ActionSystemLabel::ExitActive)
                .before(ActionSystemLabel::Exit),
            )
            .add_system(
                (|| {
                    // debug!("Exit");
                })
                .label(ActionSystemLabel::Exit),
            )
            .add_system(
                (|| {
                    // debug!("Exit => Enter");
                })
                .after(ActionSystemLabel::Exit)
                .before(ActionSystemLabel::Enter),
            )
            .add_system(
                (|| {
                    // debug!("Enter");
                })
                .label(ActionSystemLabel::Enter),
            )
            .add_system(
                (|| {
                    // debug!("Enter => EnterActive");
                })
                .after(ActionSystemLabel::Enter)
                .before(ActionSystemLabel::EnterActive),
            )
            .add_system(
                (|| {
                    // debug!("EnterActive");
                })
                .label(ActionSystemLabel::EnterActive),
            )
            .add_system(
                (|| {
                    // debug!("EnterActive =>");
                })
                .after(ActionSystemLabel::EnterActive),
            );
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Component, Reflect, Default)]
#[reflect_value()]
pub struct UnitState {
    pub action_id: SkillId,
    pub action_entity: Option<Entity>,
    pub action_state: ActionState,
    pub action_time: Option<f32>,

    pub command: Option<UnitCommand>,
}

#[derive(Debug, Clone, PartialEq, Copy, Eq, Serialize, Deserialize, Reflect)]
pub enum ActionState {
    Startup,
    Active,
    Recover,
}
impl Default for ActionState {
    fn default() -> Self {
        Self::Startup
    }
}

#[derive(Debug)]
pub struct UnitPassiveUpdateEvent {
    pub unit: Entity,
    pub skill: Entity,
}
fn unit_passive_update(
    config: Res<GameWorldConfig>,
    mut unit_q: Query<(Entity, &UnitState, &UnitActions)>,
    mut events: EventWriter<UnitPassiveUpdateEvent>,
) {
    if !config.active {
        return;
    }
    for (id, state, ua) in unit_q.iter_mut() {
        let setting = state.action_id.setting();
        match setting.base {
            BaseSkill::Dead => (),
            _ => {
                events.send_batch(ua.actions.iter().map(|skill| UnitPassiveUpdateEvent {
                    unit: id,
                    skill: *skill,
                }));
            }
        }
    }
}

#[derive(Debug)]
pub struct UnitActionActiveUpdateEvent {
    pub unit: Entity,
    pub action: Entity,
    pub action_id: SkillId,
}

pub fn unit_state_update(
    mut unit_q: Query<(Entity, &mut UnitState)>,
    time: Res<Time>,
    config: Res<GameWorldConfig>,
    mut transition_events: EventWriter<StateTransitionRequest>,
    mut update_events: EventWriter<UnitActionActiveUpdateEvent>,
) {
    if !config.active {
        return;
    }
    for (id, mut state) in unit_q.iter_mut() {
        let delta = time.delta_seconds();
        if let Some(time) = state.action_time.as_mut() {
            *time -= delta;
        }
        match state.action_state {
            ActionState::Startup => {
                // TODO: send startup update event
                if state.action_time.map_or(false, |t| t <= 0.0) {
                    transition_events.send(StateTransitionRequest {
                        unit: id,
                        transition: TransitionType::ActionState {
                            current_action_id: state.action_id,
                            target_action_state: ActionState::Active,
                        },
                    });
                }
            }
            ActionState::Active => {
                if let Some(action) = state.action_entity {
                    update_events.send(UnitActionActiveUpdateEvent {
                        unit: id,
                        action,
                        action_id: state.action_id,
                    });
                }
                if let Some(time) = state.action_time {
                    if time <= 0.0 {
                        transition_events.send(StateTransitionRequest {
                            unit: id,
                            transition: TransitionType::ActionState {
                                current_action_id: state.action_id,
                                target_action_state: ActionState::Recover,
                            },
                        });
                    }
                }
            }
            ActionState::Recover => {
                // TODO: send recover update event
                if state.action_time.map_or(false, |t| t <= 0.0) {
                    transition_events.send(StateTransitionRequest {
                        unit: id,
                        transition: TransitionType::Action(SkillId::Idle, UnitCommand::default()),
                    });
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ChangeActionRequest {
    pub action_id: SkillId,
    pub command: UnitCommand,
    pub entity: Entity,
}
pub fn change_action_events(
    mut events: EventReader<ChangeActionRequest>,
    mut transition_events: EventWriter<StateTransitionRequest>,
) {
    for event in events.iter() {
        // info!("{event:?}");
        transition_events.send(StateTransitionRequest {
            unit: event.entity,
            transition: TransitionType::Action(event.action_id, event.command.clone()),
        });
    }
}

#[derive(Debug, Clone)]
pub struct StateTransitionRequest {
    pub unit: Entity,
    pub transition: TransitionType,
}

#[derive(Debug, Clone)]
pub enum TransitionType {
    Action(SkillId, UnitCommand),
    ActionState {
        current_action_id: SkillId,
        target_action_state: ActionState,
    },
}

#[derive(Debug)]
pub struct UnitActionEnterEvent {
    pub unit: Entity,
    pub action_id: SkillId,
    pub action: Entity,
}

#[derive(Debug)]
pub struct UnitActionEnterActiveEvent {
    pub unit: Entity,
    pub action_id: SkillId,
    pub action: Entity,
}

pub fn state_transition(
    mut events: EventReader<StateTransitionRequest>,
    mut enter_active_events: EventWriter<UnitActionEnterActiveEvent>,
    mut enter_events: EventWriter<UnitActionEnterEvent>,
    mut unit_q: Query<(&mut UnitState, &UnitActions)>,
    action_q: Query<(Entity, &Skill)>,
) {
    for ev in events.iter() {
        if let Ok((mut us, ua)) = unit_q.get_mut(ev.unit) {
            match &ev.transition {
                TransitionType::Action(action_id, command) => {
                    if us.action_id == SkillId::Dead || us.action_id == SkillId::Stun {
                        continue;
                    }

                    us.action_id = *action_id;
                    us.command = Some(command.clone());
                    for entity in ua.actions.iter() {
                        if let Ok((e, action)) = action_q.get(*entity) {
                            if action.action_id == us.action_id {
                                us.action_entity = Some(e);
                            }
                        }
                    }

                    enter_events.send(UnitActionEnterEvent {
                        unit: ev.unit,
                        action_id: us.action_id,
                        action: us.action_entity.unwrap(),
                    });

                    let s = action_id.setting();
                    match s.action_state {
                        ActionState::Startup => {
                            us.action_time = s.startup_time;
                        }
                        ActionState::Active => {
                            us.action_time = s.active_time;

                            enter_active_events.send(UnitActionEnterActiveEvent {
                                unit: ev.unit,
                                action_id: us.action_id,
                                action: us.action_entity.unwrap(),
                            });
                            debug!(
                                "ev.unit: {:?}, unit.action_id: {:?}, unit.action_time: {:?}",
                                ev.unit, us.action_id, us.action_time
                            );
                        }
                        ActionState::Recover => {
                            us.action_time = s.recover_time;
                        }
                    }
                    us.action_state = s.action_state;
                    // unit.action_data = s.action_data;
                }
                TransitionType::ActionState {
                    current_action_id,
                    target_action_state,
                } => {
                    // Check it is still the current state
                    if *current_action_id == us.action_id && us.action_state != *target_action_state
                    {
                        us.action_state = *target_action_state;

                        let s = us.action_id.setting();
                        match target_action_state {
                            ActionState::Startup => {
                                // This should not happen
                                todo!();
                                // unit.action_time = s.startup_time;
                            }
                            ActionState::Active => {
                                us.action_time = s.active_time;

                                enter_active_events.send(UnitActionEnterActiveEvent {
                                    unit: ev.unit,
                                    action_id: us.action_id,
                                    action: us.action_entity.unwrap(),
                                });
                                // debug!(
                                //     "ev.unit: {:?}, unit.action_id: {:?}, unit.action_time: {:?}",
                                //     ev.unit, unit.action_id, unit.action_time
                                // );
                            }
                            ActionState::Recover => {
                                us.action_time = s.recover_time;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum ActionSystemLabel {
    Enter,
    EnterActive,
    ActiveUpdate,
    ExitActive,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub struct PassiveUpdateLabel;
