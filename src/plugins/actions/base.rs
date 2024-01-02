use std::ops::Mul;

use bevy::prelude::*;

#[derive(Debug, Clone, Reflect)]
#[reflect_value()]
pub enum BaseSkill {
    Stun,
    Dead,
    Stop,
    Idle,
    MoveTo,
    Attack,
    Slash,
    ForbiddenArray,
    IceSpear,
    Stab,
    BurstFire(BurstFire),
    Hook,
    Fireball,
    Explosion,
    Burning,
    Drone,
    GhostLight(GhostLight),
    SpiderAttack,
    WolfAttack,
    //
    // Active
    //
    DeadFinger(DeadFinger),
    FireBreath(FireBreath),
    //
    // Continuous
    //
    Thunder(Thunder),
    LifeDrain(LifeDrain),
    //
    // Passive
    //
    LifeSteal,
    CriticalHit,
    Diffusion(Diffusion),
    FrostBall(FrostBall),
    SmashWave(SmashWave),
    //
    // Aura
    //
    HealAura(HealAura),
    AttackAura(AttackAura),
    SpeedAura(SpeedAura),
}

#[derive(Debug, Clone, Reflect)]
pub enum Value<T: Default + Reflect + FromReflect> {
    Fixed(T),
    /// Multiply by level
    Multiply(T),
}
impl<T: Copy + From<i32> + Mul<Output = T> + Default + Reflect + FromReflect> Value<T> {
    pub fn get(&self, level: i32) -> T {
        match self {
            Value::Fixed(v) => *v,
            Value::Multiply(v) => T::from(level) * (*v),
        }
    }
}
impl<T: Default + Reflect + FromReflect> Default for Value<T> {
    fn default() -> Self {
        Self::Fixed(T::default())
    }
}

#[derive(Debug, Clone, Default, Reflect)]
pub struct BurstFire {
    pub f: f32,
    pub i: i32,
}
#[derive(Debug, Clone, Default, Reflect)]
pub struct GhostLight {
    pub f: f32,
    pub i: i32,
}

#[derive(Debug, Clone, Default, Reflect)]
pub struct Diffusion {
    pub percentage: Value<i32>,
    pub radius: f32,
}

#[derive(Debug, Clone, Default, Reflect)]
pub struct FrostBall {
    pub chance: u32,
    pub damage: Value<i32>,
    pub radius: f32,
}

#[derive(Debug, Clone, Default, Reflect)]
pub struct SmashWave {
    pub chance: u32,
    pub damage: Value<i32>,
    pub radius: f32,
}

#[derive(Debug, Clone, Default, Reflect)]
pub struct HealAura {
    pub radius: f32,
    pub percentage: Value<i32>,
}

#[derive(Debug, Clone, Default, Reflect)]
pub struct AttackAura {
    pub radius: f32,
    pub percentage: Value<i32>,
}

#[derive(Debug, Clone, Default, Reflect)]
pub struct SpeedAura {
    pub radius: f32,
    pub percentage: Value<i32>,
}

#[derive(Debug, Clone, Default, Reflect)]
pub struct LifeDrain {
    pub duration: f32,
    pub amount: Value<i32>,
}

#[derive(Debug, Clone, Default, Reflect)]
pub struct Thunder {
    pub duration: f32,
    pub radius: f32,
    pub damage: Value<i32>,
}

#[derive(Debug, Clone, Default, Reflect)]
pub struct DeadFinger {
    pub damage: Value<i32>,
}

#[derive(Debug, Clone, Default, Reflect)]
pub struct FireBreath {
    pub damage: Value<i32>,
}
