use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_rapier2d::{na::UnitComplex, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{
    utils::{self, Knockback, Shape},
    RAPIER_SCALE,
};

use super::{
    animation::ChangeAnimation,
    hook::Hook,
    movement::Movement,
    player::Player,
    unit::Unit,
    unit_state::{
        ActionState, ActionStateEventKind, ChangeActionRequest, UnitCommand, UnitStateEvent,
    },
};

pub struct UnitActionPlugin;

impl Plugin for UnitActionPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_system(state_events)
            // Action
            .add_system(fire_ghost_light);
    }
}

#[derive(Debug, Default, Component)]
pub struct UnitActions {
    pub actions: Vec<UnitAction>,
}

#[derive(Debug, Default)]
pub struct UnitAction {
    pub action_id: ActionId,
    pub recharge: i32,
    pub charged: i32,
}

#[derive(Debug, Default, Clone, Component, Inspectable)]
pub struct ActionSetting {
    pub startup_time: Option<f32>,
    pub active_time: Option<f32>,
    pub recover_time: Option<f32>,
    pub action_state: ActionState,
    pub action_id: ActionId,
    pub action_data: ActionData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Inspectable)]
pub enum ActionId {
    Idle,
    Stun,
    Dead,
    Walk,
    Run,
    Attack,
    ForbiddenArray,
    IceSpear,
    Stab,
    BurstFire,
    Hook,
    Fireball,
    Explosion,
    Burning,
    Drone,
    GhostLight,
    SpiderAttack,
    WolfAttack,
}
impl Default for ActionId {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Inspectable)]
pub enum ActionData {
    None,
    Repeat(f32, i32),
}
impl Default for ActionData {
    fn default() -> Self {
        Self::None
    }
}

impl ActionId {
    pub fn setting(&self) -> ActionSetting {
        match self {
            ActionId::Idle => ActionSetting {
                startup_time: Some(0.1),
                active_time: None,
                recover_time: Some(0.2),
                action_id: ActionId::Idle,
                action_state: ActionState::Startup,
                action_data: ActionData::None,
            },
            ActionId::Stun => ActionSetting {
                startup_time: None,
                active_time: None,
                recover_time: None,
                action_id: ActionId::Stun,
                action_state: ActionState::Active,
                action_data: ActionData::None,
            },
            ActionId::Dead => ActionSetting {
                startup_time: None,
                active_time: None,
                recover_time: None,
                action_id: ActionId::Dead,
                action_state: ActionState::Active,
                action_data: ActionData::None,
            },
            ActionId::Walk => ActionSetting {
                startup_time: Some(0.1),
                active_time: None,
                recover_time: Some(0.2),
                action_id: ActionId::Walk,
                action_state: ActionState::Startup,
                action_data: ActionData::None,
            },
            ActionId::Run => ActionSetting {
                startup_time: Some(0.1),
                active_time: None,
                recover_time: Some(0.2),
                action_id: ActionId::Run,
                action_state: ActionState::Startup,
                action_data: ActionData::None,
            },
            ActionId::Attack => ActionSetting {
                startup_time: Some(0.1),
                active_time: Some(0.05),
                recover_time: Some(0.2),
                action_id: ActionId::Attack,
                action_state: ActionState::Startup,
                action_data: ActionData::None,
            },
            ActionId::ForbiddenArray => ActionSetting {
                startup_time: Some(0.1),
                active_time: Some(0.05),
                recover_time: Some(0.2),
                action_id: ActionId::ForbiddenArray,
                action_state: ActionState::Startup,
                action_data: ActionData::None,
            },
            ActionId::IceSpear => ActionSetting {
                startup_time: Some(0.1),
                active_time: Some(0.05),
                recover_time: Some(0.2),
                action_id: ActionId::IceSpear,
                action_state: ActionState::Startup,
                action_data: ActionData::None,
            },
            ActionId::Stab => ActionSetting {
                startup_time: Some(0.1),
                active_time: Some(0.05),
                recover_time: Some(0.2),
                action_id: ActionId::Stab,
                action_state: ActionState::Startup,
                action_data: ActionData::None,
            },
            ActionId::BurstFire => ActionSetting {
                startup_time: Some(0.1),
                active_time: Some(0.15),
                recover_time: Some(0.0),
                action_id: ActionId::BurstFire,
                action_state: ActionState::Startup,
                action_data: ActionData::Repeat(0.15, 0),
            },
            ActionId::Hook => ActionSetting {
                startup_time: Some(0.2),
                active_time: Some(0.2),
                recover_time: Some(0.0),
                action_id: ActionId::Hook,
                action_state: ActionState::Startup,
                action_data: ActionData::None,
            },
            ActionId::Fireball => ActionSetting {
                startup_time: Some(0.1),
                active_time: Some(0.05),
                recover_time: Some(0.1),
                action_id: ActionId::Fireball,
                action_state: ActionState::Startup,
                action_data: ActionData::None,
            },
            ActionId::Explosion => ActionSetting {
                startup_time: Some(0.1),
                active_time: Some(0.05),
                recover_time: Some(0.1),
                action_id: ActionId::Explosion,
                action_state: ActionState::Startup,
                action_data: ActionData::None,
            },
            ActionId::Burning => ActionSetting {
                startup_time: Some(0.1),
                active_time: Some(0.05),
                recover_time: Some(0.1),
                action_id: ActionId::Burning,
                action_state: ActionState::Startup,
                action_data: ActionData::None,
            },
            ActionId::Drone => ActionSetting {
                startup_time: Some(0.1),
                active_time: Some(0.05),
                recover_time: Some(0.1),
                action_id: ActionId::Drone,
                action_state: ActionState::Startup,
                action_data: ActionData::None,
            },
            ActionId::GhostLight => ActionSetting {
                startup_time: Some(0.1),
                active_time: None,
                recover_time: Some(1.0),
                action_id: ActionId::GhostLight,
                action_state: ActionState::Startup,
                action_data: ActionData::Repeat(1.0, 0),
            },
            ActionId::SpiderAttack => ActionSetting {
                startup_time: Some(0.5),
                active_time: Some(0.05),
                recover_time: Some(0.5),
                action_id: ActionId::SpiderAttack,
                action_state: ActionState::Startup,
                action_data: ActionData::None,
            },
            ActionId::WolfAttack => ActionSetting {
                startup_time: Some(0.5),
                active_time: Some(0.5),
                recover_time: Some(0.5),
                action_id: ActionId::WolfAttack,
                action_state: ActionState::Startup,
                action_data: ActionData::None,
            },
        }
    }
}

