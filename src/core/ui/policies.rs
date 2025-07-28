use bevy::prelude::*;
use bevy_egui::egui::{ScrollArea, Ui};
use strum::IntoEnumIterator;

use crate::core::factors::Factor;
use crate::core::global_economy::{GlobalEconomy};
use crate::core::player::Player;
use crate::core::politics::{Culture, Government, Ideology, Orientation};
use crate::core::ui::state::{PoliciesTab, UiState};
use crate::core::ui::utils::CustomUi;
use crate::utils::NameFromEnum;

pub fn policies_panel(
    ui: &mut Ui,
    state: &mut UiState,
    economy: &mut GlobalEconomy,
    player: &mut Player,
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
                in these fields represent the global tendency towards one of the two directions \
                in the field. Countries and sectors are influenced depending on their affiliation \
                to each field. Use your companies influence to change the landscape to your advantage.",
            );

            ui.separator();

            ScrollArea::vertical().show(ui, |ui| {
                ui.add_influence_block(
                    "Government",
                    Government::iter().next().unwrap().to_name(),
                    Government::iter().last().unwrap().to_name(),
                    &mut player.influence.current(),
                    &mut economy.politics.government,
                );

                ui.add_space(window.height() * 0.05);

                ui.add_influence_block(
                    "Ideology",
                    Ideology::iter().next().unwrap().to_name(),
                    Ideology::iter().last().unwrap().to_name(),
                    &mut player.influence.current(),
                    &mut economy.politics.ideology,
                );

                ui.add_space(window.height() * 0.05);

                ui.add_influence_block(
                    "Culture",
                    Culture::iter().next().unwrap().to_name(),
                    Culture::iter().last().unwrap().to_name(),
                    &mut player.influence.current(),
                    &mut economy.politics.culture,
                );

                ui.add_space(window.height() * 0.05);

                ui.add_influence_block(
                    "Orientation",
                    Orientation::iter().next().unwrap().to_name(),
                    Orientation::iter().last().unwrap().to_name(),
                    &mut player.influence.current(),
                    &mut economy.politics.orientation,
                );
            });
        },
        PoliciesTab::Laws => (),
    }
}
