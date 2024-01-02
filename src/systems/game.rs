use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    plugins::{
        actions::skill_id::SkillId, player::Hero, unit::Unit, unit_state::UnitState,
        units::unit_command::UnitCommand,
    },
    res::GameWorldConfig,
};

pub fn pause_game(
    mut rapier: ResMut<RapierConfiguration>,
    mut config: ResMut<GameWorldConfig>,
    player_q: Query<(&Unit, &UnitState, &UnitCommand), With<Hero>>,
) {
    let _active = player_q
        .get_single()
        .map(|(_, unit_state, command)| {
            unit_state.action_id != SkillId::Idle || command.action_id != SkillId::Idle
        })
        .unwrap_or(true);
    let active = true;
    rapier.physics_pipeline_active = active;
    config.active = active;
}
