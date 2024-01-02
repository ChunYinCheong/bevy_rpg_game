use std::{any::TypeId, collections::HashMap};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use strum::IntoEnumIterator;

use crate::plugins::game_world::{GameObjectId, GameObjectType};

use super::scene_loader::SceneRes;

pub struct EditorComponentPlugin;
impl Plugin for EditorComponentPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .init_resource::<EditorComponentRes>()
            .add_event::<GoToObject>()
            .add_system(go_to_object)
            .add_system(component_list_ui)
            .add_system(object_list_ui);
    }
}

pub struct GoToObject(GameObjectId);
fn go_to_object(
    mut go_events: EventReader<GoToObject>,
    mut cameras: Query<(&Camera, &mut Transform)>,
    objs: Query<(&GameObjectId, &GlobalTransform)>,
) {
    for ev in go_events.iter() {
        for (id, gt) in objs.iter() {
            if &ev.0 != id {
                continue;
            }

            for (c, mut t) in cameras.iter_mut() {
                if !c.is_active {
                    continue;
                }
                t.translation.x = gt.translation().x;
                t.translation.y = gt.translation().y;
            }
        }
    }
}

pub struct CreateObject(String);
fn create_object(
    mut commands: Commands,
    mut events: EventReader<CreateObject>,
    objs: Query<&GameObjectId>,
    mut scene_res: ResMut<SceneRes>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let camera = cameras.iter().find(|(c, _)| c.is_active);
    for ev in events.iter() {
        if objs.iter().any(|id| id.0 == ev.0) {
            continue;
        }
        if let Some((_, tran)) = camera {
            let t = Transform::from_xyz(tran.translation().x, tran.translation().y, 0.0);
            scene_res.ecs.transforms.insert(
                GameObjectId(ev.0.clone()),
                (t.translation, t.rotation, t.scale),
            );
            todo!();
        }
        // commands
    }
}

fn object_list_ui(
    mut egui_context: ResMut<EguiContext>,
    mut object: ResMut<SceneRes>,
    mut go_events: EventWriter<GoToObject>,
) {
    egui::Window::new("Scene")
        .resizable(true)
        .vscroll(true)
        .hscroll(true)
        .show(egui_context.ctx_mut(), |ui| {
            let keys = object.ecs.transforms.keys().cloned().collect::<Vec<_>>();
            for k in keys {
                ui.collapsing(&k.0, |ui| {
                    if let Some(value) = object.ecs.transforms.get_mut(&k) {
                        // egui::CollapsingHeader::new("Transform")
                        //     .id_source(format!("{} Transform", k.0))
                        //     .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Transform");
                            ui.add(egui::DragValue::new(&mut value.0.x).speed(8.0));
                            ui.add(egui::DragValue::new(&mut value.0.y).speed(8.0));
                            ui.add(egui::DragValue::new(&mut value.0.z).speed(8.0));
                            if ui.button("Go").clicked() {
                                go_events.send(GoToObject(k.clone()));
                            }
                        });
                        // });
                    }

                    if let Some(value) = object.ecs.objects.get_mut(&k) {
                        // egui::CollapsingHeader::new("GameObjectType")
                        //     .id_source(format!("{} GameObjectType", k.0))
                        //     .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("GameObjectType");
                            egui::ComboBox::from_label("")
                                .selected_text(format!("{value:?}"))
                                .show_ui(ui, |ui| {
                                    for t in GameObjectType::iter() {
                                        ui.selectable_value(value, t, t.to_string());
                                    }
                                });
                        });
                        // });
                    }
                });
            }
        });
}

fn component_list_ui(
    mut egui_context: ResMut<EguiContext>,
    mut component: ResMut<EditorComponentRes>,
) {
    egui::Window::new("EditorComponentRes")
        // .open(&mut show)
        // .collapsible(false)
        // .vscroll(true)
        // .hscroll(true)
        // .default_pos([0.0, 0.0])
        // .min_width(600.0)
        // .min_height(400.0)
        // .default_size([600.0, 600.0])
        .resizable(true)
        // .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(egui_context.ctx_mut(), |ui| {
            egui::Grid::new("some_unique_id")
                .striped(true)
                .spacing([16.0, 16.0])
                .show(ui, |ui| {
                    for item in component.map.values_mut() {
                        // ui.label(&item.item_id);
                        ui.label("Component Start");
                        item.ui(ui);
                        ui.label("Component End");
                        // ui.label(item.price.to_string());
                        // if ui.button("Buy!").clicked() {
                        //     if let Ok(entity) = player.get_single() {
                        //         buy_events.send(BuyEvent {
                        //             buyer: entity,
                        //             item_id: item.item_id,
                        //             price: item.price,
                        //         });
                        //     }
                        // }
                        ui.end_row();
                    }
                });
        });
}

#[derive(Debug, Default, Resource)]
pub struct EditorComponentRes {
    pub map: HashMap<TypeId, Box<dyn EditorComponent>>,
}

impl EditorComponentRes {
    pub fn register<T: EditorComponent + Default + 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.map.insert(type_id, Box::<T>::default());
        // self.register_raw::<T, _>(|value, ui, context| {
        //     value.ui(ui, <T as Inspectable>::Attributes::default(), context)
        // });
    }
}

pub trait EditorComponent: std::fmt::Debug + Send + Sync {
    // type Attributes: Default + Clone;

    fn ui(&mut self, ui: &mut egui::Ui);

    // fn ui_raw(&mut self, ui: &mut egui::Ui, options: Self::Attributes) {
    //     let mut empty_context = Context::new_shared(None);
    //     self.ui(ui, options, &mut empty_context);
    // }
}

pub trait RegisterEditorComponent {
    fn register_editor_component<T: EditorComponent + Component + Default + 'static>(
        &mut self,
    ) -> &mut Self;
}

impl RegisterEditorComponent for App {
    fn register_editor_component<T: EditorComponent + Component + Default + 'static>(
        &mut self,
    ) -> &mut Self {
        self.world
            .get_resource_or_insert_with(EditorComponentRes::default)
            .register::<T>();

        self
    }
}

impl EditorComponent for Transform {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Transform");
        ui.text_edit_singleline(&mut self.translation.to_string());
    }
}
