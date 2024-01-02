use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Component, Reflect)]
pub enum Team {
    Player,
    Enemy,
}

impl Team {
    pub fn enemy_target(&self) -> Team {
        match &self {
            Team::Player => Team::Enemy,
            Team::Enemy => Team::Player,
        }
    }
    pub fn is_enemy(&self, team: &Team) -> bool {
        self != team
    }
    pub fn is_ally(&self, team: &Team) -> bool {
        self == team
    }
}
