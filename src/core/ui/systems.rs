use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy_egui::EguiContexts;
use bevy_egui::egui::epaint::text::{FontInsert, FontPriority, InsertFontFamily};
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{
    Align, CentralPanel, Color32, FontData, FontFamily, Frame, Id, Layout, Margin, Modal, RichText,
    SidePanel, TopBottomPanel, UiBuilder,
};
use chrono::Datelike;
use strum::IntoEnumIterator;

use crate::core::assets::WorldAssets;
use crate::core::constants::{
    CUSTOM_GREEN, DATE_FORMAT, HEIGHT, LEFT_LABEL_FRAC, TOP_LABEL_FRAC, WIDTH,
};
use crate::core::factors::Factor;
use crate::core::game_settings::GameSettings;
use crate::core::global_economy::GlobalEconomy;
use crate::core::messages::MessageEv;
use crate::core::player::Player;
use crate::core::research::TechName;
use crate::core::resources::ImageIds;
use crate::core::states::GameState;
use crate::core::ui::bonds::bonds_panel;
use crate::core::ui::commodities::commodities_panel;
use crate::core::ui::credit::credit_panel;
use crate::core::ui::crypto::crypto_panel;
use crate::core::ui::forex::forex_panel;
use crate::core::ui::overview::overview_panel;
use crate::core::ui::policies::policies_panel;
use crate::core::ui::research::research_panel;
use crate::core::ui::state::{Tab, UiState};
use crate::core::ui::stocks::stock_panel;
use crate::core::ui::utils::CustomUi;
use crate::utils::{EnhFloat, NameFromEnum};

pub fn set_egui_style(
    mut contexts: EguiContexts,
    game_settings: Res<GameSettings>,
    window: Single<&Window>,
) {
    let context = contexts.ctx_mut();

    context.options_mut(|options| {
        options.line_scroll_speed = 100.;
    });

    context.set_style(game_settings.theme.get().custom_style(window.width(), window.height()));

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
        contexts.ctx_mut().set_style(game_settings.theme.get().custom_style(ev.width, ev.height));
    }
}

pub fn start_game(
    mut contexts: EguiContexts,
    mut next_game_state: ResMut<NextState<GameState>>,
    images: Res<ImageIds>,
    window: Single<&Window>,
) {
    Modal::new(Id::new("start_game")).frame(Frame::default().inner_margin(0.)).show(
        contexts.ctx_mut(),
        |ui| {
            ui.set_width((window.width() * 0.55).max(WIDTH * 0.55));
            ui.set_height((window.height() * 0.6).max(HEIGHT * 0.6));

            let response = ui.add(bevy_egui::egui::Image::new(SizedTexture::new(
                images.get("trading"),
                ui.available_size(),
            )));

            ui.allocate_new_ui(UiBuilder::new().max_rect(response.rect), |ui| {
                ui.add_space(window.height() * 0.02);
                ui.vertical_centered(|ui| {
                    ui.heading("Welcome to MarketCap");
                });

                ui.add_space(window.height() * 0.02);

                Frame::default()
                    .fill(Color32::from_black_alpha(120))
                    .outer_margin(Margin::same(20))
                    .inner_margin(Margin::same(10))
                    .corner_radius(4.)
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(
                                "You are the newly minted CEO of Trident Capital, a scrappy asset \
                                manager with ambitions to dominate global markets. Backed by a mix of \
                                ruthless investors and young in-house talent, you're entering the most \
                                cutthroat financial landscape seen to date.\n\n\
                                Your mission is simple: grow your Assets Under Management (AUM). Trade \
                                in stocks, bonds, forex, commodities, cryptos and derivatives. Shape \
                                politics, rewrite economic policy and tilt the balance of power. Navigate \
                                corporate scandals, macro shocks and global conflict - all while charming \
                                clients and outwitting regulators. Your company isn't just about market \
                                plays - it's a political force.\n\n\
                                In this world, success is measured in billions. Are you ready to rewrite \
                                history with your portfolio? Let the markets open!",
                            )
                            .color(Color32::WHITE),
                        );
                    });

                ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                    ui.add_space(window.height() * 0.05);

                    ui.horizontal(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                            ui.add_space(window.width() * 0.02);

                            if ui.add_modal_button("Start game", &window).clicked() {
                                next_game_state.set(GameState::Running);
                            }
                        });
                    });
                });
            });
        },
    );
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
    let text_color = game_settings.theme.get().fg_primary_text_color_visuals().unwrap();

    TopBottomPanel::top("top_panel")
        .exact_height(window.height() * TOP_LABEL_FRAC)
        .show_separator_line(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(window.width() * 0.06);

                ui.add_factor(
                    "Assets Under Management",
                    player.aum(&economy).format(),
                    CUSTOM_GREEN,
                    images.get("aum"),
                    format!(
                        "The Assets Under Management (AUM) refers to the total market value of all \
                        financial instruments and cash deposits (minus debts) that an asset manager \
                        oversees.\n\n\
                        In the game, the AUM represents a measure of the success of the player. If \
                        the AUM drops to zero, the company goes bankrupt and the game is lost.\n\n\
                        Cash: {}\n\
                        Stocks: {}\n\
                        Commodities: {}\n\
                        Crypto: {}\n\
                        Term loan debt: {}\n\
                        Margin loan debt: {}\n\
                        ------------------------\n\
                        AUM: {}",
                        player.cash.current().signed(),
                        player
                            .stocks()
                            .iter()
                            .map(|o| o.amount as f32 * economy.get_price(&o.kind))
                            .sum::<f32>()
                            .signed(),
                        player
                            .commodities()
                            .iter()
                            .map(|o| o.amount as f32 * economy.get_price(&o.kind))
                            .sum::<f32>()
                            .signed(),
                        player
                            .crypto()
                            .iter()
                            .map(|o| o.amount as f32 * economy.get_price(&o.kind))
                            .sum::<f32>()
                            .signed(),
                        (-player.term_loan_debt()).signed(),
                        (-player.margin_loan_debt()).signed(),
                        player.aum(&economy) as i32,
                    ),
                    None,
                    economy.date,
                    &window,
                );

                ui.add_space(window.width() * 0.01);

                ui.add_factor(
                    "Cash",
                    player.cash.current().format(),
                    match player.cash.current() {
                        n if n <= -1. => Color32::RED,
                        n if n >= 1. => CUSTOM_GREEN,
                        _ => text_color,
                    },
                    images.get(player.cash.image()),
                    player.cash.description(),
                    None,
                    economy.date,
                    &window,
                );

                ui.add_space(window.width() * 0.01);

                let netflow = player.netflow(&economy);
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
                        It shows whether the player will gain or lose money this month. Note \
                        that dividend and coupon payments are an approximation. The real values \
                        may vary.\n\n\
                        Cash interest: {}\n\
                        Dividend payments: {}\n\
                        Coupon payments: {}\n\
                        Storage costs: {}\n\
                        Term loan installments: {}\n\
                        Margin loan interest: {}\n\
                        Research costs: {}\n\
                        -----------------------------\n\
                        Net flow: {}",
                        player.cash.accumulated_interest.signed(),
                        (if economy.date.month() % 3 == 0 { player.approx_dividends(&economy) } else { 0. }).signed(),
                        (if economy.date.month() % 6 == 0 { player.approx_coupons(&economy) } else { 0. }).signed(),
                        (-player.storage_costs(&economy)).signed(),
                        (-player.loan_installments()).signed(),
                        (-player.short_sell_interest()).signed(),
                        (-player.research.costs()).signed(),
                        player.netflow(&economy).signed(),
                    ),
                    None,
                    economy.date,
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
                    economy.date,
                    &window,
                );

                ui.add_space(window.width() * 0.01);

                ui.add_factor(
                    "Influence",
                    player.influence.current().floor().to_string(),
                    match player.influence.current() {
                        n if n > 0. => CUSTOM_GREEN,
                        _ => text_color,
                    },
                    images.get(player.influence.image()),
                    player.influence.description(),
                    None,
                    economy.date,
                    &window,
                );

                ui.add_space(window.width() * 0.04);

                ui.add_factor(
                    "Global economy",
                    (economy.economy.current() as u8).to_string(),
                    text_color,
                    images.get(economy.economy.image()),
                    economy.economy.description(),
                    Some(&economy.economy.values),
                    economy.date,
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
                    economy.date,
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
                    economy.date,
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
                    economy.date,
                    &window,
                );
            });
        });
}

