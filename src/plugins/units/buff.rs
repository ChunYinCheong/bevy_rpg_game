use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::plugins::visual_effect::{VisualEffect, VisualEffectMarker};

pub struct BuffPlugin;
impl Plugin for BuffPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<Buff>()
            .register_type::<BuffId>()
            .register_type::<CreateBuff>()
            .add_event::<CreateBuff>()
            .add_system(create_buff);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Component, Default, Reflect)]
pub struct Buff {
    pub level: i32,
    pub duration: Option<f32>,
    // pub source: Entity,
}

#[derive(Debug, Clone, Reflect)]
pub enum BuffId {
    AttackUp,
}

#[derive(Debug, Clone, Reflect)]
pub struct CreateBuff {
    pub unit: Entity,
    pub buff_id: BuffId,
    pub buff: Buff,
}
fn create_buff(mut events: EventReader<CreateBuff>, mut commands: Commands) {
    for ev in events.iter() {
        // debug!("ev: {ev:?}");
        let visual_effect_marker = match ev.buff_id {
            BuffId::AttackUp => VisualEffectMarker {
                visual_effect: VisualEffect::DeadFinger,
                pos: Default::default(),
                size: Vec2 { x: 50.0, y: 50.0 },
                repeat: true,
                auto_despawn: false,
                duration: None,
            },
        };

        let id = commands.spawn(visual_effect_marker).id();
        commands
            .entity(id)
            .insert(Name::new(format!("Buff {:?} ({id:?})", ev.buff_id)));

        commands.entity(ev.unit).add_child(id);
    }
}
