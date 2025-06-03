use crate::core::ui::state::{OverviewTab, UiState};
use crate::core::ui::utils::{TextSizes, add_text};
use crate::utils::NameFromEnum;
use bevy::prelude::Window;
use bevy_egui::egui::Ui;
use strum::IntoEnumIterator;

pub fn overview_panel(ui: &mut Ui, ui_state: &mut UiState, window: &Window) {
    ui.horizontal(|ui| {
        for tab in OverviewTab::iter() {
            ui.selectable_value(
                &mut ui_state.overview,
                tab,
                add_text(
                    format!("{}  {}", tab.emoji(), tab.to_name()),
                    window.l_size(),
                ),
            );
        }
    });

    ui.separator();
}
