use bevy::prelude::Window;
use bevy_egui::egui::{ScrollArea, Ui};

use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::instrument::Instrument;
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::ui::state::{OrderOptions, UiState};
use crate::core::ui::utils::CustomUi;

pub fn crypto_panel(
    ui: &mut Ui,
    state: &mut UiState,
    economy: &GlobalEconomy,
    player: &Player,
    images: &ImageIds,
    window: &Window,
) {
    ui.label(
        "Cryptocurrencies are a novel trading instrument that operate on a blockchain \
        network. They are decentralized digital assets, meaning that they are not controlled \
        by any central authority like a government or bank.\n\n\
        Cryptos are highly volatile and speculative, with prices that can fluctuate dramatically \
        in a short period of time. If the price reaches zero, the currency is removed from the \
        exchange and can no longer be traded.",
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
            &mut state.cryptos,
            window,
        );

        let mut instruments = economy
            .cryptos
            .iter()
            .map(|c| c as &dyn Instrument)
            .collect::<Vec<_>>();

        for inst in OrderOptions::sort_instrument(&mut instruments, &state.cryptos, economy, player)
        {
            ui.add_enabled_ui(inst.current() > 0., |ui| {
                let response = ui.add_instrument(inst, economy, images, window);

                if response.clicked() {
                    state.modal = Some(inst.kind());
                }
            });
        }
    });
}
