use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::epaint::text::{FontInsert, FontPriority, InsertFontFamily};
use bevy_egui::egui::widget_text::RichText;
use bevy_egui::egui::{
    Align, CentralPanel, Color32, FontData, FontFamily, Frame, Layout, Margin, SidePanel,
    TopBottomPanel,
};
use strum::IntoEnumIterator;

use crate::core::assets::WorldAssets;
use crate::core::constants::{DATE_FORMAT, GREEN, LEFT_LABEL_FRAC, TOP_LABEL_FRAC, WIDTH};
use crate::core::factors::Factor;
use crate::core::game_settings::GameSettings;
use crate::core::global_economy::GlobalEconomy;
use crate::core::messages::MessageEv;
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::states::GameState;
use crate::core::ui::bonds::bonds_panel;
use crate::core::ui::commodities::commodities_panel;
use crate::core::ui::credit::credit_panel;
use crate::core::ui::forex::forex_panel;
use crate::core::ui::state::{Tab, UiState};
use crate::core::ui::utils::{CustomUi, TextSizes};
use crate::utils::{NameFromEnum, format_number};

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
                    GREEN,
                    images.get("enterprise"),
                    format!(
                        "The enterprise value is a comprehensive measure of a company's total \
                        worth. This includes any kind of assets, investments and cash deposits, \
                        minus debts.\n\n\
                        In the game, the enterprise value represents a measure of the success \
                        of the player. If the enterprise value drops below zero, the company \
                        goes bankrupt and the game is lost.\n\n\
                        Cash: {:+.0}\nCommodities: {:+.0}\nDebt: {:-.0}\n\
                        -------------------\nEnterprise value: {:.0}",
                        player.cash,
                        player
                            .commodities
                            .iter()
                            .map(|s| s.amount as f32 * economy.get_commodity(&s.name).current())
                            .sum::<f32>()
                            .max(0.),
                        player
                            .loans
                            .iter()
                            .map(|l| l.outstanding)
                            .sum::<f32>()
                            .max(0.),
                        player.enterprise_value(&economy).floor(),
                    ),
                    None,
                    &window,
                );

                ui.add_space(window.width() * 0.01);

                ui.add_factor(
                    "Cash",
                    player.cash.to_string(),
                    GREEN,
                    images.get(player.cash.image()),
                    player.cash.description(),
                    None,
                    &window,
                );

                ui.add_space(window.width() * 0.01);

                let netflow = player.netflow(&economy).floor();
                ui.add_factor(
                    "Net flow",
                    format!("{netflow:+}"),
                    match netflow {
                        n if n <= -1. => Color32::RED,
                        n if n >= 1. => GREEN,
                        _ => text_color,
                    },
                    images.get("netflow"),
                    format!(
                        "The net flow represents the total financial movement at the end of \
                        each month, calculated as income minus debt repayments and expenses. \
                        It shows whether the player will gain or lose money this month.\n\n\
                        Inflow: {:+.0}\nOutflow: {:-.0}",
                        player.inflow().floor(),
                        player.outflow(&economy).floor(),
                    ),
                    None,
                    &window,
                );

                ui.add_space(window.width() * 0.01);

                ui.add_factor(
                    "Credit score",
                    player.credit_score.to_string(),
                    match player.credit_score.current() {
                        n if n < 30. => Color32::RED,
                        n if n > 70. => GREEN,
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
                    economy.economy.to_string(),
                    text_color,
                    images.get(economy.economy.image()),
                    economy.economy.description(),
                    Some(&economy.economy.0),
                    &window,
                );

                ui.add_space(window.width() * 0.01);

                ui.add_factor(
                    "Inflation",
                    economy.inflation.to_string(),
                    text_color,
                    images.get(economy.inflation.image()),
                    economy.inflation.description(),
                    Some(&economy.inflation.0),
                    &window,
                );

                ui.add_space(window.width() * 0.01);

                ui.add_factor(
                    "Global interest rate",
                    economy.interest.to_string(),
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

pub fn left_panel(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    window: Single<&Window>,
) {
    SidePanel::left("left_panel")
        .exact_width(window.width().min(1.2 * WIDTH) * LEFT_LABEL_FRAC)
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
    mut messages: EventWriter<MessageEv>,
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
        .show(contexts.ctx_mut(), |ui| match ui_state.tab {
            Tab::Overview => {
                ui.heading("Home");
            },
            Tab::Stocks => {
                ui.heading("Stocks");
            },
            Tab::Bonds => bonds_panel(ui, &mut ui_state, &window),
            Tab::Forex => forex_panel(ui, &mut ui_state, &window),
            Tab::Crypto => forex_panel(ui, &mut ui_state, &window),
            Tab::Commodities => commodities_panel(ui, &mut ui_state, &economy, &images, &window),
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
            },
        });
}

pub fn check_keys(keyboard: Res<ButtonInput<KeyCode>>, mut ui_state: ResMut<UiState>) {
    if keyboard.just_pressed(KeyCode::KeyO) {
        ui_state.tab = Tab::Overview;
    }
}