#[derive(Debug, strum::Display)]
pub enum UnitAnimation {
    Idle,
    Dead,
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

pub fn state_events(
    mut events: EventReader<UnitStateEvent>,
    mut change_events: EventWriter<ChangeActionRequest>,
    mut unit_q: Query<(
        Entity,
        &mut Unit,
        &mut Movement,
        &UnitCommand,
        &Transform,
        &mut CollisionGroups,
    )>,

    mut anim_events: EventWriter<ChangeAnimation>,

    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
) {
    for ev in events.iter() {
        let id = ev.id;
        let delta = time.delta_seconds();
        if let Ok((entity, mut unit, mut movement, command, position, mut flag)) =
            unit_q.get_mut(id)
        {
            match ev.kind {
                ActionStateEventKind::Enter => match ev.action {
                    ActionId::Idle => {
                        movement.speed = 0.0;
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::Idle.to_string(),
                        });
                    }
                    ActionId::Walk => {
                        movement.speed = unit.movement_speed;
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::Walk.to_string(),
                        });
                    }
                    ActionId::Run => {
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::Run.to_string(),
                        });
                    }
                    ActionId::Attack => {
                        movement.speed = 0.0;
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::Attack.to_string(),
                        });
                    }
                    ActionId::ForbiddenArray => {
                        movement.speed = 0.0;
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::Attack.to_string(),
                        });
                    }
                    ActionId::IceSpear => {
                        movement.speed = 0.0;
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::BurstFire.to_string(),
                        });
                    }
                    ActionId::Stab => {
                        movement.speed = 0.0;
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::Stab.to_string(),
                        });
                    }
                    ActionId::Stun => {
                        movement.speed = 0.0;
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::Stun.to_string(),
                        });
                    }
                    ActionId::Dead => {
                        movement.speed = 0.0;
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::Dead.to_string(),
                        });

                        flag.memberships = 0b0;
                        flag.filters = 0b0;
                    }
                    ActionId::BurstFire => {
                        movement.speed = 0.0;
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::BurstFire.to_string(),
                        });
                    }
                    ActionId::Hook => {
                        movement.speed = 0.0;
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::Hook.to_string(),
                        });
                    }
                    ActionId::Fireball => {
                        movement.speed = 0.0;
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::Fireball.to_string(),
                        });
                    }
                    ActionId::Explosion => {
                        movement.speed = 0.0;
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::Explosion.to_string(),
                        });
                    }
                    ActionId::Burning => {
                        movement.speed = 0.0;
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::Burning.to_string(),
                        });
                    }
                    ActionId::Drone => {
                        movement.speed = 0.0;
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::Drone.to_string(),
                        });
                    }
                    ActionId::GhostLight => {
                        movement.speed = 0.0;
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::GhostLight.to_string(),
                        });
                    }
                    ActionId::SpiderAttack => {
                        movement.speed = 0.0;
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::Attack.to_string(),
                        });

                        movement.face = command.target_direction;
                    }
                    ActionId::WolfAttack => {
                        movement.speed = 0.0;
                        anim_events.send(ChangeAnimation {
                            entity,
                            name: UnitAnimation::Attack.to_string(),
                        });

                        movement.face = command.target_direction;
                    }
                },
                ActionStateEventKind::EnterActive => match ev.action {
                    ActionId::Idle => (),
                    ActionId::Walk => (),
                    ActionId::Run => (),
                    ActionId::Attack => {
                        spawn_attack(
                            &mut commands,
                            *position,
                            id,
                            &mut asset_server,
                            &mut texture_atlases,
                        );
                    }
                    ActionId::ForbiddenArray => {
                        forbidden_array(
                            &mut commands,
                            *position,
                            id,
                            &mut asset_server,
                            &mut texture_atlases,
                        );
                    }
                    ActionId::IceSpear => {
                        ice_spear(
                            &mut commands,
                            *position,
                            id,
                            &mut asset_server,
                            &mut texture_atlases,
                        );
                    }
                    ActionId::Stab => {
                        movement.speed = unit.movement_speed;
                        movement.direction = utils::get_forward(position);
                        stab(
                            &mut commands,
                            *position,
                            id,
                            &mut asset_server,
                            &mut texture_atlases,
                        );
                    }
                    ActionId::Stun => {
                        movement.speed = 0.0;
                    }
                    ActionId::Dead => {
                        movement.speed = 0.0;
                    }
                    ActionId::BurstFire => {
                        burst_fire(
                            &mut commands,
                            *position,
                            id,
                            &mut asset_server,
                            &mut texture_atlases,
                        );
                    }
                    ActionId::Hook => {
                        hook(
                            &mut commands,
                            *position,
                            id,
                            &mut asset_server,
                            &mut texture_atlases,
                        );
                    }
                    ActionId::Fireball => (),
                    ActionId::Explosion => (),
                    ActionId::Burning => {
                        let animation_entity = {
                            let texture_handle = asset_server.load("images/fox/unknown.png");
                            let texture_atlas = TextureAtlas::from_grid(
                                texture_handle,
                                Vec2::new(128.0, 128.0),
                                1,
                                1,
                            );
                            let texture_atlas_handle = texture_atlases.add(texture_atlas);
                            commands
                                .spawn_bundle(SpriteSheetBundle {
                                    texture_atlas: texture_atlas_handle,
                                    transform: Transform {
                                        translation: Vec3::new(0.0, 0.0, 1.0),
                                        ..Default::default()
                                    },
                                    sprite: TextureAtlasSprite {
                                        index: 0,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .id()
                        };
                        let id = crate::utils::spawn_melee(
                            crate::utils::Melee {
                                offset: Vec2::new(0.0, 0.0),
                                lifespan: 0.5,
                                source: id,
                                shape: Shape::Ball(0.8),
                                parent_position: *position,
                                damage: 1,
                                hit_stun: 0.3,
                                knockback: Knockback::Center(0.1),
                            },
                            &mut commands,
                        );
                        commands.entity(id).add_child(animation_entity);

                        // id
                    }
                    ActionId::Drone => (),
                    ActionId::GhostLight => {}
                    ActionId::SpiderAttack => {
                        spider_attack(
                            &mut commands,
                            *position,
                            id,
                            &mut asset_server,
                            &mut texture_atlases,
                        );
                    }
                    ActionId::WolfAttack => {
                        movement.speed = unit.movement_speed * 3.0;
                        movement.direction = utils::get_forward(position);
                        wolf_attack(
                            &mut commands,
                            *position,
                            id,
                            &mut asset_server,
                            &mut texture_atlases,
                        );
                    }
                },
                ActionStateEventKind::UpdateActive => {
                    match unit.action_id {
                        ActionId::Idle => match command.action_id {
                            ActionId::Idle => {
                                if let Some(pos) = command.target_position {
                                    movement.face = Some(pos - position.translation.truncate());
                                }
                            }
                            ActionId::Stun => (),
                            ActionId::Dead => (),
                            ActionId::Run => (),
                            ActionId::Walk => {
                                movement.face = command.target_direction;
                                movement.direction = command.movement_direction;
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Walk,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Attack => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Attack,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::ForbiddenArray => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::ForbiddenArray,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::IceSpear => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::IceSpear,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Stab => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Stab,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::BurstFire => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::BurstFire,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Hook => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Hook,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::GhostLight => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::GhostLight,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Fireball => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Fireball,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Explosion => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Explosion,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Burning => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Burning,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Drone => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Drone,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::SpiderAttack => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::SpiderAttack,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::WolfAttack => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::WolfAttack,
                                    entity: id,
                                });
                                continue;
                            }
                        },
                        ActionId::Walk => match command.action_id {
                            ActionId::Idle => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Idle,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Stun => (),
                            ActionId::Dead => (),
                            ActionId::Run => (),
                            ActionId::Walk => {
                                movement.face = command.target_direction;
                                movement.direction = command.movement_direction;
                                movement.speed = unit.movement_speed;
                                if let Some(pos) = command.target_position {
                                    movement.face = Some(pos - position.translation.truncate());
                                }
                            }
                            ActionId::Attack => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Attack,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::ForbiddenArray => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::ForbiddenArray,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::IceSpear => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::IceSpear,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Stab => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Stab,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::BurstFire => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::BurstFire,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Hook => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Hook,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::GhostLight => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::GhostLight,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Fireball => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Fireball,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Explosion => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Explosion,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Burning => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Burning,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Drone => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Drone,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::SpiderAttack => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::SpiderAttack,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::WolfAttack => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::WolfAttack,
                                    entity: id,
                                });
                                continue;
                            }
                        },
                        ActionId::Run => match command.action_id {
                            ActionId::Idle => {
                                movement.speed = 0.0;
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Idle,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Stun => {}
                            ActionId::Dead => (),
                            ActionId::Run => (),
                            ActionId::Walk => {
                                movement.direction = command.movement_direction;
                                movement.speed = unit.movement_speed;
                            }
                            ActionId::Attack => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Attack,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::ForbiddenArray => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::ForbiddenArray,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::IceSpear => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::IceSpear,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Stab => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Stab,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::BurstFire => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::BurstFire,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Hook => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Hook,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::GhostLight => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::GhostLight,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Fireball => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Fireball,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Explosion => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Explosion,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Burning => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Burning,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::Drone => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::Drone,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::SpiderAttack => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::SpiderAttack,
                                    entity: id,
                                });
                                continue;
                            }
                            ActionId::WolfAttack => {
                                change_events.send(ChangeActionRequest {
                                    action_id: ActionId::WolfAttack,
                                    entity: id,
                                });
                                continue;
                            }
                        },
                        ActionId::Attack => {}
                        ActionId::ForbiddenArray => {}
                        ActionId::IceSpear => {}
                        ActionId::Stab => {}
                        ActionId::BurstFire => {
                            if let ActionData::Repeat(ref mut t, ref mut i) = unit.action_data {
                                *t -= delta;
                                if *t <= 0.1 && *i == 0 {
                                    *i = 1;
                                    let position = *position;
                                    burst_fire(
                                        &mut commands,
                                        position,
                                        id,
                                        &mut asset_server,
                                        &mut texture_atlases,
                                    );
                                }
                                if *t <= 0.05 && *i == 1 {
                                    *i = 2;
                                    burst_fire(
                                        &mut commands,
                                        *position,
                                        id,
                                        &mut asset_server,
                                        &mut texture_atlases,
                                    );
                                }
                            }
                        }
                        ActionId::Hook => {
                            // hook_update();
                        }
                        ActionId::Fireball => {}
                        ActionId::Explosion => {}
                        ActionId::Burning => {}
                        ActionId::Drone => {}
                        ActionId::GhostLight => {
                            if let ActionData::Repeat(ref mut t, ref mut i) = unit.action_data {
                                *t -= delta;
                                if *t <= 0.8 && *i == 0 {
                                    *i = 1;
                                    let position = *position;
                                    spawn_ghost(
                                        &mut commands,
                                        &mut asset_server,
                                        &mut texture_atlases,
                                        id,
                                        position,
                                        0.0,
                                    );
                                }
                                if *t <= 0.6 && *i == 1 {
                                    *i = 2;
                                    spawn_ghost(
                                        &mut commands,
                                        &mut asset_server,
                                        &mut texture_atlases,
                                        id,
                                        *position,
                                        std::f32::consts::PI / 4.0,
                                    );
                                    spawn_ghost(
                                        &mut commands,
                                        &mut asset_server,
                                        &mut texture_atlases,
                                        id,
                                        *position,
                                        std::f32::consts::PI * 7.0 / 4.0,
                                    );
                                }
                                if *t <= 0.4 && *i == 2 {
                                    *i = 3;
                                    spawn_ghost(
                                        &mut commands,
                                        &mut asset_server,
                                        &mut texture_atlases,
                                        id,
                                        *position,
                                        std::f32::consts::PI / 2.0,
                                    );
                                    spawn_ghost(
                                        &mut commands,
                                        &mut asset_server,
                                        &mut texture_atlases,
                                        id,
                                        *position,
                                        std::f32::consts::PI * 3.0 / 2.0,
                                    );
                                }
                                if *t <= 0.2 && *i == 3 {
                                    *i = 4;
                                    spawn_ghost(
                                        &mut commands,
                                        &mut asset_server,
                                        &mut texture_atlases,
                                        id,
                                        *position,
                                        std::f32::consts::PI * 3.0 / 4.0,
                                    );
                                    spawn_ghost(
                                        &mut commands,
                                        &mut asset_server,
                                        &mut texture_atlases,
                                        id,
                                        *position,
                                        std::f32::consts::PI * 5.0 / 4.0,
                                    );
                                }
                                if *t <= 0.0 {
                                    spawn_ghost(
                                        &mut commands,
                                        &mut asset_server,
                                        &mut texture_atlases,
                                        id,
                                        *position,
                                        std::f32::consts::PI,
                                    );

                                    change_events.send(ChangeActionRequest {
                                        action_id: ActionId::Idle,
                                        entity: id,
                                    });
                                    continue;
                                }
                            }
                        }
                        ActionId::Stun => {}
                        ActionId::Dead => (),
                        ActionId::SpiderAttack => {}
                        ActionId::WolfAttack => {}
                    }
                    // None
                }
                ActionStateEventKind::ExitActive => (),
                ActionStateEventKind::Exit => (),
            }
        }
    }
}

