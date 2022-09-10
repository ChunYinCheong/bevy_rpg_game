use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::{
    res::GameWorldConfig,
    utils::{self, Knockback},
};

use super::damage::HitEvent;

pub struct KnockbackPlugin;

impl Plugin for KnockbackPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // .add_system(hit_detection)
            .register_inspectable::<KnockbackVec>()
            .add_system(on_hit)
            .add_system(knockback)
            // .add_event::<HitEvent>()
            ;
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct KnockbackVec {
    pub knockbacks: Vec<(Vec2, f32)>,
}

pub const KNOCKBACK_DURATION: f32 = 0.1;

pub fn on_hit(
    mut events: EventReader<HitEvent>,

    transfomr_q: Query<(&GlobalTransform,)>,
    mut unit_q: Query<(&mut KnockbackVec,)>,
) {
    for ev in events.iter() {
        info!("on_hit: {ev:?}");

        if let Ok((mut unit,)) = unit_q.get_mut(ev.victim) {
            match ev.knockback {
                Knockback::Center(f) => {
                    let (hit_tran,) = transfomr_q.get(ev.source_collider).unwrap();
                    let (tran,) = transfomr_q.get(ev.victim).unwrap();
                    let dir = tran.translation() - hit_tran.translation();
                    let k = dir.truncate().normalize() * f;
                    unit.knockbacks.push((k, KNOCKBACK_DURATION));
                }
                Knockback::Direction(x, _y) => {
                    let (hit_tran,) = transfomr_q.get(ev.source_collider).unwrap();
                    let forward = utils::get_forward_global(hit_tran);
                    let k = forward * x;
                    unit.knockbacks.push((k, KNOCKBACK_DURATION));
                }
            }
        }
    }
}

pub fn knockback(
    mut knockback_q: Query<(&mut KnockbackVec,)>,
    time: Res<Time>,
    config: Res<GameWorldConfig>,
) {
    if !config.active {
        return;
    }
    let delta = time.delta_seconds();
    for (mut k,) in knockback_q.iter_mut() {
        k.knockbacks.retain_mut(|k| {
            k.1 -= delta;
            k.1 > 0.0
        });
    }
}
