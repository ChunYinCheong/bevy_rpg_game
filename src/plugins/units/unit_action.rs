use bevy::prelude::*;

pub struct UnitActionPlugin;
impl Plugin for UnitActionPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Debug, Default, Component)]
pub struct UnitActions {
    pub actions: Vec<Entity>,
}

#[derive(Debug, strum::Display)]
pub enum UnitAnimation {
    Idle,
    Dead,
    Move,
    Walk,
    Run,
    Attack,
    Stun,
    Stab,
    BurstFire,
    Hook,
    Fireball,
    Explosion,
    Burning,
    Drone,
    GhostLight,
}
