use bevy::prelude::Window;
use bevy_egui::egui::{ScrollArea, Ui};

use crate::core::global_economy::GlobalEconomy;
use crate::core::resources::ImageIds;
use crate::core::securities::SecurityKind;
use crate::core::ui::state::UiState;
use crate::core::ui::utils::{CustomUi, TextSizes};

pub fn commodities_panel(
    ui: &mut Ui,
    ui_state: &mut UiState,
    economy: &GlobalEconomy,
    images: &ImageIds,
    window: &Window,
) {
    ui.add_text(
        "Commodities are raw materials or primary agricultural products that can be \
        bought, sold and traded. They serve as the building blocks of the global economy,\
        their prices often having a direct impact on stock prices.\n\n\
        Commodities typically present volatile price movements. Be aware, commodities are \
        natural products that degrade over time. Be sure to sell them before they lose their \
        value.",
        window.m_size(),
    );

    ui.separator();

    ui.add_space(window.height() * 0.02);

    ScrollArea::vertical().show(ui, |ui| {
        for security in economy
            .securities
            .iter()
            .filter(|k| k.kind == SecurityKind::Commodity)
        {
            let response = ui.add_security(security, images, window);

            if response.clicked() {
                ui_state.trade.security = security.name;
                ui_state.trade.active = true;
            }
        }
    });
}
