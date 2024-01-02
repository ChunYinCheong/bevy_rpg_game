use bevy::prelude::*;

use super::{
    attack::AttackPlugin, attack_aura::AttackAuraPlugin, burning::BurningPlugin,
    burst_fire::BurstFirePlugin, dead::DeadPlugin, dead_finger::DeadFingerPlugin,
    forbidden_array::ForbiddenArrayPlugin, ghost_light::GhostLightPlugin,
    heal_aura::HealAuraPlugin, hook::HookPlugin, ice_spear::IceSpearPlugin, idle::IdlePlugin,
    move_to::MoveToPlugin, skill_id::SkillId, slash::SlashPlugin,
    spider_attack::SpiderAttackPlugin, stab::StabPlugin, stop::StopPlugin, stun::StunPlugin,
    wolf_attack::WolfAttackPlugin,
};

pub struct ActionPlugin;
impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugin(StunPlugin)
            .add_plugin(DeadPlugin)
            .add_plugin(IdlePlugin)
            .add_plugin(StopPlugin)
            .add_plugin(MoveToPlugin)
            .add_plugin(AttackPlugin)
            //
            .add_plugin(BurningPlugin)
            .add_plugin(BurstFirePlugin)
            .add_plugin(ForbiddenArrayPlugin)
            .add_plugin(GhostLightPlugin)
            .add_plugin(HookPlugin)
            .add_plugin(IceSpearPlugin)
            .add_plugin(SlashPlugin)
            .add_plugin(SpiderAttackPlugin)
            .add_plugin(StabPlugin)
            .add_plugin(WolfAttackPlugin)
            .add_plugin(DeadFingerPlugin)
            .add_plugin(HealAuraPlugin)
            .add_plugin(AttackAuraPlugin)
            .register_type::<Skill>();
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct Skill {
    pub action_id: SkillId,
    // pub recharge: i32,
    // pub charged: i32,
    pub level: i32,
}