pub fn left_panel(
    mut contexts: EguiContexts,
    player: Res<Player>,
    mut state: ResMut<UiState>,
    window: Single<&Window>,
) {
    SidePanel::left("left_panel")
        .exact_width(window.width().min(1.2 * WIDTH) * LEFT_LABEL_FRAC)
        .show_separator_line(false)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(Layout::top_down_justified(Align::Min), |ui| {
                ui.add_space(window.height() * 0.14);

                for tab in Tab::iter().filter(|t| match t {
                    Tab::Forex => player.has_tech(&TechName::ForeignExchange),
                    Tab::Commodities => player.has_tech(&TechName::Commodities),
                    Tab::Crypto => player.has_tech(&TechName::Cryptocurrencies),
                    _ => true,
                }) {
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
    mut economy: ResMut<GlobalEconomy>,
    mut player: ResMut<Player>,
    mut message: EventWriter<MessageEv>,
    images: Res<ImageIds>,
    window: Single<&Window>,
) {
    CentralPanel::default()
        .frame(
            Frame::new().fill(game_settings.theme.get().bg_primary_color_visuals()).inner_margin(
                Margin {
                    left: 60,
                    right: 60,
                    top: 40,
                    bottom: 40,
                },
            ),
        )
        .show(contexts.ctx_mut(), |ui| match state.tab {
            Tab::Overview => overview_panel(
                ui,
                &mut state,
                &economy,
                &mut player,
                &mut message,
                &images,
                &window,
            ),
            Tab::Stocks => stock_panel(ui, &mut state, &economy, &player, &images, &window),
            Tab::Bonds => bonds_panel(ui, &mut state, &economy, &player, &images, &window),
            Tab::Forex => forex_panel(ui, &mut state, &economy, &player, &images, &window),
            Tab::Crypto => crypto_panel(ui, &mut state, &economy, &player, &images, &window),
            Tab::Commodities => {
                commodities_panel(ui, &mut state, &economy, &player, &images, &window)
            },
            Tab::Credit => {
                credit_panel(ui, &mut state, &mut economy, &mut player, &mut message, &window)
            },
            Tab::Policies => {
                policies_panel(ui, &mut state, &economy, &mut player, &mut message, &window);
            },
            Tab::Research => research_panel(ui, &mut player, &mut message, &images, &window),
        });
}

pub fn check_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<UiState>,
    mut player: ResMut<Player>,
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
                    player.favourites.insert(index, instrument);
                }
            } else if let Some(map) = player.favourites.get(&index) {
                state.modal = Some(*map);
            }
        }
    }
}
