use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    plugins::{
        actions::{action::Skill, skill_id::SkillId},
        animation::{AnimationSheet, AnimationState},
        damage::HitDamageEvent,
        item::{Equipment, Inventory},
        knockback::KnockbackVec,
        movement::Movement,
        save::{
            SaveAnimationState, SaveCollisionGroups, SaveEquipment, SaveGameObjectType,
            SaveInventory, SaveTransform, SaveUnit,
        },
        team::Team,
        unit_action::UnitActions,
        unit_state::{ActionState, ChangeActionRequest},
        units::unit_command::UnitCommand,
    },
    res::GameWorldConfig,
    ALL_GROUP, RAPIER_SCALE, UNIT_GROUP,
};

use super::{unit_command::UnitActionPlanner, unit_state::UnitState};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<Unit>()
            .register_type::<KillReward>()
            .add_system(unit_update)
            .add_system(on_unit_hit)
            .add_event::<UnitDieEvent>()
            // Attack Attribute
            .add_event::<AttackModifierEvent>()
            .add_system(attack_modifier)
            // Hp
            .add_system(super::hp_text::attach_text)
            .add_system(super::hp_text::update_hp_text)
            .add_system_to_stage(CoreStage::Last, super::hp_text::fix_rotation)
            // .add_system(super::hp_text::update_remote_transform)
            // .add_system(super::hp_text::clear_text)
            ;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Component, Reflect, Default)]
pub struct Unit {
    pub hp: i32,
    pub hp_max: i32,
    pub movement_speed: f32,
    pub atk: i32,
    // pub actions: Vec<Entity>,
    pub dead: bool,
    // Status Effect
    pub stun: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct AttributeModifier {
    pub source: Entity,
    pub amount: i32,
    pub percentage: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Component, Default)]
pub struct AttackAttribute {
    pub base: i32,
    pub modifiers: Vec<AttributeModifier>,
}

impl AttackAttribute {
    fn get_attr(&self) -> i32 {
        let (amount, percentage) = self
            .modifiers
            .iter()
            .fold((0, 0), |acc, x| (acc.0 + x.amount, acc.1 + x.percentage));
        self.base + amount + self.base * percentage / 100
    }
}

#[derive(Debug, Clone)]
pub enum AttackModifierEvent {
    Add(Entity, AttributeModifier),
    Remove(Entity, Entity),
}
fn attack_modifier(
    mut events: EventReader<AttackModifierEvent>,
    mut query: Query<(&mut AttackAttribute, &mut Unit)>,
) {
    for ev in events.iter() {
        match ev {
            AttackModifierEvent::Add(unit, modifier) => {
                if let Ok((mut attr, mut unit)) = query.get_mut(*unit) {
                    attr.modifiers.retain(|m| m.source != modifier.source);
                    attr.modifiers.push(modifier.clone());
                    unit.atk = attr.get_attr();
                }
            }
            AttackModifierEvent::Remove(unit, entity) => {
                if let Ok((mut attr, mut unit)) = query.get_mut(*unit) {
                    attr.modifiers.retain(|m| &m.source != entity);
                    unit.atk = attr.get_attr();
                }
            }
        }
    }
}

#[derive(Debug, Component, Reflect)]
pub struct KillReward {
    pub exp: i32,
    pub money: i32,
}

pub struct SpawnUnit {
    pub name: &'static str,
    pub unit: Unit,
    pub translation: Vec2,
    pub action_ids: Vec<SkillId>,
    pub team: Team,

    pub texture_path: &'static str,
    pub texture_columns: usize,
    pub texture_rows: usize,
    pub animation_sheet: AnimationSheet,
    pub animation_state: AnimationState,
}

pub fn spawn_unit(
    s: SpawnUnit,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let texture_handle = asset_server.load(s.texture_path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(64.0, 64.0),
        s.texture_columns,
        s.texture_rows,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let actions: Vec<Entity> = s
        .action_ids
        .iter()
        .map(|id| {
            let entity = commands
                .spawn(Skill {
                    action_id: *id,
                    level: 1,
                    ..Default::default()
                })
                .id();

            commands
                .entity(entity)
                .insert(Name::new(format!("Skill {id:?} ({entity:?})")));
            entity
        })
        .collect();

    commands
        .spawn(SpatialBundle {
            transform: Transform::from_translation(s.translation.extend(0.0)),
            ..Default::default()
        })
        .insert(Name::new(s.name))
        .insert(AttackAttribute {
            base: s.unit.atk,
            ..Default::default()
        })
        .insert(s.unit)
        .insert(UnitState {
            action_id: SkillId::Idle,
            action_entity: None,
            action_state: ActionState::Active,
            action_time: None,
            command: None,
        })
        .insert(UnitCommand::default())
        .insert(UnitActionPlanner::default())
        .insert(Movement::default())
        .insert(KnockbackVec::default())
        .push_children(&actions)
        .insert(UnitActions { actions })
        .insert(Inventory::default())
        .insert(Equipment::default())
        .insert(s.team)
        // Save
        .insert(SaveGameObjectType)
        .insert(SaveTransform)
        .insert(SaveUnit)
        .insert(SaveInventory)
        .insert(SaveEquipment)
        .insert(SaveCollisionGroups)
        .insert(SaveAnimationState)
        // Animation
        .insert(SpriteSheetBundle {
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
        .insert(CollisionGroups::new(UNIT_GROUP, ALL_GROUP))
        .id()
}

pub fn unit_update(
    mut unit_q: Query<(Entity, &mut Unit, &mut UnitState)>,
    time: Res<Time>,
    config: Res<GameWorldConfig>,
) {
    if !config.active {
        return;
    }
    let delta = time.delta_seconds();
    for (_, mut unit, mut unit_state) in unit_q.iter_mut() {
        if unit.stun > 0.0 {
            unit.stun -= delta;
        }
        if unit.stun < 0.0 {
            unit.stun = 0.0;
        }
        if unit.stun <= 0.0 && unit_state.action_id == SkillId::Stun {
            unit_state.action_id = SkillId::Idle;
            unit_state.action_state = ActionState::Active;
            unit_state.action_time = None;
            unit_state.action_entity = None;
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct UnitDieEvent(pub Entity);
impl Default for UnitDieEvent {
    fn default() -> Self {
        Self(Entity::from_raw(0))
    }
}

pub fn on_unit_hit(
    mut events: EventReader<HitDamageEvent>,
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
                if !unit.dead {
                    die_events.send(UnitDieEvent(ev.victim));

                    unit.dead = true;

                    change_events.send(ChangeActionRequest {
                        action_id: SkillId::Dead,
                        command: default(),
                        entity: ev.victim,
                    });
                }
            } else if unit.stun > 0.0 {
                change_events.send(ChangeActionRequest {
                    action_id: SkillId::Stun,
                    command: default(),
                    entity: ev.victim,
                });
            }
        }
    }
}
