use bevy::prelude::Window;
use bevy_egui::egui::Ui;
use strum::IntoEnumIterator;

use crate::core::ui::state::{BondTab, UiState};
use crate::core::ui::utils::{CustomUi, TextSizes, add_text};
use crate::utils::NameFromEnum;

pub fn bonds_panel(ui: &mut Ui, ui_state: &mut UiState, window: &Window) {
    ui.horizontal(|ui| {
        for tab in BondTab::iter() {
            ui.selectable_value(
                &mut ui_state.bonds,
                tab,
                add_text(
                    format!("{}  {}", tab.emoji(), tab.to_name()),
                    window.l_size(),
                ),
            );
        }
    });

    ui.add_space(window.height() * 0.02);

    ui.separator();

    match ui_state.bonds {
        BondTab::Overview => {
            ui.add_text("", window.m_size());

            ui.separator();
        },
        BondTab::Government => {
            ui.add_text("", window.m_size());

            ui.separator();
        },
        BondTab::Corporate => {
            ui.add_text("", window.m_size());

            ui.separator();
        },
    }
}
