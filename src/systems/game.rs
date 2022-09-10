use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    plugins::{player::Player, unit::Unit, unit_action::ActionId, unit_state::UnitCommand},
    res::GameWorldConfig,
};

pub fn pause_game(
    mut rapier: ResMut<RapierConfiguration>,
    mut config: ResMut<GameWorldConfig>,
    player_q: Query<(&Unit, &UnitCommand), With<Player>>,
) {
    let _active = player_q
        .get_single()
        .map(|(player, command)| {
            player.action_id != ActionId::Idle || command.action_id != ActionId::Idle
        })
        .unwrap_or(true);
    let active = true;
    rapier.physics_pipeline_active = active;
    config.active = active;
}
