use crate::core::constants::{GREEN, LEFT_LABEL_FRAC, TOP_LABEL_FRAC};
use crate::core::factors::Factor;
use crate::core::global_economy::GlobalEconomy;
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::states::GameState;
use crate::core::ui::utils::CustomUi;
use crate::utils::NameFromEnum;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::widget_text::RichText;
use bevy_egui::egui::{
    Align, CentralPanel, Color32, Frame, Layout, Margin, SidePanel, TopBottomPanel,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq)]
pub enum Tab {
    #[default]
    Home,
    Stocks,
    Bonds,
    Crypto,
    Commodities,
    Credit,
    Policies,
}

impl Tab {
    pub fn emoji(&self) -> &str {
        match self {
            Tab::Home => "🏠",
            Tab::Stocks => "📈",
            Tab::Bonds => "💵",
            Tab::Crypto => "💰",
            Tab::Commodities => "💎",
            Tab::Credit => "💳",
            Tab::Policies => "📜",
        }
    }
}

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq)]
pub enum CreditTab {
    #[default]
    Bank,
    AlternativeLoanProvider,
}

#[derive(Resource, Clone, Debug, Default)]
pub struct UiState {
    pub tab: Tab,
    pub credit_tab: CreditTab,
}

pub fn top_panel(
    mut contexts: EguiContexts,
    economy: Res<GlobalEconomy>,
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
                    economy.economy.to_string(),
                    economy.economy.description(),
                    images.get(economy.economy.image()),
                    Color32::WHITE,
                    &window,
                );

                ui.add_block(
                    economy.inflation.to_string(),
                    economy.inflation.description(),
                    images.get(economy.inflation.image()),
                    Color32::WHITE,
                    &window,
                );

                ui.add_block(
                    economy.interest.to_string(),
                    economy.interest.description(),
                    images.get(economy.interest.image()),
                    Color32::WHITE,
                    &window,
                );

                ui.add_space(window.width() * 0.04);

                ui.add_block(
                    economy.date.format("%d-%m-%Y").to_string(),
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
    mut ui_state: ResMut<UiState>,
    window: Single<&Window>,
) {
    let width = window.width() * LEFT_LABEL_FRAC;

    SidePanel::left("left_panel")
        .exact_width(width)
        .show_separator_line(false)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(Layout::top_down_justified(Align::Min), |ui| {
                ui.add_space(window.height() * 0.12);

                for tab in Tab::iter() {
                    ui.selectable_value(
                        &mut ui_state.tab,
                        tab,
                        RichText::new(format!("{}  {}", tab.emoji(), tab.to_name()))
                            .size(width * 0.1),
                    );
                }
            });
        });
}

pub fn central_panel(
    mut contexts: EguiContexts,
    ui_state: Res<UiState>,
    window: Single<&Window>,
) {
    CentralPanel::default()
        // .frame(Frame::default().inner_margin(Margin::same(10)))
        .show(contexts.ctx_mut(), |ui| {
            ui.add_space(window.height() * 0.08);

            match ui_state.tab {
                Tab::Home => {
                    ui.heading("Home");
                }
                Tab::Stocks => {
                    ui.heading("Stocks");
                }
                Tab::Bonds => {
                    ui.heading("Bonds");
                }
                Tab::Crypto => {
                    ui.heading("Crypto");
                }
                Tab::Commodities => {
                    ui.heading("Commodities");
                }
                Tab::Credit => {
                    ui.label(RichText::new("Credit")
                        .size(window.height() * 0.08));
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.add_space(window.width() * 0.08);

                        for tab in CreditTab::iter() {
                            ui.selectable_value(
                                &mut ui_state.credit_tab,
                                tab,
                                RichText::new(format!("{}  {}", tab.emoji(), tab.to_name()))
                                    .size(window.width() * 0.1),
                            );
                        }
                    });
                }
                Tab::Policies => {
                    ui.heading("Policies");
                }
            }
        });
}
