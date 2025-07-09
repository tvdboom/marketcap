use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::player::Player;
use crate::core::research::ResearchField;
use crate::core::resources::ImageIds;
use crate::core::ui::utils::CustomUi;
use crate::utils::{NameFromEnum, get_ratio};
use bevy::prelude::{EventWriter, Window};
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Align, Image, Layout, ScrollArea, Sense, Slider, TextStyle, Ui};
use strum::IntoEnumIterator;

pub fn research_panel(
    ui: &mut Ui,
    player: &mut Player,
    message: &mut EventWriter<MessageEv>,
    images: &ImageIds,
    window: &Window,
) {
    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
        ui.horizontal(|ui| {
            if ui
                .label(format!("{:.0}", player.research.capacity))
                .interact(Sense::click())
                .clicked()
            {}

            ui.add(
                Slider::new(&mut player.research.capacity, 0.0..=200.)
                    .show_value(false)
                    .step_by(1.),
            );

            ui.add(Image::new(SizedTexture::new(
                images.get("research"),
                [get_ratio(window.width(), window.height(), TextStyle::Heading); 2],
            )))
            .on_hover_text(
                "Research capacity\n\nThe research capacity represents the scale of the R&D \
                department. A higher capacity increases the research speed. Research costs are \
                paid monthly and are linearly proportional to the capacity.",
            );
        });
    });

    ScrollArea::vertical().show(ui, |ui| {
        ui.set_width(ui.available_width());

        for category in ResearchField::iter() {
            ui.add_space(5.);
            ui.horizontal(|ui| {
                ui.add_space(20.);
                ui.label(category.to_name());
            });
            ui.add_space(15.);

            ui.horizontal(|ui| {
                let complete = player.research.clone();
                for research in player.research.get_tech_mut(&category) {
                    ui.add_space(10.);

                    ui.add_enabled_ui(
                        research.progress < 100.
                            && matches!(&research.dependency, Some(r) if complete.has_technology(r)),
                        |ui| {
                            if ui.add_research(research).clicked() {
                                research.researching = true;

                                message.write(MessageEv {
                                    message: format!(
                                        "Started researching '{}'.",
                                        research.name.to_name(),
                                    ),
                                    level: MessageLevel::Info,
                                });
                            }
                        },
                    );
                }
            });
        }
    });
}
