use bevy::prelude::*;

use crate::res::GameWorldConfig;

use super::hit::HitEvent;

pub struct HookPlugin;

impl Plugin for HookPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // .add_system(hit_detection)
            .register_type::<OnHitHook>()
            .register_type::<Hooked>()
            .add_system(on_hit)
            .add_system(hook)
            // .add_event::<HitEvent>()
            ;
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct OnHitHook {
    pub target: Vec2,
}

#[derive(Debug, Default, Component, Reflect)]
pub struct Hooked {
    pub from: Vec2,
    pub to: Vec2,
    pub duration: f32,
    pub remain: f32,
    pub k: Vec2,
}

pub fn on_hit(
    mut events: EventReader<HitEvent>,

    transfomr_q: Query<(&OnHitHook, &GlobalTransform)>,
    mut commands: Commands,
) {
    for ev in events.iter() {
        // info!("on_hit: {ev:?}");
        if let Ok((hook, tran)) = transfomr_q.get(ev.hit_entity) {
            let dir = hook.target - tran.translation().truncate();
            commands.entity(ev.target_entity).insert(Hooked {
                from: tran.translation().truncate(),
                to: hook.target,
                duration: 0.2,
                remain: 0.2,
                k: dir,
            });
        }
    }
}

pub fn hook(mut knockback_q: Query<(&mut Hooked,)>, time: Res<Time>, config: Res<GameWorldConfig>) {
    if !config.active {
        return;
    }
    let delta = time.delta_seconds();
    for (mut k,) in knockback_q.iter_mut() {
        k.remain -= delta;
    }
}
