use bevy::prelude::Window;
use bevy_egui::egui::{ScrollArea, Ui};

use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::Instrument;
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::ui::state::{OrderOptions, UiState};
use crate::core::ui::utils::CustomUi;

pub fn commodities_panel(
    ui: &mut Ui,
    state: &mut UiState,
    economy: &GlobalEconomy,
    player: &Player,
    images: &ImageIds,
    window: &Window,
) {
    ui.label(
        "Commodities are raw materials or primary agricultural products that can be traded. \
        They serve as the building blocks of the global economy, their prices often having a \
        direct impact on bond and stock prices.\n\n\
        Because commodities are physical instruments, they require storage facilities to preserve \
        the products before selling them. This incurs a storage cost, which is a variable price \
        per unit per month (with a minimum of one month). Storage cost prices increase with \
        inflation.",
    );

    ui.separator();

    ScrollArea::vertical().show(ui, |ui| {
        ui.set_width(ui.available_width());

        ui.add_combobox(
            "",
            [
                OrderOptions::Name,
                OrderOptions::OwnedAmount,
                OrderOptions::OwnedValue,
                OrderOptions::Price,
                OrderOptions::Volatility,
            ]
            .into(),
            &mut state.commodities,
            window,
        );

        let mut instruments = economy
            .commodities
            .iter()
            .map(|c| c as &dyn Instrument)
            .collect::<Vec<_>>();

        for inst in OrderOptions::sort(&mut instruments, &state.commodities, economy, player) {
            let response = ui.add_instrument(inst, images, window);

            if response.clicked() {
                state.modal = Some(inst.kind());
            }
        }
    });
}
