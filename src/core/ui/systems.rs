use crate::core::assets::WorldAssets;
use crate::core::constants::{DATE_FORMAT, GREEN, LEFT_LABEL_FRAC, TOP_LABEL_FRAC};
use crate::core::factors::Factor;
use crate::core::game_settings::GameSettings;
use crate::core::global_economy::GlobalEconomy;
use crate::core::messages::Messages;
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::states::GameState;
use crate::core::ui::bonds::bonds_panel;
use crate::core::ui::credit::credit_panel;
use crate::core::ui::currencies::currencies_panel;
use crate::core::ui::state::{Tab, UiState};
use crate::core::ui::utils::{CustomUi, TextSizes};
use crate::utils::NameFromEnum;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::epaint::text::{FontInsert, FontPriority, InsertFontFamily};
use bevy_egui::egui::widget_text::RichText;
use bevy_egui::egui::{
    Align, CentralPanel, Color32, FontData, FontFamily, Frame, Layout, Margin, SidePanel,
    TopBottomPanel,
};
use strum::IntoEnumIterator;

pub fn set_egui_style(mut contexts: EguiContexts, game_settings: Res<GameSettings>) {
    let context = contexts.ctx_mut();

    context.set_style(game_settings.theme.get().custom_style());

    context.add_font(FontInsert::new(
        "firamono",
        FontData::from_static(include_bytes!("../../../assets/fonts/FiraMono-Medium.ttf")),
        vec![
            InsertFontFamily {
                family: FontFamily::Proportional,
                priority: FontPriority::Highest,
            },
            InsertFontFamily {
                family: FontFamily::Monospace,
                priority: FontPriority::Lowest,
            },
        ],
    ));

    context.add_font(FontInsert::new(
        "firasans",
        FontData::from_static(include_bytes!("../../../assets/fonts/FiraSans-Bold.ttf")),
        vec![
            InsertFontFamily {
                family: FontFamily::Proportional,
                priority: FontPriority::Highest,
            },
            InsertFontFamily {
                family: FontFamily::Monospace,
                priority: FontPriority::Lowest,
            },
        ],
    ));
}

pub fn add_egui_images(
    mut contexts: EguiContexts,
    mut images: ResMut<ImageIds>,
    assets: Local<WorldAssets>,
) {
    for (k, v) in assets.images.iter() {
        let id = contexts.add_image(v.clone_weak());
        images.0.insert(k, id);
    }
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
                ui.add_space(window.width() * 0.095);

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
                        Cash: {}\nDebt: {:.0}",
                        player.cash,
                        player.loans.iter().map(|l| l.outstanding).sum::<f32>()
                    ),
                    images.get("enterprise"),
                    GREEN,
                    window.xxl_size(),
                );

                ui.add_space(window.width() * 0.01);

                ui.add_block(
                    player.cash.to_string(),
                    player.cash.description(),
                    images.get(player.cash.image()),
                    GREEN,
                    window.xxl_size(),
                );

                ui.add_space(window.width() * 0.01);

                ui.add_block(
                    format!("{:+.0}", player.netflow().floor()),
                    format!(
                        "Net flow\n\n\
                        The net flow represents the total financial movement at the end of \
                        each month, calculated as income minus debt repayments and expenses. \
                        It shows whether the player will gain or lose money this month.\n\n\
                        Inflow: {:+.0}\nOutflow: {:+.0}",
                        player.inflow().floor(),
                        -player.outflow().floor(),
                    ),
                    images.get("netflow"),
                    match player.netflow() {
                        n if n <= -1. => Color32::RED,
                        n if n >= 1. => GREEN,
                        _ => Color32::WHITE,
                    },
                    window.xxl_size(),
                );

                ui.add_space(window.width() * 0.01);

                ui.add_block(
                    player.credit_score.to_string(),
                    player.credit_score.description(),
                    images.get(player.credit_score.image()),
                    match player.credit_score.current() {
                        n if n < 30. => Color32::RED,
                        n if n > 70. => GREEN,
                        _ => Color32::WHITE,
                    },
                    window.xxl_size(),
                );

                ui.add_space(window.width() * 0.04);

                ui.add_block(
                    economy.economy.to_string(),
                    economy.economy.description(),
                    images.get(economy.economy.image()),
                    Color32::WHITE,
                    window.xxl_size(),
                );

                ui.add_space(window.width() * 0.01);

                ui.add_block(
                    economy.inflation.to_string(),
                    economy.inflation.description(),
                    images.get(economy.inflation.image()),
                    Color32::WHITE,
                    window.xxl_size(),
                );

                ui.add_space(window.width() * 0.01);

                ui.add_block(
                    economy.interest.to_string(),
                    economy.interest.description(),
                    images.get(economy.interest.image()),
                    Color32::WHITE,
                    window.xxl_size(),
                );

                ui.add_space(window.width() * 0.04);

                ui.add_block(
                    economy.date.format(DATE_FORMAT).to_string(),
                    "Current date\n\n\
                        Income and expenses are paid every first day of the month. Interest \
                        (for example on cash) is calculated daily.\n\n\
                        Use the space key to pause/unpause the time.",
                    images.get(if *game_state.get() == GameState::Running {
                        "time"
                    } else {
                        "time-paused"
                    }),
                    Color32::WHITE,
                    window.xxl_size(),
                );
            });
        });
}

