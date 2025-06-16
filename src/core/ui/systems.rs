use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy_egui::EguiContexts;
use bevy_egui::egui::epaint::text::{FontInsert, FontPriority, InsertFontFamily};
use bevy_egui::egui::{
    Align, CentralPanel, Color32, FontData, FontFamily, Frame, Layout, Margin, SidePanel,
    TopBottomPanel,
};
use strum::IntoEnumIterator;

use crate::core::assets::WorldAssets;
use crate::core::constants::{CUSTOM_GREEN, DATE_FORMAT, LEFT_LABEL_FRAC, TOP_LABEL_FRAC, WIDTH};
use crate::core::factors::Factor;
use crate::core::game_settings::GameSettings;
use crate::core::global_economy::GlobalEconomy;
use crate::core::messages::MessageEv;
use crate::core::player::Player;
use crate::core::resources::{ImageIds, KeyMap};
use crate::core::states::GameState;
use crate::core::ui::bonds::bonds_panel;
use crate::core::ui::commodities::commodities_panel;
use crate::core::ui::credit::credit_panel;
use crate::core::ui::crypto::crypto_panel;
use crate::core::ui::forex::forex_panel;
use crate::core::ui::overview::overview_panel;
use crate::core::ui::state::{Tab, UiState};
use crate::core::ui::utils::CustomUi;
use crate::utils::{NameFromEnum, Round1, format_number};

pub fn set_egui_style(
    mut contexts: EguiContexts,
    game_settings: Res<GameSettings>,
    window: Single<&Window>,
) {
    let context = contexts.ctx_mut();

    context.options_mut(|options| {
        options.line_scroll_speed = 100.;
    });

    context.set_style(
        game_settings
            .theme
            .get()
            .custom_style(window.width(), window.height()),
    );

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

pub fn on_resize_system(
    mut contexts: EguiContexts,
    game_settings: Res<GameSettings>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for ev in resize_reader.read() {
        contexts
            .ctx_mut()
            .set_style(game_settings.theme.get().custom_style(ev.width, ev.height));
    }
}

pub fn top_panel(
    mut contexts: EguiContexts,
    economy: Res<GlobalEconomy>,
    player: Res<Player>,
    game_state: Res<State<GameState>>,
    game_settings: Res<GameSettings>,
    images: Res<ImageIds>,
    window: Single<&Window>,
) {
    let text_color = game_settings
        .theme
        .get()
        .fg_primary_text_color_visuals()
        .unwrap();

    TopBottomPanel::top("top_panel")
        .exact_height(window.height() * TOP_LABEL_FRAC)
        .show_separator_line(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(window.width() * 0.095);

                ui.add_factor(
                    "Enterprise value",
                    format_number(player.enterprise_value(&economy)),
                    CUSTOM_GREEN,
                    images.get("enterprise"),
                    format!(
                        "The enterprise value is a comprehensive measure of a company's total \
                        worth. This includes any kind of assets, investments and cash deposits, \
                        minus debts.\n\n\
                        In the game, the enterprise value represents a measure of the success \
                        of the player. If the enterprise value drops below zero, the company \
                        goes bankrupt and the game is lost.\n\n\
                        Cash: {}\n\
                        Collateral: {}\n\
                        Commodities: {}\n\
                        Crypto: {}\n\
                        Debt: {}\n\
                        -------------------\n\
                        Enterprise value: {}",
                        player.cash.current().signed(),
                        player.collateral.signed(),
                        player
                            .commodities()
                            .iter()
                            .map(|o| o.amount as f32 * economy.get_current(&o.kind))
                            .sum::<f32>()
                            .signed(),
                        player
                            .crypto()
                            .iter()
                            .map(|o| o.amount as f32 * economy.get_current(&o.kind))
                            .sum::<f32>()
                            .signed(),
                        player
                            .loans
                            .iter()
                            .map(|l| -l.outstanding)
                            .sum::<f32>()
                            .signed(),
                        player.enterprise_value(&economy) as i32,
                    ),
                    None,
                    &window,
                );

                ui.add_space(window.width() * 0.01);

                ui.add_factor(
                    "Cash",
                    (player.cash.current() as u32).to_string(),
                    CUSTOM_GREEN,
                    images.get(player.cash.image()),
                    player.cash.description(),
                    None,
                    &window,
                );

                ui.add_space(window.width() * 0.01);

                let netflow = player.netflow(&economy).floor();
                ui.add_factor(
                    "Net flow",
                    netflow.signed(),
                    match netflow {
                        n if n <= -1. => Color32::RED,
                        n if n >= 1. => CUSTOM_GREEN,
                        _ => text_color,
                    },
                    images.get("netflow"),
                    format!(
                        "The net flow represents the total financial movement at the end of \
                        each month, calculated as income minus debt repayments and expenses. \
                        It shows whether the player will gain or lose money this month.\n\n\
                        Cash interest: {}\n\
                        ------------------------\n\
                        Inflow: {}\n\n\
                        Storage costs: {}\n\
                        Loan installments: {}\n\
                        ------------------------\n\
                        Outflow: {}",
                        player.cash.accumulated_interest as u32,
                        player.inflow().signed(),
                        player.storage_costs(&economy) as u32,
                        player.loan_installments() as u32,
                        (-player.outflow(&economy)).signed(),
                    ),
                    None,
                    &window,
                );

                ui.add_space(window.width() * 0.01);

                ui.add_factor(
                    "Credit score",
                    player.credit_score.current().to_string(),
                    match player.credit_score.current() {
                        n if n < 30. => Color32::RED,
                        n if n > 70. => CUSTOM_GREEN,
                        _ => text_color,
                    },
                    images.get(player.credit_score.image()),
                    player.credit_score.description(),
                    None,
                    &window,
                );

                ui.add_space(window.width() * 0.04);

                ui.add_factor(
                    "Global economic factor",
                    (economy.economy.current() as u8).to_string(),
                    text_color,
                    images.get(economy.economy.image()),
                    economy.economy.description(),
                    Some(&economy.economy.0),
                    &window,
                );

                ui.add_space(window.width() * 0.01);

                ui.add_factor(
                    "Inflation",
                    format!("{:.1}%", economy.inflation.current()),
                    text_color,
                    images.get(economy.inflation.image()),
                    economy.inflation.description(),
                    Some(&economy.inflation.0),
                    &window,
                );

                ui.add_space(window.width() * 0.01);

                ui.add_factor(
                    "Global interest rate",
                    format!("{:.1}%", economy.interest.current()),
                    text_color,
                    images.get(economy.interest.image()),
                    economy.interest.description(),
                    Some(&economy.interest.rate),
                    &window,
                );

                ui.add_space(window.width() * 0.04);

                ui.add_factor(
                    "Current date",
                    economy.date.format(DATE_FORMAT).to_string(),
                    text_color,
                    images.get(if *game_state.get() == GameState::Running {
                        "time"
                    } else {
                        "time-paused"
                    }),
                    "Income and expenses are paid every first day of the month. Interest \
                    (for example on cash) is calculated daily.\n\n\
                    Use the space key to pause/unpause the time."
                        .to_string(),
                    None,
                    &window,
                );
            });
        });
}

