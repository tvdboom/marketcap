use bevy::prelude::Window;
use bevy_egui::egui::{ScrollArea, Ui};

use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::instrument::Instrument;
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::ui::state::{OrderOptions, UiState};
use crate::core::ui::utils::CustomUi;

pub fn stock_panel(
    ui: &mut Ui,
    state: &mut UiState,
    economy: &GlobalEconomy,
    player: &Player,
    images: &ImageIds,
    window: &Window,
) {
    ui.label(
        "Stocks are a traditional trading instrument representing partial ownership in a \
        company. They are issued by corporations to raise capital and are bought and sold on \
        stock exchanges. Owning a stock entitles the investor to a share of the company’s \
        profits, paid out quarterly as dividends.\n\n\
        Stocks vary in volatility depending on the company, sector, and market conditions. \
        They are generally less volatile than other instruments such as commodities and \
        cryptocurrencies.",
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
            &mut state.stocks.order,
            window,
        );

        let instruments = economy
            .stocks
            .iter()
            .map(|c| c as &dyn Instrument)
            .collect::<Vec<_>>();

        for inst in OrderOptions::sort_instrument(instruments, &state.stocks.order, economy, player)
        {
            let response = ui.add_instrument(
                inst,
                economy,
                player,
                Some(&mut state.stocks.plot_range),
                images,
                window,
            );

            if response.clicked() {
                state.modal = Some(inst.kind());
            }
        }
    });
}