pub fn left_panel(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    window: Single<&Window>,
) {
    SidePanel::left("left_panel")
        .exact_width(window.width() * LEFT_LABEL_FRAC)
        .show_separator_line(false)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(Layout::top_down_justified(Align::Min), |ui| {
                ui.add_space(window.height() * 0.14);

                for tab in Tab::iter() {
                    ui.selectable_value(
                        &mut ui_state.tab,
                        tab,
                        RichText::new(format!("{}  {}", tab.emoji(), tab.to_name()))
                            .size(window.xl_size()),
                    );
                }
            });
        });
}

pub fn central_panel(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    game_settings: Res<GameSettings>,
    economy: Res<GlobalEconomy>,
    mut player: ResMut<Player>,
    mut messages: ResMut<Messages>,
    window: Single<&Window>,
) {
    CentralPanel::default()
        .frame(
            Frame::new()
                .fill(game_settings.theme.get().bg_primary_color_visuals())
                .inner_margin(Margin {
                    left: 60,
                    right: 60,
                    top: 40,
                    bottom: 40,
                }),
        )
        .show(contexts.ctx_mut(), |ui| match ui_state.tab {
            Tab::Home => {
                ui.heading("Home");
            }
            Tab::Stocks => {
                ui.heading("Stocks");
            }
            Tab::Bonds => bonds_panel(
                ui,
                &mut ui_state,
                &mut player,
                &economy,
                &mut messages,
                &window,
            ),
            Tab::Currencies => currencies_panel(
                ui,
                &mut ui_state,
                &mut player,
                &economy,
                &mut messages,
                &window,
            ),
            Tab::Commodities => {
                ui.heading("Commodities");
            }
            Tab::Credit => credit_panel(
                ui,
                &mut ui_state,
                &mut player,
                &economy,
                &mut messages,
                &window,
            ),
            Tab::Policies => {
                ui.heading("Policies");
            }
        });
}

pub fn check_keys(keyboard: Res<ButtonInput<KeyCode>>, mut ui_state: ResMut<UiState>) {
    if keyboard.just_pressed(KeyCode::Digit1) {
        ui_state.tab = Tab::Home;
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        ui_state.tab = Tab::Stocks;
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        ui_state.tab = Tab::Bonds;
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        ui_state.tab = Tab::Currencies;
    } else if keyboard.just_pressed(KeyCode::Digit5) {
        ui_state.tab = Tab::Commodities;
    } else if keyboard.just_pressed(KeyCode::Digit6) {
        ui_state.tab = Tab::Credit;
    } else if keyboard.just_pressed(KeyCode::Digit7) {
        ui_state.tab = Tab::Policies;
    }
}
