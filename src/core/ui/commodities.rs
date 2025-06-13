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

        let mut commodities = economy
            .commodities
            .iter()
            .sorted_by(|a, b| match state.commodities.order {
                OrderOptions::Name => a.name.to_lowername().cmp(&b.name.to_lowername()),
                OrderOptions::OwnedAmount => player
                    .get_owned(&InstrumentKind::Commodity(b.name))
                    .cmp(&player.get_owned(&InstrumentKind::Commodity(a.name))),
                OrderOptions::OwnedValue => player
                    .get_value(&InstrumentKind::Commodity(b.name), economy)
                    .partial_cmp(&player.get_value(&InstrumentKind::Commodity(a.name), economy))
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

        if state.commodities.descending {
            commodities.reverse();
        }

        for commodity in commodities {
            let response = ui.add_commodity(commodity, images, window);

            if response.clicked() {
                state.modal = Some(InstrumentKind::Commodity(commodity.name));
            }
        }
    });
}
