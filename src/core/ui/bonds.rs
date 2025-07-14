use bevy::prelude::Window;
use bevy_egui::egui::{ScrollArea, Ui};
use strum::IntoEnumIterator;

use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::bonds::BondKind;
use crate::core::instruments::instrument::Instrument;
use crate::core::player::Player;
use crate::core::research::TechName;
use crate::core::resources::ImageIds;
use crate::core::ui::state::{OrderOptions, UiState};
use crate::core::ui::utils::CustomUi;
use crate::utils::NameFromEnum;

pub fn bonds_panel(
    ui: &mut Ui,
    state: &mut UiState,
    economy: &GlobalEconomy,
    player: &Player,
    images: &ImageIds,
    window: &Window,
) {
    ui.horizontal(|ui| {
        for tab in BondKind::iter() {
            if tab != BondKind::Corporate || player.has_tech(&TechName::CorporateBonds) {
                ui.selectable_value(
                    &mut state.bonds.tab,
                    tab,
                    format!("{}  {}", tab.emoji(), tab.to_name()),
                );
            }
        }
    });

    ui.add_space(window.height() * 0.02);

    ui.separator();

    ui.label(
        "Bonds are fixed-income securities that represent a loan made by an investor to a \
        corporate or governmental borrower. When you purchase a bond, you are essentially lending \
        money to the issuer in exchange for periodic interest payments and the return of the \
        bond's face value when it matures.\n\n\
        Use bonds as a way of securing a steady cash inflow to pay your debts. High-yield bonds \
        offer higher interest than investment grade bonds, but have a higher chance of default. \
        Bond interest payments are semi-annually and, in the case of government bonds, paid out \
        in the national currency (exchange rates apply). Government bonds are issued every six \
        months and corporate bonds are issued once a year.\n\n\
        Bonds in this game are hold-to-maturity only, meaning that they can't be traded.",
    );

    ui.separator();

    ScrollArea::vertical().show(ui, |ui| {
        ui.set_width(ui.available_width());

        let order = if state.bonds.tab == BondKind::Government {
            &mut state.bonds.order_government
        } else {
            &mut state.bonds.order_corporate
        };

        ui.add_combobox(
            "",
            [
                OrderOptions::Name,
                OrderOptions::OwnedAmount,
                OrderOptions::OwnedValue,
                OrderOptions::Quality,
                OrderOptions::Interest,
            ]
            .into(),
            order,
            window,
        );

        let instruments = economy
            .bonds
            .iter()
            .filter(|b| {
                b.kind() == state.bonds.tab
                    && (!b.quality().is_high_yield() || player.has_tech(&TechName::HighYield))
            })
            .map(|c| c as &dyn Instrument)
            .collect::<Vec<_>>();

        for inst in OrderOptions::sort_instrument(instruments, order, economy, player) {
            let response = ui.add_instrument(inst, economy, player, None, images, window);

            if response.clicked() {
                state.modal = Some(inst.kind());
            }
        }
    });
}
