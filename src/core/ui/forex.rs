use bevy::prelude::Window;
use bevy_egui::egui::Ui;

use crate::core::ui::state::UiState;

pub fn forex_panel(ui: &mut Ui, ui_state: &mut UiState, window: &Window) {
    ui.add_space(window.height() * 0.02);

    ui.separator();
}
