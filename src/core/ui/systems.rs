use crate::core::attributes::attribute::Attribute;
use crate::core::constants::{GREEN, LEFT_LABEL_FRAC, TOP_LABEL_FRAC};
use crate::core::game_params::GameParams;
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::states::GameState;
use crate::core::ui::utils::CustomUi;
use crate::utils::NameFromEnum;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::widget_text::RichText;
use bevy_egui::egui::{Align, Color32, Layout, SidePanel, TopBottomPanel};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq)]
pub enum Tab {
    #[default]
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
    TopBottomPanel::top("top_panel")
        .exact_height(window.height() * TOP_LABEL_FRAC)
        .show_separator_line(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(window.width() * 0.12);

                ui.add_block(
                    format!("{:.0}", player.enterprise_value().floor()),
                    format!(
                        "Enterprise value\n\n\
                        The enterprise value is a comprehensive measure of a company's total \
                        worth. This includes any kind of assets, investments and cash deposits, \
                        minus debts.\n\n\
                        In the game, the enterprise value represents a measure of the success \
                        of the player. If the enterprise value drops below zero, the company \
                        goes bankrupt and the game is lost.\n\n\
                        Cash: {}",
                        player.cash
                    ),
                    images.get("enterprise"),
                    GREEN,
                    &window,
                );

                ui.add_block(
                    player.cash.to_string(),
                    player.cash.description(),
                    images.get(player.cash.image()),
                    GREEN,
                    &window,
                );

                ui.add_block(
                    format!("{:+.0}", player.netflow().floor()),
                    format!(
                        "Net flow\n\n\
                        The net flow represents the total financial movement at the end of \
                        each month, calculated as income minus debt repayments and expenses. \
                        It shows whether the player will gain or lose money this month.\n\n\
                        Inflow: {:+.0}\nOutflow: {:+.0}",
                        player.inflow().floor(),
                        player.outflow().floor(),
                    ),
                    images.get("netflow"),
                    match player.netflow() {
                        n if n <= -1. => Color32::RED,
                        n if n >= 1. => GREEN,
                        _ => Color32::WHITE,
                    },
                    &window,
                );

                ui.add_space(window.width() * 0.04);

                ui.add_block(
                    game_params.economic_factor.to_string(),
                    game_params.economic_factor.description(),
                    images.get(game_params.economic_factor.image()),
                    Color32::WHITE,
                    &window,
                );

                ui.add_block(
                    game_params.interest_rate.to_string(),
                    game_params.interest_rate.description(),
                    images.get(game_params.interest_rate.image()),
                    Color32::WHITE,
                    &window,
                );

                ui.add_block(
                    game_params.date.format("%d-%m-%Y").to_string(),
                    "Current date\n\n\
                        Income and expenses are paid every last day of the month. Interests are \
                        calculated daily.\n\n\
                        Use the space key to pause/unpause the time.",
                    images.get(if *game_state.get() == GameState::Running {
                        "time"
                    } else {
                        "time-paused"
                    }),
                    Color32::WHITE,
                    &window,
                );
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
