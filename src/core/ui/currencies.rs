use crate::core::ui::state::{CurrencyTab, UiState};
use crate::core::ui::utils::{CustomUi, TextSizes, add_text};
use crate::utils::NameFromEnum;
use bevy::prelude::Window;
use bevy_egui::egui::Ui;
use strum::IntoEnumIterator;

pub fn currencies_panel(ui: &mut Ui, ui_state: &mut UiState, window: &Window) {
    ui.horizontal(|ui| {
        for tab in CurrencyTab::iter() {
            ui.selectable_value(
                &mut ui_state.currencies,
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

    match ui_state.currencies {
        CurrencyTab::Overview => {
            ui.add_text("", window.m_size());

            ui.separator();
        }
        CurrencyTab::Forex => {
            ui.add_text("", window.m_size());

            ui.separator();
        }
        CurrencyTab::Crypto => {
            ui.add_text("", window.m_size());

            ui.separator();
        }
    }
}
