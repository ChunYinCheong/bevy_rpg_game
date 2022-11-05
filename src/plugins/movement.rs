use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{plugins::knockback::KNOCKBACK_DURATION, res::GameWorldConfig, RAPIER_SCALE};

use super::{hook::Hooked, knockback::KnockbackVec};

#[derive(Debug, Default, Component, Inspectable)]
pub struct Movement {
    pub direction: Vec2,
    pub speed: f32,
    pub face: Option<Vec2>,
}

pub(crate) fn update_movement(
    time: Res<Time>,
    mut query: Query<(
        &Movement,
        &mut Velocity,
        &Transform,
        Option<&KnockbackVec>,
        Option<&Hooked>,
    )>,
    config: Res<GameWorldConfig>,
) {
    if !config.active {
        return;
    }
    for (movement, mut vel, pos, ks, hooked) in query.iter_mut() {
        let dir = movement.direction.normalize_or_zero();
        if let Some(ks) = ks {
            let k = ks
                .knockbacks
                .iter()
                .map(|k| k.0)
                .reduce(|accum, item| accum + item)
                .unwrap_or_default();
            // info!("k: {k:?}, ks: {ks:?}");
            vel.linvel = (dir * movement.speed + k / KNOCKBACK_DURATION) * RAPIER_SCALE;
        } else {
            vel.linvel = dir * movement.speed * RAPIER_SCALE;
        }
        if let Some(hooked) = hooked {
            if hooked.remain > 0.0 {
                // vel.linvel = (hooked.k / hooked.duration) * RAPIER_SCALE;
                vel.linvel = hooked.k / hooked.duration;
            }
        }
        // let mut impulse = movement.direction; //* movement.speed * 0.02 / 10.0;
        // println!("impulse: {impulse}");
        // impulse.x = impulse.x - vel.linvel.x;
        // impulse.y = impulse.y - vel.linvel.y;
        // println!(
        //     "ext_impulse: {ext_impulse:?}, impulse: {impulse}, vel: {:?}",
        //     vel.linvel
        // );
        // ext_impulse.impulse = impulse;
        // if vel.linvel.length() > 0.0 {
        //     println!("vel.linvel: {}, tran: {}", vel.linvel, pos.translation);
        // }

        // vel.apply_impulse(mass, impulse.into());
        if let Some(dir) = movement.face {
            let forward = crate::utils::get_forward(pos);
            if dir.length_squared() > 0.0 {
                let angle = forward.angle_between(dir);
                let y = angle / time.delta_seconds();
                vel.angvel = y;
            }
        } else if dir.length_squared() > 0.0 {
            let forward = crate::utils::get_forward(pos);
            let angle = forward.angle_between(dir);

            // println!("dir: {dir}, forward:{forward}, angle:{angle:.3}");

            let y = angle / time.delta_seconds();
            // println!("y: {y:.2}, angle: {angle:.2}");
            // rotation speed
            // let cy = y.clamp(-10.0, 10.0);
            // let y = cy;
            vel.angvel = y;
        } else {
            vel.angvel = 0.0;
        }
    }
}
