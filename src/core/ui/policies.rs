use bevy::prelude::*;
use bevy_egui::egui::{Ui, Button};
use strum::IntoEnumIterator;

use crate::core::global_economy::{GlobalEconomy};
use crate::core::messages::MessageEv;
use crate::core::player::Player;
use crate::core::ui::state::{PoliciesTab, UiState};
use crate::core::ui::utils::CustomUi;
use crate::utils::NameFromEnum;

pub fn policies_panel(
    ui: &mut Ui,
    state: &mut UiState,
    economy: &mut GlobalEconomy,
    player: &mut Player,
    message: &mut EventWriter<MessageEv>,
    window: &Window,
) {
    ui.horizontal(|ui| {
        for tab in PoliciesTab::iter() {
            ui.selectable_value(
                &mut state.policies,
                tab.clone(),
                format!("{}  {}", tab.emoji(), tab.to_name()),
            );
        }
    });

    ui.separator();

    ui.add_space(window.height() * 0.01);

    match state.policies {
        PoliciesTab::Sectors => {
            ui.label(
                "Credit refers to the ability of borrowing money, with the promise of repayment \
                in the future. It's a fundamental part of the financial system, allowing companies to \
                make purchases, invest, and manage expenses beyond their immediate cash availability. \
                There are two types of loans: term loans and margin loans. Term loans provide cash to \
                use at the discretion of the borrower, while margin loans are taken to leverage a specific \
                instrument. Six months after the start date of a term loan, a company can choose to repay \
                the debt early, paying an additional fee to the provider to cover missed earnings. A new \
                loan can be taken by the same provider only when the remaining debt is less than 50% of \
                the principal.",
            );

            ui.separator();
        },
        PoliciesTab::Politics => {
            ui.label(
                "The game features four political fields where players can influence the \
                political landscape: government, ideology, culture, and orientation. The values \
                in these fields represent the global tendency towards one of the two directions. \
                Countries and sectors are influenced depending on their affiliation to each field.\n\n\
                Use your influence to change the political landscape to your advantage.",
            );

            ui.separator();

            ui.heading("Government");

            ui.horizontal(|ui| {
                ui.label("Democracy");
                if ui.add_sized([20., 20.], Button::new("👈 +1")).clicked() {
                    economy.politics.government -= 1;
                }
                ui.add_bar(economy.politics.government);
                if ui.add_sized([20., 20.], Button::new("👉 +1")).clicked() {
                    economy.politics.government += 1;
                }
                ui.label("Autocracy");
            });

            ui.add_space(window.height() * 0.02);

            ui.heading("Ideology");

            ui.horizontal(|ui| {
                ui.label("Left");
                ui.add_bar(economy.politics.ideology);
                ui.label("Right");
            });

            ui.add_space(window.height() * 0.02);

            ui.heading("Culture");

            ui.horizontal(|ui| {
                ui.label("Conservative");
                ui.add_bar(economy.politics.culture);
                ui.label("Progressive");
            });

            ui.add_space(window.height() * 0.02);

            ui.heading("Orientation");

            ui.horizontal(|ui| {
                ui.label("Socialism");
                ui.add_bar(economy.politics.orientation);
                ui.label("Capitalism");
            });
        },
        PoliciesTab::Laws => (),
    }
}
