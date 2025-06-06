use bevy::prelude::Window;
use bevy_egui::egui::{Align, ComboBox, Layout, ScrollArea, Ui};
use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::Instrument;
use crate::core::instruments::bonds::BondKind;
use crate::core::player::{InstrumentKind, Player};
use crate::core::resources::ImageIds;
use crate::core::ui::state::{OrderOptions, UiState};
use crate::core::ui::utils::CustomUi;
use crate::utils::NameFromEnum;

pub fn bonds_panel(
    ui: &mut Ui,
    ui_state: &mut UiState,
    economy: &GlobalEconomy,
    player: &Player,
    images: &ImageIds,
    window: &Window,
) {
    ui.horizontal(|ui| {
        for tab in BondKind::iter() {
            ui.selectable_value(
                &mut ui_state.bonds,
                tab,
                format!("{}  {}", tab.emoji(), tab.to_name()),
            );
        }
    });

    ui.add_space(window.height() * 0.02);

    ui.separator();

    ui.label(
        "\
        Bonds are fixed-income securities that represent a loan made by an investor to a \
        corporate or governmental borrower. When you purchase a bond, you are essentially \
        lending money to the issuer in exchange for periodic interest payments and the \
        return of the bond's face value when it matures.\n\n\
        Use bonds as a way of securing a steady cash inflow to pay your debts. High-yield \
        bonds offer higher interest than investment grade bonds, but have a higher chance \
        of default, in which case you won't get the face value back. Bond interest payments \
        are semi-annually. Government bonds are issued every six months, while corporate bonds \
        are issued once a year.",
    );

    ui.separator();

    ScrollArea::vertical().show(ui, |ui| {
        ui.set_width(ui.available_width());

        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            ui.set_max_height(window.height() * 0.05);

            ui.add_space(window.width() * 0.02);

            ComboBox::from_id_salt("order")
                .selected_text("Order by")
                .show_ui(ui, |ui| {
                    for order in OrderOptions::iter().filter(|o| {
                        !matches!(
                            o,
                            OrderOptions::HighestVolatility | OrderOptions::LowestVolatility
                        )
                    }) {
                        ui.selectable_value(&mut ui_state.bond_modal.order, order, order.to_name());
                    }
                });
        });

        let bonds = economy
            .bonds
            .iter()
            .filter(|b| b.kind == ui_state.bonds)
            .sorted_by(|a, b| match ui_state.bond_modal.order {
                OrderOptions::Alphabetical => a.name.to_lowername().cmp(&b.name.to_lowername()),
                OrderOptions::OwnedAmount => player
                    .get_owned(&InstrumentKind::Bond(b.name))
                    .cmp(&player.get_owned(&InstrumentKind::Bond(a.name))),
                OrderOptions::OwnedValue => player
                    .get_value(&InstrumentKind::Bond(b.name), economy)
                    .partial_cmp(&player.get_value(&InstrumentKind::Bond(a.name), economy))
                    .unwrap_or(std::cmp::Ordering::Equal),
                OrderOptions::LowestPrice => a
                    .current()
                    .partial_cmp(&b.current())
                    .unwrap_or(std::cmp::Ordering::Equal),
                OrderOptions::HighestPrice => b
                    .current()
                    .partial_cmp(&a.current())
                    .unwrap_or(std::cmp::Ordering::Equal),
                OrderOptions::LowestInterest => a
                    .interest
                    .partial_cmp(&b.interest)
                    .unwrap_or(std::cmp::Ordering::Equal),
                OrderOptions::HighestInterest => b
                    .interest
                    .partial_cmp(&a.interest)
                    .unwrap_or(std::cmp::Ordering::Equal),
                _ => unreachable!(),
            });

        for bond in bonds {
            let response = ui.add_bond(bond, images, window);

            if response.clicked() {
                ui_state.active_modal = Some(InstrumentKind::Bond(bond.name));
            }
        }
    });
}
