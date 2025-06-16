use bevy::prelude::Window;
use bevy_egui::egui::{ScrollArea, Ui};
use itertools::Itertools;

use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::Instrument;
use crate::core::player::{InstrumentKind, Player};
use crate::core::resources::ImageIds;
use crate::core::ui::state::{OrderOptions, UiState};
use crate::core::ui::utils::CustomUi;
use crate::utils::NameFromEnum;

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
        in a short period of time. The value of a cryptocurrency is purely based on supply and \
        demand. If the price reaches zero, the currency is removed from the exchange and can no \
        longer be traded.",
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

        let mut cryptos = economy
            .cryptos
            .iter()
            .sorted_by(|a, b| match state.cryptos.order {
                OrderOptions::Name => a.name.to_lowername().cmp(&b.name.to_lowername()),
                OrderOptions::OwnedAmount => player
                    .get_owned(&InstrumentKind::Crypto(b.name))
                    .cmp(&player.get_owned(&InstrumentKind::Crypto(a.name))),
                OrderOptions::OwnedValue => player
                    .get_value(&InstrumentKind::Crypto(b.name), economy)
                    .partial_cmp(&player.get_value(&InstrumentKind::Crypto(a.name), economy))
                    .unwrap_or(std::cmp::Ordering::Equal),
                OrderOptions::Price => a
                    .current()
                    .partial_cmp(&b.current())
                    .unwrap_or(std::cmp::Ordering::Equal),
                OrderOptions::Volatility => a
                    .volatility
                    .partial_cmp(&b.volatility)
                    .unwrap_or(std::cmp::Ordering::Equal),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        if state.cryptos.descending {
            cryptos.reverse();
        }

        for crypto in cryptos {
            let response = ui.add_commodity(crypto, images, window);

            if response.clicked() {
                state.modal = Some(InstrumentKind::Crypto(crypto.name));
            }
        }
    });
}