#[derive(Debug, Component)]
pub struct GhostLight {
    timer: Timer,
}

pub fn spawn_ghost(
    commands: &mut Commands,
    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    spider: Entity,
    mut position: Transform,
    angle: f32,
) -> Entity {
    let animation_entity = {
        let texture_handle = asset_server.load("images/spider/spritesheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 5, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    index: 4,
                    ..Default::default()
                },
                ..Default::default()
            })
            .id()
    };
    let forward = utils::get_forward(&position);

    let rot = UnitComplex::new(angle);
    let offset = rot.transform_vector(&forward.into());
    let offset = offset * 1.0 * RAPIER_SCALE;
    position.translation.x += offset.x;
    position.translation.y += offset.y;

    let id = utils::spawn_projectile(
        utils::Projectile {
            position,
            lifespan: 5.0,
            linvel: Vec2::ZERO,
            source: spider,
            shape: Shape::Ball(0.25),
            damage: 1,
            hit_stun: 0.0,
            knockback: Knockback::Center(0.1),
        },
        commands,
    );
    commands
        .entity(id)
        .insert(GhostLight {
            timer: Timer::from_seconds(2.0, false),
        })
        .add_child(animation_entity);
    id
}

fn fire_ghost_light(
    mut q: Query<(&mut GhostLight, &mut Velocity, &Transform), (With<GhostLight>, Without<Player>)>,
    player_q: Query<&Transform, (With<Player>, Without<GhostLight>)>,
    time: Res<Time>,
) {
    for (mut ghost, mut vel, pos) in q.iter_mut() {
        ghost.timer.tick(time.delta());
        if ghost.timer.just_finished() {
            if let Ok(player_pos) = player_q.get_single() {
                let dir = player_pos.translation.truncate() - pos.translation.truncate();
                let v = dir.normalize() * 5.0 * RAPIER_SCALE;
                vel.linvel = v;
            }
        }
    }
}

