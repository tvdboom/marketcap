use crate::core::constants::{GREEN, TOP_LABEL_FRAC};
use crate::core::game_settings::GameSettings;
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::states::GameState;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::widget_text::RichText;
use bevy_egui::egui::widgets::Image;
use bevy_egui::egui::{Align, Layout, TopBottomPanel};

pub fn top_panel(
    mut contexts: EguiContexts,
    game_settings: Res<GameSettings>,
    player: Res<Player>,
    game_state: Res<State<GameState>>,
    images: Res<ImageIds>,
    window: Single<&Window>,
) {
    let window_width = window.width();
    let window_height = window.height();

    let height = window_height * TOP_LABEL_FRAC;
    TopBottomPanel::top("top_panel")
        .exact_height(height)
        .show_separator_line(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(window_width * 0.1);

                ui.add(Image::new(SizedTexture::new(
                    images.get("logo"),
                    [height * 0.5, height * 0.5],
                )));
                ui.label(
                    RichText::new(player.market_cap().to_string())
                        .size(height * 0.5)
                        .color(GREEN),
                );

                ui.add_space(window_width * 0.05);

                ui.add(Image::new(SizedTexture::new(
                    images.get("cash"),
                    [height * 0.5, height * 0.5],
                )));
                ui.label(
                    RichText::new(player.cash.to_string())
                        .size(height * 0.5)
                        .color(GREEN),
                );

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add_space(window_width * 0.1);

                    ui.label(
                        RichText::new(game_settings.date.format("%d-%m-%Y").to_string())
                            .size(height * 0.5),
                    );

                    ui.add(Image::new(SizedTexture::new(
                        images.get(if *game_state.get() == GameState::Running {
                            "time"
                        } else {
                            "time-paused"
                        }),
                        [height * 0.5, height * 0.5],
                    )));
                });
            });
        });
}
