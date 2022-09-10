use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::AppState;

pub fn main_menu(mut egui_context: ResMut<EguiContext>, mut app_state: ResMut<State<AppState>>) {
    // egui::Window::new("Hello")
    egui::TopBottomPanel::bottom("main_menu").show(egui_context.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            let response = ui.button("Start");
            response.request_focus();
            if response.clicked() {
                app_state.set(AppState::Level).unwrap();
            }
            if ui.add_enabled(false, egui::Button::new("Option")).clicked() {
                // To option
            }
            if ui.add_enabled(false, egui::Button::new("Exit")).clicked() {
                // To exit
            }
            ui.allocate_space(ui.available_size());
        });
    });
}
