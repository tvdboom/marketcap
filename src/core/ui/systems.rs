use crate::core::player::Player;
use crate::core::resources::ImageIds;
use bevy::prelude::*;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::widget_text::RichText;
use bevy_egui::egui::widgets::Image;
use bevy_egui::{EguiContexts, egui};
use crate::core::game_settings::GameSettings;

pub fn top_panel(
    mut contexts: EguiContexts,
    game_settings: Res<GameSettings>,
    player: Res<Player>,
    images: Res<ImageIds>,
    window: Single<&Window>,
) {
    let window_width = window.width();
    let window_height = window.height();

    let height = window_height * 0.1;
    egui::TopBottomPanel::top("top_panel")
        .exact_height(height)
        .show_separator_line(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(window_width * 0.2);
                ui.add(Image::new(SizedTexture::new(
                    images.get("cash"),
                    [height * 0.5, height * 0.5],
                )));
                ui.label(RichText::new(player.cash.to_string()).size(height * 0.5));
                
                ui.add_space(window_width * 0.2);
                
                ui.add(Image::new(SizedTexture::new(
                    images.get("clock"),
                    [height * 0.5, height * 0.5],
                )));
                ui.label(RichText::new(game_settings.date.format("%d-%m-%Y").to_string()).size(height * 0.5));
            });
        });
}
