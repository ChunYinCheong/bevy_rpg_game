use crate::plugins::{actions::skill_id::SkillId, unit_state::ActionState};

use super::base::BaseSkill;

#[derive(Debug, Clone)]
pub struct SkillSetting<'a> {
    pub action_id: SkillId,

    pub startup_time: Option<f32>,
    pub active_time: Option<f32>,
    pub recover_time: Option<f32>,
    pub action_state: ActionState,
    pub cancelable: bool,
    pub target: TargetSetting,
    pub target_range: Option<u32>,

    pub name: &'a str,
    pub desc: &'a str,
    pub icon: &'a str,
    pub base: BaseSkill,

    pub skill_type: SkillType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillType {
    Active,
    Passive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetSetting {
    None,
    Unit,
    Position,
}
