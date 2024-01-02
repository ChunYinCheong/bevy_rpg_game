use std::{fmt::Debug, marker::PhantomData};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    area::{Area, PlayerEnterEvent},
    blocker::Blocker,
    game_world::GameObjectId,
    save::SaveBuffer,
    unit::UnitDieEvent,
};

pub struct TriggerPlugin;
impl Plugin for TriggerPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_event::<TriggerAction>()
            .add_system(action)
            // .register_type::<EventTrigger<UnitDieEvent>>()
            .add_system(event_action::<UnitDieEvent>)
            // .register_type::<EventTrigger<PlayerEnterEvent>>()
            .add_system(event_action::<PlayerEnterEvent>)
            // ...
            ;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect_value()]
pub enum TriggerAction {
    None,
    ShowBlocker(GameObjectId),
    HideBlocker(GameObjectId),
    DisableArea(GameObjectId),
}
impl TriggerAction {
    pub(crate) fn new(action: &str, entity_iid: String) -> TriggerAction {
        match action {
            "ShowBlocker" => TriggerAction::ShowBlocker(GameObjectId(entity_iid)),
            "HideBlocker" => TriggerAction::HideBlocker(GameObjectId(entity_iid)),
            "DisableArea" => TriggerAction::DisableArea(GameObjectId(entity_iid)),
            _ => {
                error!("Unknown TriggerAction name: {}", action);
                TriggerAction::None
            }
        }
    }
}

impl Default for TriggerAction {
    fn default() -> Self {
        Self::None
    }
}
fn action(
    mut ev: EventReader<TriggerAction>,
    mut save: ResMut<SaveBuffer>,
    mut blocker_query: Query<(&mut Blocker, &GameObjectId)>,
    mut area_query: Query<(&mut Area, &GameObjectId)>,
) {
    for e in ev.iter() {
        // debug!("{e:?}");
        match e {
            TriggerAction::None => {}
            TriggerAction::ShowBlocker(target) => {
                for (mut blocker, id) in blocker_query.iter_mut() {
                    if target == id {
                        blocker.blocking = true;
                    }
                }
                match save.0.data.blockers.get_mut(target) {
                    Some(blocker) => {
                        blocker.blocking = true;
                    }
                    None => {
                        save.0.data.blockers.insert(
                            target.clone(),
                            Blocker {
                                blocking: true,
                                hx: 0.0,
                                hy: 0.0,
                            },
                        );
                    }
                }
            }
            TriggerAction::HideBlocker(target) => {
                for (mut blocker, id) in blocker_query.iter_mut() {
                    if target == id {
                        blocker.blocking = false;
                    }
                }
                match save.0.data.blockers.get_mut(target) {
                    Some(blocker) => {
                        blocker.blocking = false;
                    }
                    None => {
                        save.0.data.blockers.insert(
                            target.clone(),
                            Blocker {
                                blocking: false,
                                hx: 0.0,
                                hy: 0.0,
                            },
                        );
                    }
                }
            }
            TriggerAction::DisableArea(target) => {
                for (mut area, id) in area_query.iter_mut() {
                    if target == id {
                        area.disable = true;
                    }
                }
                match save.0.data.areas.get_mut(target) {
                    Some(area) => {
                        area.disable = true;
                    }
                    None => {
                        save.0.data.areas.insert(
                            target.clone(),
                            Area {
                                hx: 0.0,
                                hy: 0.0,
                                disable: true,
                            },
                        );
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Component, Default, Reflect)]
#[reflect_value()]
pub struct EventTrigger<T: Reflect + Clone> {
    // #[inspectable(ignore)]
    #[reflect(ignore)]
    pub event: PhantomData<T>,
    pub actions: Vec<TriggerAction>,
}
fn event_action<T: bevy::ecs::event::Event + TriggerEvent + Debug + Reflect + Clone>(
    mut ev: EventReader<T>,
    query: Query<&EventTrigger<T>>,
    mut action_ev: EventWriter<TriggerAction>,
) {
    // debug!("event_action: {:?}", ev.len());
    for e in ev.iter() {
        debug!("{e:?}");
        let entity = e.entity();
        if let Ok(t) = query.get(entity) {
            // debug!("{e:?}, actions: {:?}", t.actions.clone().into_iter());
            action_ev.send_batch(t.actions.clone().into_iter());
        }
    }
}

pub trait TriggerEvent {
    fn entity(&self) -> Entity;
}
impl TriggerEvent for UnitDieEvent {
    fn entity(&self) -> Entity {
        self.0
    }
}
impl TriggerEvent for PlayerEnterEvent {
    fn entity(&self) -> Entity {
        self.0
    }
}