pub fn spawn_attack(
    commands: &mut Commands,
    position: Transform,
    player: Entity,
    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let animation_entity = {
        let texture_handle = asset_server.load("images/player/spritesheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 10, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    index: 9,
                    ..Default::default()
                },
                ..Default::default()
            })
            .id()
    };

    let id = crate::utils::spawn_melee(
        crate::utils::Melee {
            offset: Vec2::new(1.0, 0.0),
            lifespan: 0.1,
            source: player,
            shape: Shape::Cuboid(0.5, 0.5),
            parent_position: position,
            damage: 1,
            hit_stun: 0.3,
            knockback: Knockback::Direction(0.2, 0.0),
        },
        commands,
    );
    commands.entity(id).add_child(animation_entity);
    id
}

pub fn stab(
    commands: &mut Commands,
    position: Transform,
    player: Entity,
    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let animation_entity = {
        let texture_handle = asset_server.load("images/player/spritesheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 10, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    index: 9,
                    ..Default::default()
                },
                ..Default::default()
            })
            .id()
    };

    let id = crate::utils::spawn_melee(
        crate::utils::Melee {
            offset: Vec2::new(1.5, 0.0),
            lifespan: 0.1,
            source: player,
            shape: Shape::Cuboid(1.0, 0.3),
            parent_position: position,
            damage: 1,
            hit_stun: 0.3,
            knockback: Knockback::Direction(0.2, 0.0),
        },
        commands,
    );
    commands.entity(id).add_child(animation_entity);
    id
}

