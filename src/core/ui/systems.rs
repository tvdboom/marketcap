use crate::core::constants::{GREEN, LEFT_LABEL_FRAC, TOP_LABEL_FRAC};
use crate::core::game_params::GameParams;
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::states::GameState;
use crate::utils::NameFromEnum;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::widget_text::RichText;
use bevy_egui::egui::widgets::Image;
use bevy_egui::egui::{Align, Color32, Layout, SidePanel, TopBottomPanel};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, PartialEq)]
pub enum Tab {
    Home,
    Stocks,
    Bonds,
    Crypto,
    Credit,
}

impl Tab {
    pub fn emoji(&self) -> &str {
        match self {
            Tab::Home => "🏠",
            Tab::Stocks => "📈",
            Tab::Bonds => "💵",
            Tab::Crypto => "💰",
            Tab::Credit => "💳",
        }
    }
}

pub fn top_panel(
    mut contexts: EguiContexts,
    game_params: Res<GameParams>,
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
                ui.add_space(window_width * 0.02);

                ui.add(Image::new(SizedTexture::new(
                    images.get("logo"),
                    [height * 0.5, height * 0.5],
                )));
                ui.label(
                    RichText::new(player.market_cap().to_string())
                        .size(height * 0.5)
                        .color(GREEN),
                );

                ui.add_space(window_width * 0.01);

                ui.add(Image::new(SizedTexture::new(
                    images.get("cash"),
                    [height * 0.5, height * 0.5],
                )));
                ui.label(
                    RichText::new(player.cash.to_string())
                        .size(height * 0.5)
                        .color(GREEN),
                );

                ui.add_space(window_width * 0.01);

                ui.add(Image::new(SizedTexture::new(
                    images.get("netflow"),
                    [height * 0.5, height * 0.5],
                )));
                ui.label(
                    RichText::new(format!("{:+}", player.netflow()))
                        .size(height * 0.5)
                        .color(match player.netflow() {
                            n if n < 0 => Color32::RED,
                            0 => Color32::WHITE,
                            _ => GREEN,
                        }),
                );

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add_space(window_width * 0.02);

                    ui.label(
                        RichText::new(game_params.date.format("%d-%m-%Y").to_string())
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

                    ui.add_space(window_width * 0.01);

                    ui.label(
                        RichText::new(format!("{:.1}%", game_params.interest_rate.current()))
                            .size(height * 0.5),
                    );
                    ui.add(Image::new(SizedTexture::new(
                        images.get("interest-rate"),
                        [height * 0.5, height * 0.5],
                    )))
                    .on_hover_text(game_params.interest_rate.description());

                    ui.add_space(window_width * 0.01);

                    ui.label(
                        RichText::new(format!("{:.0}", game_params.economic_factor.current()))
                            .size(height * 0.5),
                    );
                    ui.add(Image::new(SizedTexture::new(
                        images.get("global-economy"),
                        [height * 0.5, height * 0.5],
                    )))
                    .on_hover_text(game_params.economic_factor.description());
                });
            });
        });
}

pub fn left_panel(
    mut contexts: EguiContexts,
    mut game_params: ResMut<GameParams>,
    window: Single<&Window>,
) {
    let window_width = window.width();
    let window_height = window.height();

    let width = window_width * LEFT_LABEL_FRAC;
    SidePanel::left("left_panel")
        .exact_width(width)
        .show_separator_line(false)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(Layout::top_down_justified(Align::Min), |ui| {
                ui.add_space(window_height * 0.1);

                for tab in Tab::iter() {
                    ui.selectable_value(
                        &mut game_params.tab,
                        tab,
                        RichText::new(format!("{}  {}", tab.emoji(), tab.to_name()))
                            .size(width * 0.12),
                    );
                }
            });
        });
}
