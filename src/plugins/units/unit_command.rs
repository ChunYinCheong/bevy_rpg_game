use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::plugins::{
    actions::{setting::TargetSetting, skill_id::SkillId},
    unit_state::{ChangeActionRequest, UnitState},
};

use super::unit::Unit;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Component, Reflect)]
pub struct UnitCommand {
    pub action_id: SkillId,
    /// check zero lenght
    pub movement_direction: Vec2,
    pub target_unit: Option<Entity>,
    pub target_position: Option<Vec2>,
    pub target_direction: Option<Vec2>,
}

#[derive(Debug, Default, Clone, Component, Reflect)]
pub struct UnitActionPlanner {}

pub fn planning(
    mut query: Query<(
        Entity,
        &UnitActionPlanner,
        &UnitState,
        &GlobalTransform,
        &mut UnitCommand,
    )>,
    unit_q: Query<(&Unit, &GlobalTransform)>,
    mut change_events: EventWriter<ChangeActionRequest>,
) {
    for (entity, _p, us, gt, mut uc) in query.iter_mut() {
        if !us.action_id.setting().cancelable {
            continue;
        }
        if us.command.as_ref() == Some(uc.as_ref()) {
            continue;
        }
        let setting = uc.action_id.setting();
        match setting.target_range {
            Some(range) => {
                match setting.target {
                    TargetSetting::None => {
                        change_events.send(ChangeActionRequest {
                            action_id: uc.action_id,
                            command: uc.clone(),
                            entity,
                        });
                    }
                    TargetSetting::Unit => {
                        if let Some(unit) = uc.target_unit {
                            if let Ok((unit, unit_gt)) = unit_q.get(unit) {
                                if unit.dead {
                                    uc.action_id = SkillId::Idle;
                                    continue;
                                }
                                let tran = gt.translation().truncate();
                                let unit_tran = unit_gt.translation().truncate();
                                let distance = tran.distance(unit_tran);
                                if distance <= range as f32 {
                                    change_events.send(ChangeActionRequest {
                                        action_id: uc.action_id,
                                        command: uc.clone(),
                                        entity,
                                    });
                                } else {
                                    // move
                                    change_events.send(ChangeActionRequest {
                                        action_id: SkillId::MoveTo,
                                        command: UnitCommand {
                                            action_id: SkillId::MoveTo,
                                            target_position: Some(unit_tran),
                                            ..Default::default()
                                        },
                                        entity,
                                    });
                                }
                            }
                        }
                    }
                    TargetSetting::Position => {
                        if let Some(pos) = uc.target_position {
                            let tran = gt.translation();
                            let distance = tran.truncate().distance(pos);
                            if distance <= range as f32 {
                                change_events.send(ChangeActionRequest {
                                    action_id: uc.action_id,
                                    command: uc.clone(),
                                    entity,
                                });
                            } else {
                                change_events.send(ChangeActionRequest {
                                    action_id: SkillId::MoveTo,
                                    command: UnitCommand {
                                        action_id: SkillId::MoveTo,
                                        target_position: Some(pos),
                                        ..Default::default()
                                    },
                                    entity,
                                });
                            }
                        }
                    }
                }
            }
            None => {
                change_events.send(ChangeActionRequest {
                    action_id: uc.action_id,
                    command: uc.clone(),
                    entity,
                });
            }
        }
    }
}