pub fn forbidden_array(
    commands: &mut Commands,
    position: Transform,
    player: Entity,
    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    for i in 0..72 {
        let animation_entity = {
            let texture_handle = asset_server.load("images/player/spritesheet.png");
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 10, 1);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 1.0),
                        ..Default::default()
                    },
                    sprite: TextureAtlasSprite {
                        index: 9,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .id()
        };
        let mut position = position;
        let angle = std::f32::consts::PI * 2.0 / 72.0 * i as f32;
        let rotation_change = Quat::from_rotation_z(angle);
        position.rotate(rotation_change);
        let forward = utils::get_forward(&position);
        let linvel = forward * 20.0 * RAPIER_SCALE;
        let id = utils::spawn_projectile(
            utils::Projectile {
                position,
                lifespan: 1.0,
                linvel,
                source: player,
                shape: Shape::Cuboid(0.2, 0.1),
                damage: 1,
                hit_stun: 0.3,
                knockback: Knockback::Center(0.1),
            },
            commands,
        );
        commands.entity(id).add_child(animation_entity);
    }
}

pub fn ice_spear(
    commands: &mut Commands,
    mut position: Transform,
    player: Entity,
    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let animation_entity = {
        let texture_handle = asset_server.load("images/player/spritesheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 10, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    index: 9,
                    ..Default::default()
                },
                ..Default::default()
            })
            .id()
    };
    let forward = utils::get_forward(&position);
    let offset = forward * 1.0 * RAPIER_SCALE;
    let linvel = forward * 10.0 * RAPIER_SCALE;
    position.translation.x += offset.x;
    position.translation.y += offset.y;

    let id = utils::spawn_projectile(
        utils::Projectile {
            position,
            lifespan: 3.0,
            linvel,
            source: player,
            shape: Shape::Ball(0.25),
            damage: 1,
            hit_stun: 0.3,
            knockback: Knockback::Center(1.0),
        },
        commands,
    );
    commands.entity(id).add_child(animation_entity);
}

