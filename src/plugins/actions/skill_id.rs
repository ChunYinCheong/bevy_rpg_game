use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::plugins::unit_state::ActionState;

use super::{
    base::{
        AttackAura, BaseSkill, BurstFire, DeadFinger, Diffusion, FrostBall, GhostLight, HealAura,
        LifeDrain, SmashWave, SpeedAura, Thunder, Value,
    },
    setting::{SkillSetting, SkillType, TargetSetting},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Component, Reflect)]
pub enum SkillId {
    Idle,
    Stun,
    Dead,
    MoveTo,
    Attack,
    Slash,
    ForbiddenArray,
    IceSpear,
    Stab,
    BurstFire,
    Hook,
    Fireball,
    Explosion,
    Burning,
    Drone,
    GhostLight,
    SpiderAttack,
    WolfAttack,
    // Active
    DeadFinger,
    // FireBreath,
    // Continuous
    Thunder,
    LifeDrain,
    // Summon
    // SummonElement,
    // Passive
    LifeSteal,
    Diffusion,
    FrostBall,
    SmashWave,
    // Aura
    HealAura,
    AttackAura,
    SpeedAura,
}

impl Default for SkillId {
    fn default() -> Self {
        Self::Idle
    }
}

impl SkillId {
    pub fn setting(&self) -> SkillSetting {
        match self {
            SkillId::Idle => SkillSetting {
                startup_time: Some(0.1),
                active_time: None,
                recover_time: Some(0.2),
                action_id: SkillId::Idle,
                action_state: ActionState::Startup,
                cancelable: true,
                target: TargetSetting::None,
                target_range: None,
                base: BaseSkill::Idle,
                skill_type: SkillType::Active,
                name: "Idle",
                desc: "Just idle!",
                icon: "images/particlePack_1.1/PNG (Transparent)/circle_01.png",
            },
            SkillId::Stun => SkillSetting {
                startup_time: None,
                active_time: None,
                recover_time: None,
                action_id: SkillId::Stun,
                action_state: ActionState::Active,
                cancelable: false,
                target: TargetSetting::None,
                target_range: None,
                base: BaseSkill::Stun,
                skill_type: SkillType::Active,
                name: "Stun",
                desc: "Stunning!",
                icon: "images/particlePack_1.1/PNG (Transparent)/circle_01.png",
            },
            SkillId::Dead => SkillSetting {
                startup_time: None,
                active_time: None,
                recover_time: None,
                action_id: SkillId::Dead,
                action_state: ActionState::Active,
                cancelable: false,
                target: TargetSetting::None,
                target_range: None,
                base: BaseSkill::Dead,
                skill_type: SkillType::Active,
                name: "Dead",
                desc: "You dead!",
                icon: "images/particlePack_1.1/PNG (Transparent)/circle_01.png",
            },
            SkillId::MoveTo => SkillSetting {
                startup_time: None,
                active_time: None,
                recover_time: None,
                action_id: SkillId::MoveTo,
                action_state: ActionState::Active,
                cancelable: true,
                target: TargetSetting::Position,
                target_range: None,
                base: BaseSkill::MoveTo,
                skill_type: SkillType::Active,
                name: "Move",
                desc: "Moving!",
                icon: "images/particlePack_1.1/PNG (Transparent)/circle_01.png",
            },
            SkillId::Attack => SkillSetting {
                startup_time: Some(0.2),
                active_time: Some(0.2),
                recover_time: Some(0.2),
                action_id: SkillId::Attack,
                action_state: ActionState::Startup,
                cancelable: true,
                target: TargetSetting::Unit,
                target_range: Some(100),
                base: BaseSkill::Attack,
                skill_type: SkillType::Active,
                name: "Attack",
                desc: "Attack!",
                icon: "images/particlePack_1.1/PNG (Transparent)/circle_01.png",
            },
            SkillId::Slash => SkillSetting {
                startup_time: Some(0.1),
                active_time: Some(0.05),
                recover_time: Some(0.2),
                action_id: SkillId::Slash,
                action_state: ActionState::Startup,
                cancelable: false,
                target: TargetSetting::Unit,
                target_range: None,
                base: BaseSkill::Slash,
                skill_type: SkillType::Active,
                name: "Slash",
                desc: "Slash!",
                icon: "images/particlePack_1.1/PNG (Transparent)/dirt_01.png",
            },
            SkillId::ForbiddenArray => SkillSetting {
                startup_time: Some(0.1),
                active_time: Some(0.05),
                recover_time: Some(0.2),
                action_id: SkillId::ForbiddenArray,
                action_state: ActionState::Startup,
                cancelable: false,
                target: TargetSetting::None,
                target_range: None,
                base: BaseSkill::ForbiddenArray,
                skill_type: SkillType::Active,
                name: "ForbiddenArray",
                desc: "ForbiddenArray!",
                icon: "images/particlePack_1.1/PNG (Transparent)/fire_01.png",
            },
            SkillId::IceSpear => SkillSetting {
                startup_time: Some(0.1),
                active_time: Some(0.05),
                recover_time: Some(0.2),
                action_id: SkillId::IceSpear,
                action_state: ActionState::Startup,
                cancelable: false,
                target: TargetSetting::Position,
                target_range: None,
                base: BaseSkill::IceSpear,
                skill_type: SkillType::Active,
                name: "IceSpear",
                desc: "IceSpear!",
                icon: "images/particlePack_1.1/PNG (Transparent)/flame_01.png",
            },
            SkillId::Stab => SkillSetting {
                startup_time: Some(0.1),
                active_time: Some(0.05),
                recover_time: Some(0.2),
                action_id: SkillId::Stab,
                action_state: ActionState::Startup,
                cancelable: false,
                target: TargetSetting::Position,
                target_range: None,
                base: BaseSkill::Stab,
                skill_type: SkillType::Active,
                name: "Stab",
                desc: "Stab!",
                icon: "images/particlePack_1.1/PNG (Transparent)/light_01.png",
            },
            SkillId::BurstFire => SkillSetting {
                startup_time: Some(0.1),
                active_time: Some(0.15),
                recover_time: Some(0.0),
                action_id: SkillId::BurstFire,
                action_state: ActionState::Startup,
                cancelable: false,
                target: TargetSetting::Position,
                target_range: None,
                base: BaseSkill::BurstFire(BurstFire { f: 0.15, i: 0 }),
                skill_type: SkillType::Active,
                name: "BurstFire",
                desc: "BurstFire!",
                icon: "images/particlePack_1.1/PNG (Transparent)/magic_03.png",
            },
            SkillId::Hook => SkillSetting {
                startup_time: Some(0.2),
                active_time: Some(0.2),
                recover_time: Some(0.0),
                action_id: SkillId::Hook,
                action_state: ActionState::Startup,
                cancelable: false,
                target: TargetSetting::Position,
                target_range: None,
                base: BaseSkill::Hook,
                skill_type: SkillType::Active,
                name: "Hook",
                desc: "Hook!",
                icon: "images/particlePack_1.1/PNG (Transparent)/muzzle_01.png",
            },
            SkillId::Fireball => SkillSetting {
                startup_time: Some(0.1),
                active_time: Some(0.05),
                recover_time: Some(0.1),
                action_id: SkillId::Fireball,
                action_state: ActionState::Startup,
                cancelable: false,
                target: TargetSetting::Position,
                target_range: None,
                base: BaseSkill::Fireball,
                skill_type: SkillType::Active,
                name: "Fireball",
                desc: "Fireball!",
                icon: "images/particlePack_1.1/PNG (Transparent)/muzzle_02.png",
            },
            SkillId::Explosion => SkillSetting {
                startup_time: Some(0.1),
                active_time: Some(0.05),
                recover_time: Some(0.1),
                action_id: SkillId::Explosion,
                action_state: ActionState::Startup,
                cancelable: false,
                target: TargetSetting::Position,
                target_range: None,
                base: BaseSkill::Explosion,
                skill_type: SkillType::Active,
                name: "Explosion",
                desc: "Explosion!",
                icon: "images/particlePack_1.1/PNG (Transparent)/scorch_02.png",
            },
            SkillId::Burning => SkillSetting {
                startup_time: Some(0.5),
                active_time: Some(0.05),
                recover_time: Some(1.5),
                action_id: SkillId::Burning,
                action_state: ActionState::Startup,
                cancelable: false,
                target: TargetSetting::Position,
                target_range: None,
                base: BaseSkill::Burning,
                skill_type: SkillType::Active,
                name: "Burning",
                desc: "Burning!",
                icon: "images/particlePack_1.1/PNG (Transparent)/scorch_03.png",
            },
            SkillId::Drone => SkillSetting {
                startup_time: Some(0.1),
                active_time: Some(0.05),
                recover_time: Some(0.1),
                action_id: SkillId::Drone,
                action_state: ActionState::Startup,
                cancelable: false,
                target: TargetSetting::Position,
                target_range: None,
                base: BaseSkill::Drone,
                skill_type: SkillType::Active,
                name: "Drone",
                desc: "Drone!",
                icon: "images/particlePack_1.1/PNG (Transparent)/slash_01.png",
            },
            SkillId::GhostLight => SkillSetting {
                startup_time: Some(0.1),
                active_time: None,
                recover_time: Some(1.0),
                action_id: SkillId::GhostLight,
                action_state: ActionState::Startup,
                cancelable: false,
                target: TargetSetting::None,
                target_range: None,
                base: BaseSkill::GhostLight(GhostLight { f: 1.0, i: 0 }),
                skill_type: SkillType::Active,
                name: "GhostLight",
                desc: "GhostLight!",
                icon: "images/particlePack_1.1/PNG (Transparent)/magic_01.png",
            },
            SkillId::SpiderAttack => SkillSetting {
                startup_time: Some(0.5),
                active_time: Some(0.05),
                recover_time: Some(0.5),
                action_id: SkillId::SpiderAttack,
                action_state: ActionState::Startup,
                cancelable: false,
                target: TargetSetting::None,
                target_range: None,
                base: BaseSkill::SpiderAttack,
                skill_type: SkillType::Active,
                name: "SpiderAttack",
                desc: "SpiderAttack!",
                icon: "images/particlePack_1.1/PNG (Transparent)/star_01.png",
            },
            SkillId::WolfAttack => SkillSetting {
                startup_time: Some(0.5),
                active_time: Some(0.5),
                recover_time: Some(0.5),
                action_id: SkillId::WolfAttack,
                action_state: ActionState::Startup,
                cancelable: false,
                target: TargetSetting::None,
                target_range: None,
                base: BaseSkill::WolfAttack,
                skill_type: SkillType::Active,
                name: "WolfAttack",
                desc: "WolfAttack!",
                icon: "images/particlePack_1.1/PNG (Transparent)/scratch_01.png",
            },
            SkillId::LifeSteal => SkillSetting {
                action_id: SkillId::LifeSteal,
                startup_time: None,
                active_time: None,
                recover_time: None,
                action_state: ActionState::Active,
                cancelable: false,
                target: TargetSetting::None,
                target_range: None,
                name: "LifeSteal",
                desc: "LifeSteal",
                icon: "images/particlePack_1.1/PNG (Transparent)/symbol_01.png",
                base: BaseSkill::LifeSteal,
                skill_type: SkillType::Passive,
            },
            SkillId::Diffusion => SkillSetting {
                action_id: SkillId::LifeSteal,
                startup_time: None,
                active_time: None,
                recover_time: None,
                action_state: ActionState::Active,
                cancelable: false,
                target: TargetSetting::None,
                target_range: None,
                name: "Diffusion",
                desc: "Deals damage to nearby enemies when attacking",
                icon: "images/particlePack_1.1/PNG (Transparent)/light_03.png",
                base: BaseSkill::Diffusion(Diffusion {
                    percentage: Value::Multiply(20),
                    radius: 2.0,
                }),
                skill_type: SkillType::Passive,
            },
            SkillId::FrostBall => SkillSetting {
                action_id: SkillId::LifeSteal,
                startup_time: None,
                active_time: None,
                recover_time: None,
                action_state: ActionState::Active,
                cancelable: false,
                target: TargetSetting::None,
                target_range: None,
                name: "FrostBall",
                desc: "FrostBall",
                icon: "images/particlePack_1.1/PNG (Transparent)/dirt_03.png",
                base: BaseSkill::FrostBall(FrostBall {
                    chance: 20,
                    damage: Value::Multiply(5),
                    radius: 1.0,
                }),
                skill_type: SkillType::Passive,
            },
            SkillId::SmashWave => SkillSetting {
                action_id: SkillId::LifeSteal,
                startup_time: None,
                active_time: None,
                recover_time: None,
                action_state: ActionState::Active,
                cancelable: false,
                target: TargetSetting::None,
                target_range: None,
                name: "Smash Wave",
                desc: "Chance to damage nearby enemies when attacking",
                icon: "images/particlePack_1.1/PNG (Transparent)/light_01.png",
                base: BaseSkill::SmashWave(SmashWave {
                    chance: 20,
                    damage: Value::Multiply(5),
                    radius: 2.0,
                }),
                skill_type: SkillType::Passive,
            },
            SkillId::DeadFinger => SkillSetting {
                startup_time: Some(0.2),
                active_time: Some(0.2),
                recover_time: Some(0.0),
                action_id: SkillId::DeadFinger,
                action_state: ActionState::Startup,
                cancelable: false,
                target: TargetSetting::Unit,
                target_range: None,
                base: BaseSkill::DeadFinger(DeadFinger {
                    damage: Value::Multiply(100),
                }),
                skill_type: SkillType::Active,
                name: "Dead Finger",
                desc: "Deal massive damage to target",
                icon: "images/particlePack_1.1/PNG (Transparent)/muzzle_01.png",
            },
            SkillId::Thunder => SkillSetting {
                startup_time: Some(0.2),
                active_time: Some(0.2),
                recover_time: Some(0.0),
                action_id: SkillId::Thunder,
                action_state: ActionState::Startup,
                cancelable: false,
                target: TargetSetting::Position,
                target_range: None,
                base: BaseSkill::Thunder(Thunder {
                    duration: 5.0,
                    radius: 3.0,
                    damage: Value::Multiply(5),
                }),
                skill_type: SkillType::Active,
                name: "Thunder",
                desc: "Deal continuous damage in range",
                icon: "images/particlePack_1.1/PNG (Transparent)/muzzle_01.png",
            },
            SkillId::LifeDrain => SkillSetting {
                startup_time: Some(0.2),
                active_time: Some(0.2),
                recover_time: Some(0.0),
                action_id: SkillId::LifeDrain,
                action_state: ActionState::Startup,
                cancelable: false,
                target: TargetSetting::Position,
                target_range: None,
                base: BaseSkill::LifeDrain(LifeDrain {
                    duration: 5.0,
                    amount: Value::Multiply(5),
                }),
                skill_type: SkillType::Active,
                name: "LifeDrain",
                desc: "LifeDrain!",
                icon: "images/particlePack_1.1/PNG (Transparent)/muzzle_01.png",
            },
            SkillId::HealAura => SkillSetting {
                action_id: SkillId::HealAura,
                startup_time: None,
                active_time: None,
                recover_time: None,
                action_state: ActionState::Active,
                cancelable: false,
                target: TargetSetting::None,
                target_range: None,
                name: "Heal Aura",
                desc: "Heal nearby allies",
                icon: "images/particlePack_1.1/PNG (Transparent)/light_01.png",
                base: BaseSkill::HealAura(HealAura {
                    radius: 5.0,
                    percentage: Value::Multiply(1),
                }),
                skill_type: SkillType::Passive,
            },
            SkillId::AttackAura => SkillSetting {
                action_id: SkillId::AttackAura,
                startup_time: None,
                active_time: None,
                recover_time: None,
                action_state: ActionState::Active,
                cancelable: false,
                target: TargetSetting::None,
                target_range: None,
                name: "Attack Aura",
                desc: "Increases the attack power of nearby allies",
                icon: "images/particlePack_1.1/PNG (Transparent)/light_01.png",
                base: BaseSkill::AttackAura(AttackAura {
                    radius: 5.0,
                    percentage: Value::Multiply(20),
                }),
                skill_type: SkillType::Passive,
            },
            SkillId::SpeedAura => SkillSetting {
                action_id: SkillId::SpeedAura,
                startup_time: None,
                active_time: None,
                recover_time: None,
                action_state: ActionState::Active,
                cancelable: false,
                target: TargetSetting::None,
                target_range: None,
                name: "SpeedAura",
                desc: "Increases the cooldown speed and attack speed of nearby allies",
                icon: "images/particlePack_1.1/PNG (Transparent)/light_01.png",
                base: BaseSkill::SpeedAura(SpeedAura {
                    radius: 2.0,
                    percentage: Value::Multiply(20),
                }),
                skill_type: SkillType::Passive,
            },
        }
    }
}
