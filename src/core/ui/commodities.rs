use bevy::prelude::Window;
use bevy_egui::egui::Ui;
use strum::IntoEnumIterator;

use crate::core::ui::state::{CommodityTab, UiState};
use crate::core::ui::utils::{CustomUi, TextSizes, add_text};
use crate::utils::NameFromEnum;

pub fn commodities_panel(ui: &mut Ui, ui_state: &mut UiState, window: &Window) {
    ui.horizontal(|ui| {
        for tab in CommodityTab::iter() {
            ui.selectable_value(
                &mut ui_state.commodities,
                tab,
                add_text(
                    format!("{}  {}", tab.emoji(), tab.to_name()),
                    window.l_size(),
                ),
            );
        }
    });

    ui.separator();

    ui.add_text(
        "Commodities are raw materials or primary agricultural products that can be \
        bought, sold and traded. They serve as the building blocks of the global economy,\
        their prices often having a direct impact on stock prices.\n\n\
        Commodities typically present volatile price movements. Be aware, commodities \
        are natural products that degrade over time. Be sure to sell them before they lose \
        their value.",
        window.m_size(),
    );

    ui.separator();

    ui.add_space(window.height() * 0.02);

    match ui_state.commodities {
        CommodityTab::Overview => {
            ui.add_text("", window.m_size());

            ui.separator();
        },
        CommodityTab::Market => {
            ui.add_text("", window.m_size());

            ui.separator();
        },
    }
}