pub fn burst_fire(
    commands: &mut Commands,
    mut position: Transform,
    player: Entity,
    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let animation_entity = {
        let texture_handle = asset_server.load("images/player/spritesheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 10, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    index: 9,
                    ..Default::default()
                },
                ..Default::default()
            })
            .id()
    };
    let forward = utils::get_forward(&position);
    let offset = forward * 1.0 * RAPIER_SCALE;
    let linvel = forward * 10.0 * RAPIER_SCALE;
    position.translation.x += offset.x;
    position.translation.y += offset.y;

    let id = utils::spawn_projectile(
        utils::Projectile {
            position,
            lifespan: 3.0,
            linvel,
            source: player,
            shape: Shape::Ball(0.25),
            damage: 1,
            hit_stun: 0.3,
            knockback: Knockback::Center(0.0),
        },
        commands,
    );
    commands.entity(id).add_child(animation_entity);
}

pub fn hook(
    commands: &mut Commands,
    mut position: Transform,
    player: Entity,
    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let animation_entity = {
        let texture_handle = asset_server.load("images/player/spritesheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 10, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    index: 9,
                    ..Default::default()
                },
                ..Default::default()
            })
            .id()
    };
    let forward = utils::get_forward(&position);
    let offset = forward * 1.0 * RAPIER_SCALE;
    let linvel = forward * 10.0 * RAPIER_SCALE;
    position.translation.x += offset.x;
    position.translation.y += offset.y;

    let id = utils::spawn_projectile(
        utils::Projectile {
            position,
            lifespan: 1.0,
            linvel,
            source: player,
            shape: Shape::Ball(0.25),
            damage: 1,
            hit_stun: 0.3,
            knockback: Knockback::Center(0.0),
        },
        commands,
    );
    commands
        .entity(id)
        .add_child(animation_entity)
        .insert(Hook {
            target: position.translation.truncate(),
        });
}