pub fn left_panel(mut contexts: EguiContexts, mut state: ResMut<UiState>, window: Single<&Window>) {
    SidePanel::left("left_panel")
        .exact_width(window.width().min(1.2 * WIDTH) * LEFT_LABEL_FRAC)
        .show_separator_line(false)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(Layout::top_down_justified(Align::Min), |ui| {
                ui.add_space(window.height() * 0.14);

                for tab in Tab::iter() {
                    ui.selectable_value(
                        &mut state.tab,
                        tab,
                        format!("{}  {}", tab.emoji(), tab.to_name()),
                    );
                }
            });
        });
}

pub fn central_panel(
    mut contexts: EguiContexts,
    mut state: ResMut<UiState>,
    game_settings: Res<GameSettings>,
    economy: Res<GlobalEconomy>,
    mut player: ResMut<Player>,
    mut message: EventWriter<MessageEv>,
    images: Res<ImageIds>,
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
        .show(contexts.ctx_mut(), |ui| match state.tab {
            Tab::Overview => {
                overview_panel(ui, &mut state, &economy, &mut player, &mut message, &window)
            },
            Tab::Stocks => {
                ui.heading("Stocks");
            },
            Tab::Bonds => bonds_panel(ui, &mut state, &economy, &player, &images, &window),
            Tab::Forex => forex_panel(ui, &mut state, &window),
            Tab::Crypto => crypto_panel(ui, &mut state, &economy, &player, &images, &window),
            Tab::Commodities => {
                commodities_panel(ui, &mut state, &economy, &player, &images, &window)
            },
            Tab::Credit => {
                credit_panel(ui, &mut state, &economy, &mut player, &mut message, &window)
            },
            Tab::Policies => {
                ui.heading("Policies");
            },
        });
}

pub fn check_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<UiState>,
    mut key_map: ResMut<KeyMap>,
) {
    if keyboard.just_pressed(KeyCode::KeyO) {
        state.tab = Tab::Overview;
        state.modal = None;
    }

    for key in keyboard.get_just_released() {
        let digit = match key {
            KeyCode::Digit0 => Some(0),
            KeyCode::Digit1 => Some(1),
            KeyCode::Digit2 => Some(2),
            KeyCode::Digit3 => Some(3),
            KeyCode::Digit4 => Some(4),
            KeyCode::Digit5 => Some(5),
            KeyCode::Digit6 => Some(6),
            KeyCode::Digit7 => Some(7),
            KeyCode::Digit8 => Some(8),
            KeyCode::Digit9 => Some(9),
            _ => None,
        };

        if let Some(index) = digit {
            if keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
                if let Some(instrument) = state.modal {
                    key_map.0.insert(index, instrument);
                }
            } else if let Some(map) = key_map.0.get(&index) {
                state.modal = Some(*map);
            }
        }
    }
}
