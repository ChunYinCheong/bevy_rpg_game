use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use serde::{Deserialize, Serialize};

use crate::res::GameWorldConfig;

use super::{unit::Unit, unit_action::ActionId};

pub struct UnitStatePlugin;

impl Plugin for UnitStatePlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_inspectable::<UnitCommand>()
            .add_event::<UnitStateEvent>()
            .add_event::<ChangeActionRequest>()
            .add_system(unit_state)
            .add_system(change_events);
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct UnitCommand {
    pub action_id: ActionId,
    /// check zero lenght
    pub movement_direction: Vec2,
    pub target_unit: Option<Entity>,
    pub target_position: Option<Vec2>,
    pub target_direction: Option<Vec2>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Inspectable)]
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
pub struct UnitStateEvent {
    pub id: Entity,
    pub action: ActionId,
    pub kind: ActionStateEventKind,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ActionStateEventKind {
    Enter,
    EnterActive,
    UpdateActive,
    ExitActive,
    Exit,
}

#[derive(Debug)]
pub struct ChangeActionRequest {
    pub action_id: ActionId,
    pub entity: Entity,
}

pub fn unit_state(
    mut unit_q: Query<(Entity, &mut Unit)>,
    time: Res<Time>,
    config: Res<GameWorldConfig>,
    mut events: EventWriter<UnitStateEvent>,
) {
    if !config.active {
        return;
    }
    for (id, mut state) in unit_q.iter_mut() {
        // info!("{id:?}, {state:?}");
        let delta = time.delta_seconds();
        if let Some(time) = state.action_time.as_mut() {
            *time -= delta;
        }
        match state.action_state {
            ActionState::Startup => {
                if state.action_time.map_or(false, |t| t <= 0.0) {
                    state.action_state = ActionState::Active;
                    state.action_time = state.action_id.setting().active_time;
                    events.send(UnitStateEvent {
                        id,
                        action: state.action_id,
                        kind: ActionStateEventKind::EnterActive,
                    });
                }
            }
            ActionState::Active => match state.action_time {
                Some(ref mut time) => {
                    *time -= delta;
                    let time = *time;

                    events.send(UnitStateEvent {
                        id,
                        action: state.action_id,
                        kind: ActionStateEventKind::UpdateActive,
                    });
                    if time <= 0.0 {
                        state.action_state = ActionState::Recover;
                        state.action_time = state.action_id.setting().recover_time;
                        events.send(UnitStateEvent {
                            id,
                            action: state.action_id,
                            kind: ActionStateEventKind::ExitActive,
                        });
                    }
                }
                None => {
                    events.send(UnitStateEvent {
                        id,
                        action: state.action_id,
                        kind: ActionStateEventKind::UpdateActive,
                    });
                }
            },
            ActionState::Recover => {
                if state.action_time.map_or(false, |t| t <= 0.0) {
                    state.action_id = ActionId::Idle;
                    state.action_state = ActionState::Active;
                    state.action_time = None;

                    events.send(UnitStateEvent {
                        id,
                        action: state.action_id,
                        kind: ActionStateEventKind::Enter,
                    });
                }
            }
        }
    }
}

pub fn change_events(
    mut events: EventReader<ChangeActionRequest>,
    mut state_events: EventWriter<UnitStateEvent>,
    mut unit_q: Query<(&mut Unit,)>,
) {
    for event in events.iter() {
        // info!("{event:?}");
        if let Ok((mut unit,)) = unit_q.get_mut(event.entity) {
            if unit.action_id == ActionId::Dead || unit.action_id == ActionId::Stun {
                continue;
            }

            if unit.action_state == ActionState::Active {
                state_events.send(UnitStateEvent {
                    id: event.entity,
                    action: unit.action_id,
                    kind: ActionStateEventKind::ExitActive,
                });
            }
            state_events.send(UnitStateEvent {
                id: event.entity,
                action: unit.action_id,
                kind: ActionStateEventKind::Exit,
            });

            unit.action_id = event.action_id;
            let s = event.action_id.setting();
            match s.action_state {
                ActionState::Startup => {
                    unit.action_time = s.startup_time;
                }
                ActionState::Active => {
                    unit.action_time = s.active_time;
                }
                ActionState::Recover => {
                    unit.action_time = s.recover_time;
                }
            }
            unit.action_state = s.action_state;
            unit.action_data = s.action_data;

            state_events.send(UnitStateEvent {
                id: event.entity,
                action: unit.action_id,
                kind: ActionStateEventKind::Enter,
            });
        }
    }
}