pub fn spider_attack(
    commands: &mut Commands,
    mut position: Transform,
    spider: Entity,
    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let animation_entity = {
        let texture_handle = asset_server.load("images/spider/spritesheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 5, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    index: 4,
                    ..Default::default()
                },
                ..Default::default()
            })
            .id()
    };
    let forward = utils::get_forward(&position);
    let offset = forward * 1.0 * RAPIER_SCALE;
    let linvel = forward * 10.0 * RAPIER_SCALE;
    position.translation.x += offset.x;
    position.translation.y += offset.y;

    let id = utils::spawn_projectile(
        utils::Projectile {
            position,
            lifespan: 5.0,
            linvel,
            source: spider,
            shape: Shape::Ball(0.25),
            damage: 1,
            hit_stun: 0.3,
            knockback: Knockback::Center(0.1),
        },
        commands,
    );
    commands.entity(id).add_child(animation_entity);
    id
}

pub fn wolf_attack(
    commands: &mut Commands,
    position: Transform,
    wolf: Entity,
    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let animation_entity = {
        let texture_handle = asset_server.load("images/player/spritesheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 5, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    index: 1,
                    ..Default::default()
                },
                ..Default::default()
            })
            .id()
    };
    let id = crate::utils::spawn_melee(
        crate::utils::Melee {
            offset: Vec2::new(0.1, 0.0),
            lifespan: 0.5,
            source: wolf,
            shape: Shape::Ball(0.5),
            parent_position: position,
            damage: 1,
            hit_stun: 0.3,
            knockback: Knockback::Center(0.1),
        },
        commands,
    );
    commands.entity(id).add_child(animation_entity);

    id
}
