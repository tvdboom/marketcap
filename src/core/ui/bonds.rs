use bevy::prelude::Window;
use bevy_egui::egui::Ui;
use strum::IntoEnumIterator;

use crate::core::ui::state::{BondTab, UiState};
use crate::utils::NameFromEnum;

pub fn bonds_panel(ui: &mut Ui, ui_state: &mut UiState, window: &Window) {
    ui.horizontal(|ui| {
        for tab in BondTab::iter() {
            ui.selectable_value(
                &mut ui_state.bonds,
                tab,
                format!("{}  {}", tab.emoji(), tab.to_name()),
            );
        }
    });

    ui.add_space(window.height() * 0.02);

    ui.separator();

    match ui_state.bonds {
        BondTab::Government => {
            ui.separator();
        },
        BondTab::Corporate => {
            ui.separator();
        },
    }
}
