use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    animation::{AnimationSheet, AnimationState},
    damage::HitEvent,
    item::{Equipment, Inventory},
    knockback::KnockbackVec,
    movement::Movement,
    save::{
        SaveAnimationState, SaveCollisionGroups, SaveEquipment, SaveGameObjectType, SaveInventory,
        SaveTransform, SaveUnit,
    },
    unit_action::{ActionData, ActionId, UnitAction, UnitActions},
    unit_state::{ActionState, UnitCommand},
};
use crate::{
    plugins::unit_state::ChangeActionRequest, res::GameWorldConfig, RAPIER_SCALE, UNIT_GROUP,
};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_inspectable::<Unit>()
            .register_inspectable::<KillReward>()
            .add_system(unit_update)
            .add_system(on_unit_hit)
            .add_event::<UnitDieEvent>();
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Component, Inspectable)]
pub struct Unit {
    pub hp: i32,
    pub movement_speed: f32,
    // pub actions: Vec<Entity>,
    pub dead: bool,
    // Status Effect
    pub stun: f32,

    // Action
    pub action_id: ActionId,
    pub action_state: ActionState,
    pub action_time: Option<f32>,
    pub action_data: ActionData,
}

#[derive(Debug, Component, Inspectable)]
pub struct KillReward {
    pub exp: i32,
    pub money: i32,
}

pub struct SpawnUnit {
    pub name: &'static str,
    pub unit: Unit,
    pub translation: Vec2,
    pub action_ids: Vec<ActionId>,

    pub texture_path: &'static str,
    pub texture_columns: usize,
    pub texture_rows: usize,
    pub animation_sheet: AnimationSheet,
    pub animation_state: AnimationState,
}

pub fn spawn_unit(
    s: SpawnUnit,
    commands: &mut Commands,
    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let texture_handle = asset_server.load(s.texture_path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(64.0, 64.0),
        s.texture_columns,
        s.texture_rows,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_translation(s.translation.extend(0.0)),
            ..Default::default()
        })
        .insert(Name::new(s.name))
        .insert(s.unit)
        .insert(UnitCommand::default())
        .insert(Movement::default())
        .insert(KnockbackVec::default())
        .insert(UnitActions {
            actions: s
                .action_ids
                .iter()
                .map(|id| UnitAction {
                    action_id: *id,
                    ..Default::default()
                })
                .collect(),
        })
        .insert(Inventory::default())
        .insert(Equipment::default())
        // Save
        .insert(SaveGameObjectType)
        .insert(SaveTransform)
        .insert(SaveUnit)
        .insert(SaveInventory)
        .insert(SaveEquipment)
        .insert(SaveCollisionGroups)
        .insert(SaveAnimationState)
        // Animation
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(s.translation.extend(1.0)),
            ..Default::default()
        })
        .insert(s.animation_sheet)
        .insert(s.animation_state)
        // Rapier
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        // .insert(Velocity {
        //     linvel: Vec2::new(100.0, 20.0),
        //     angvel: 9.4,
        // })
        // .insert(ExternalImpulse::default())
        // .insert(ExternalImpulse {
        //     impulse: Vec2::new(10.0, 0.0),
        //     ..Default::default()
        // })
        // .insert(MassProperties::default())
        // .insert(ColliderMassProperties::Density(2.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::ball(0.5 * RAPIER_SCALE))
        // .insert(Collider::ball(0.5))
        .insert(CollisionGroups::new(UNIT_GROUP, u32::MAX))
        .id()
}

pub fn unit_update(
    mut unit_q: Query<(Entity, &mut Unit)>,
    time: Res<Time>,
    config: Res<GameWorldConfig>,
) {
    if !config.active {
        return;
    }
    let delta = time.delta_seconds();
    for (_, mut unit) in unit_q.iter_mut() {
        if unit.stun > 0.0 {
            unit.stun -= delta;
        }
        if unit.stun < 0.0 {
            unit.stun = 0.0;
        }
        if unit.stun <= 0.0 && unit.action_id == ActionId::Stun {
            unit.action_id = ActionId::Idle;
            unit.action_state = ActionState::Active;
            unit.action_time = None;
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnitDieEvent(pub Entity);
pub fn on_unit_hit(
    mut events: EventReader<HitEvent>,
    mut unit_q: Query<(&mut Unit,)>,
    mut change_events: EventWriter<ChangeActionRequest>,
    mut die_events: EventWriter<UnitDieEvent>,
) {
    for ev in events.iter() {
        if let Ok((mut unit,)) = unit_q.get_mut(ev.victim) {
            info!("Unit hit: {ev:?}");
            unit.hp -= ev.damage;

            let stun = unit.stun.max(ev.hit_stun);
            unit.stun = stun;

            if unit.hp <= 0 {
                unit.dead = true;
                die_events.send(UnitDieEvent(ev.victim));

                change_events.send(ChangeActionRequest {
                    action_id: ActionId::Dead,
                    entity: ev.victim,
                });
            } else {
                change_events.send(ChangeActionRequest {
                    action_id: ActionId::Stun,
                    entity: ev.victim,
                });
            }
        }
    }
}
