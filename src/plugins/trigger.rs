use std::{fmt::Debug, marker::PhantomData};

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

use super::{
    area::{Area, PlayerEnterEvent},
    blocker::Blocker,
    editor::EditorRes,
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
            // .register_inspectable::<EventTrigger<UnitDieEvent>>()
            .add_system(event_action::<UnitDieEvent>)
            // .register_inspectable::<EventTrigger<PlayerEnterEvent>>()
            .add_system(event_action::<PlayerEnterEvent>)
            // ...
            ;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Inspectable)]
pub enum TriggerAction {
    ShowBlocker(GameObjectId),
    HideBlocker(GameObjectId),
    DisableArea(GameObjectId),
}
fn action(
    mut ev: EventReader<TriggerAction>,
    editor: Res<EditorRes>,
    mut save: ResMut<SaveBuffer>,
    mut blocker_query: Query<(&mut Blocker, &GameObjectId)>,
    mut area_query: Query<(&mut Area, &GameObjectId)>,
) {
    for e in ev.iter() {
        match e {
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
                        if let Some(blocker) = editor.ecs.blockers.get(target) {
                            let mut blocker = blocker.clone();
                            blocker.blocking = true;
                            save.0.data.blockers.insert(target.clone(), blocker);
                        }
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
                        if let Some(blocker) = editor.ecs.blockers.get(target) {
                            let mut blocker = blocker.clone();
                            blocker.blocking = false;
                            save.0.data.blockers.insert(target.clone(), blocker);
                        }
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
                        if let Some(area) = editor.ecs.areas.get(target) {
                            let mut area = area.clone();
                            area.disable = true;
                            save.0.data.areas.insert(target.clone(), area);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Component)]
pub struct EventTrigger<T> {
    pub event: PhantomData<T>,
    pub actions: Vec<TriggerAction>,
}
fn event_action<T: bevy::ecs::event::Event + TriggerEvent + Debug>(
    mut ev: EventReader<T>,
    query: Query<&EventTrigger<T>>,
    mut action_ev: EventWriter<TriggerAction>,
) {
    for e in ev.iter() {
        debug!("{e:?}");
        let entity = e.entity();
        if let Ok(t) = query.get(entity) {
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
