use bevy::prelude::{EventWriter, Window};
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Align, Image, Layout, Pos2, ScrollArea, Sense, Slider, TextStyle, Ui};
use strum::IntoEnumIterator;

use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::player::Player;
use crate::core::research::{ResearchField, TechName};
use crate::core::resources::ImageIds;
use crate::core::ui::utils::CustomUi;
use crate::utils::{NameFromEnum, get_ratio};

pub fn research_panel(
    ui: &mut Ui,
    player: &mut Player,
    message: &mut EventWriter<MessageEv>,
    images: &ImageIds,
    window: &Window,
) {
    let max_capacity = if player.has_tech(&TechName::ImprovedResearch) {
        300.
    } else {
        200.
    };

    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
        ui.horizontal(|ui| {
            if ui
                .label(format!("{:.0}", player.research.capacity))
                .interact(Sense::click())
                .clicked()
            {
                player.research.capacity = max_capacity;
            }

            ui.spacing_mut().slider_width = window.width() * 0.17;
            ui.add(
                Slider::new(&mut player.research.capacity, 0.0..=max_capacity)
                    .show_value(false)
                    .step_by(1.),
            );

            ui.add(Image::new(SizedTexture::new(
                images.get("research"),
                [get_ratio(
                    window.width() * 0.75,
                    window.height() * 0.75,
                    TextStyle::Heading,
                ); 2],
            )));
        })
        .response
        .on_hover_text(
            "Research capacity\n\nThe research capacity represents the scale of the R&D \
            department. A higher capacity increases the research speed. Research costs are \
            paid monthly and are linearly proportional to the capacity.",
        );
    });

    ScrollArea::vertical().show(ui, |ui| {
        ui.set_width(ui.available_width());

        let complete = player.research.clone();
        for category in ResearchField::iter() {
            ui.add_space(5.);
            ui.horizontal(|ui| {
                ui.add_space(20.);
                ui.label(category.to_name());
            });
            ui.add_space(5.);

            let mut rects = Vec::new();
            ui.horizontal(|ui| {
                for research in player.research.get_tech_mut(&category) {
                    ui.add_space(20.);

                    let response = ui.add_enabled_ui(
                        research
                            .dependencies
                            .as_ref()
                            .map_or(true, |v| v.iter().all(|d| complete.has_technology(d))),
                        |ui| {
                            let block = ui.add_technology(research);

                            if block.clicked() && !research.researching && research.progress < 100.
                            {
                                research.researching = true;

                                message.write(MessageEv {
                                    message: format!(
                                        "Started researching '{}'.",
                                        research.name.to_name()
                                    ),
                                    level: MessageLevel::Info,
                                });
                            }

                            block
                        },
                    );

                    rects.push(response.response.rect);
                }

                // Draw arrows after layout
                let painter = ui.painter();
                for w in rects.windows(2) {
                    let start = Pos2::new(w[0].right() + 5., w[0].center().y);
                    let end = Pos2::new(w[1].left() - 5., w[1].center().y);
                    painter.arrow(start, end - start, ui.visuals().widgets.inactive.fg_stroke);
                }
            });

            ui.add_space(25.);
        }
    });
}
