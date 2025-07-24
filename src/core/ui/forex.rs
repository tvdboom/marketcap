use bevy::prelude::Window;
use bevy_egui::egui::{ScrollArea, Ui};

use crate::core::constants::CURRENCY;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::instrument::Instrument;
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::ui::state::{OrderOptions, UiState};
use crate::core::ui::utils::CustomUi;

pub fn forex_panel(
    ui: &mut Ui,
    state: &mut UiState,
    economy: &GlobalEconomy,
    player: &Player,
    images: &ImageIds,
    window: &Window,
) {
    ui.label(
        "Forex, short for foreign exchange, is the global marketplace for buying and \
        selling currencies. It is the largest and most liquid financial market in the world, \
        where currencies are traded in pairs. For example, the USD/EUR pair represents \
        the exchange rate between the US dollar and the euro. When buying this pair, you \
        are effectively buying dollars with euros.\n\n\
        The exchange rate is heavily influenced by the commodity prices produced in the \
        corresponding country. Note that government bonds are bought in the national currency, \
        and so are the coupon payments. The current exchange rate is automatically applied.",
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
            ]
            .into(),
            &mut state.forex.order,
            window,
        );

        let instruments = economy
            .currencies
            .iter()
            .filter(|c| c.name != CURRENCY)
            .map(|c| c as &dyn Instrument)
            .collect::<Vec<_>>();

        for inst in OrderOptions::sort_instrument(instruments, &state.forex.order, economy, player)
        {
            let response = ui.add_instrument(
                inst,
                economy,
                player,
                Some(&mut state.forex.plot_range),
                images,
                window,
            );

            if response.clicked() {
                state.modal = Some(inst.kind());
            }
        }
    });
}
