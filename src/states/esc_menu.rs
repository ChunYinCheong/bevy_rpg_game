use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::AppState;

pub(crate) fn esc_menu(
    mut egui_context: ResMut<EguiContext>,
    mut app_state: ResMut<State<AppState>>,
) {
    egui::Window::new("Esc Menu").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Option").clicked() {
            // To option
        }
        if ui.button("Back to title").clicked() {
            // To exit
            app_state.replace(AppState::Title).unwrap();
        }
        if ui.button("Close").clicked() {
            // To exit
            app_state.pop().unwrap();
        }
    });
}
