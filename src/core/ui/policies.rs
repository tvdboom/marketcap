use bevy::prelude::*;
use bevy_egui::egui::{ScrollArea, Ui};
use strum::IntoEnumIterator;

use crate::core::global_economy::GlobalEconomy;
use crate::core::player::Player;
use crate::core::politics::PoliticalField;
use crate::core::resources::ImageIds;
use crate::core::sectors::SectorName;
use crate::core::ui::state::{PoliciesTab, UiState};
use crate::core::ui::utils::CustomUi;
use crate::utils::NameFromEnum;

pub fn policies_panel(
    ui: &mut Ui,
    state: &mut UiState,
    economy: &mut GlobalEconomy,
    player: &mut Player,
    images: &ImageIds,
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
                "The companies in the game are part of the following collection of sectors, \
                 which represents the foundational pillars of the global economy, encompassing \
                 a diverse range of industries that drive market dynamics. The score of the sector \
                 is determined by the underlying commodity prices, and in turn, influences the \
                 stock market. Use your companies influence to change the scores to your advantage.",
            );

            ui.separator();

            ScrollArea::vertical().show(ui, |ui| {
                for name in SectorName::iter() {
                    let mut value = economy.sectors.iter().find(|s| s.name == name).unwrap().value;

                    ui.add_influence_block(
                        format!("{} {}", name.emoji(), name.to_name()),
                        |ui| ui.add_sector(&name, economy, images, window),
                        "Negative",
                        "Positive",
                        &mut player.influence.score,
                        &mut value,
                    );

                    economy.sectors.iter_mut().find(|s| s.name == name).unwrap().value = value;

                    ui.add_space(window.height() * 0.05);
                }
            });
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
                for field in PoliticalField::iter() {
                    let mut value = economy.politics.get_mut(&field).clone();

                    ui.add_influence_block(
                        format!("{} {}", field.emoji(), field.to_name()),
                        |ui| ui.add_politics(&field, economy, images, window),
                        field.fields().first().unwrap(),
                        field.fields().last().unwrap(),
                        &mut player.influence.score,
                        &mut value,
                    );

                    *economy.politics.get_mut(&field) = value;

                    ui.add_space(window.height() * 0.05);
                }
            });
        },
        PoliciesTab::Laws => (),
    }
}
