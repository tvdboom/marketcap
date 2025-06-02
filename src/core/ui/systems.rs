use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::epaint::text::{FontInsert, FontPriority, InsertFontFamily};
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::widget_text::RichText;
use bevy_egui::egui::{
    Align, Button, CentralPanel, Color32, ComboBox, FontData, FontFamily, Frame, Id, Image, Layout,
    Margin, Modal, SidePanel, Sides, Slider, TopBottomPanel,
};
use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::core::assets::WorldAssets;
use crate::core::constants::{DATE_FORMAT, GREEN, LEFT_LABEL_FRAC, TOP_LABEL_FRAC, WIDTH};
use crate::core::factors::Factor;
use crate::core::game_settings::GameSettings;
use crate::core::global_economy::GlobalEconomy;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::player::{OwnedSecurity, Player};
use crate::core::resources::ImageIds;
use crate::core::securities::{SecurityKind, SecurityName};
use crate::core::states::GameState;
use crate::core::ui::bonds::bonds_panel;
use crate::core::ui::commodities::commodities_panel;
use crate::core::ui::credit::credit_panel;
use crate::core::ui::forex::forex_panel;
use crate::core::ui::state::{Tab, TradeTab, UiState};
use crate::core::ui::utils::{CustomHover, CustomUi, TextSizes, add_text};
use crate::utils::{NameFromEnum, create_guid, format_number};

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
                        Enterprise value: {}\n\
                        Cash: {}\nDebt: {:.0}",
                        player.cash,
                        -player.loans.iter().map(|l| l.outstanding).sum::<f32>()
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

                ui.add_factor(
                    "Net flow",
                    format!("{:+.0}", player.netflow().floor()),
                    match player.netflow() {
                        n if n <= -1. => Color32::RED,
                        n if n >= 1. => GREEN,
                        _ => text_color,
                    },
                    images.get("netflow"),
                    format!(
                        "The net flow represents the total financial movement at the end of \
                        each month, calculated as income minus debt repayments and expenses. \
                        It shows whether the player will gain or lose money this month.\n\n\
                        Inflow: {:+.0}\nOutflow: {:+.0}",
                        player.inflow().floor(),
                        -player.outflow().floor(),
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

pub fn trade_modal(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    economy: Res<GlobalEconomy>,
    mut player: ResMut<Player>,
    mut messages: EventWriter<MessageEv>,
    images: Res<ImageIds>,
    window: Single<&Window>,
) {
    if ui_state.trade.active {
        let security = economy.get(&ui_state.trade.security);

        let owned = player
            .securities
            .iter()
            .filter_map(|s| (s.name == security.name).then_some(s.amount))
            .sum::<u32>();

        let modal = Modal::new(Id::new("trade")).show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ComboBox::from_id_salt("Security")
                        .selected_text(add_text(
                            ui_state.trade.security.to_name(),
                            window.xl_size(),
                        ))
                        .show_ui(ui, |ui| {
                            for kind in SecurityKind::iter() {
                                ui.add_text(kind.plural(), window.s_size());
                                ui.separator();

                                for name in SecurityName::iter() {
                                    if economy.get(&name).kind == kind {
                                        ui.selectable_value(
                                            &mut ui_state.trade.security,
                                            name,
                                            add_text(name.to_name(), window.s_size()),
                                        );
                                    }
                                }
                            }
                        });

                    ui.add(Image::new(SizedTexture::new(
                        images.get(security.name.to_lowername().as_str()),
                        [window.height() * 0.2; 2],
                    )));
                });

                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        for tab in TradeTab::iter() {
                            ui.selectable_value(
                                &mut ui_state.trade.tab,
                                tab,
                                add_text(
                                    format!("{}  {}", tab.emoji(), tab.to_name()),
                                    window.l_size(),
                                ),
                            )
                            .on_hover(tab.description(), window.m_size());
                        }
                    });

                    ui.add_space(window.height() * 0.02);

                    ui.add_text(
                        format!("Unit price: {:.0}", security.current()),
                        window.m_size(),
                    );
                    ui.add_text(
                        format!(
                            "Owned: {owned}          Value: {:.0}",
                            owned as f32 * security.current()
                        ),
                        window.m_size(),
                    );

                    ui.horizontal(|ui| {
                        ui.add_text("Quantity:", window.m_size());

                        let amount = ui_state.trade.amount;
                        ui.spacing_mut().slider_width = window.width() * 0.15;
                        ui.add(
                            Slider::new(
                                &mut ui_state.trade.amount,
                                0..=((player.cash.current() / security.current()) as u32)
                                    .max(owned),
                            )
                            .show_value(false)
                            .text(add_text(amount.to_string(), window.m_size())),
                        );

                        ui.add_space(window.width() * 0.02);

                        ui.add_text(
                            format!(
                                "Total price: {:.0}",
                                security.current() * ui_state.trade.amount as f32
                            ),
                            window.m_size(),
                        );
                    });

                    let mut buy_clicked = false;
                    let mut sell_clicked = false;
                    let mut close_clicked = false;

                    Sides::new().show(
                        ui,
                        |ui| {
                            ui.add_enabled_ui(
                                ui_state.trade.amount > 0
                                    && player.cash.current()
                                        >= security.current() * ui_state.trade.amount as f32,
                                |ui| {
                                    let button = ui
                                        .add_sized(
                                            [window.width() * 0.08, window.height() * 0.05],
                                            Button::new(add_text("Buy", window.xl_size())),
                                        )
                                        .on_hover(
                                            format!(
                                                "Buy {} units of {}.",
                                                ui_state.trade.amount,
                                                security.name.to_lowername()
                                            ),
                                            window.m_size(),
                                        );

                                    if button.clicked() {
                                        buy_clicked = true;
                                    }
                                },
                            );
                        },
                        |ui| {
                            ui.add_enabled_ui(owned > 0, |ui| {
                                let button = ui
                                    .add_sized(
                                        [window.width() * 0.08, window.height() * 0.05],
                                        Button::new(add_text("Close position", window.xl_size())),
                                    )
                                    .on_hover(
                                        format!(
                                            "Sell all units of {}.",
                                            security.name.to_lowername()
                                        ),
                                        window.m_size(),
                                    )
                                    .on_disabled_hover(
                                        format!("No {} to sell", security.name.to_lowername()),
                                        window.m_size(),
                                    );

                                if button.clicked() {
                                    close_clicked = true;
                                }

                                ui.add_enabled_ui(owned >= ui_state.trade.amount, |ui| {
                                    let button = ui
                                        .add_sized(
                                            [window.width() * 0.08, window.height() * 0.05],
                                            Button::new(add_text("Sell", window.xl_size())),
                                        )
                                        .on_hover(
                                            format!(
                                                "Sell {} units of {}.",
                                                ui_state.trade.amount,
                                                security.name.to_lowername()
                                            ),
                                            window.m_size(),
                                        )
                                        .on_disabled_hover(
                                            format!(
                                                "Not enough units of {} to sell.",
                                                security.name.to_lowername(),
                                            ),
                                            window.m_size(),
                                        );

                                    if button.clicked() {
                                        sell_clicked = true;
                                    }
                                });
                            });
                        },
                    );

                    // Resolve button clicks
                    if buy_clicked {
                        player.cash.amount -= security.current() * ui_state.trade.amount as f32;
                        player.securities.push(OwnedSecurity {
                            id: create_guid(),
                            name: security.name,
                            amount: ui_state.trade.amount,
                            buy_date: economy.date,
                            buy_price: security.current(),
                            warning: false,
                        });

                        messages.write(MessageEv {
                            message: format!(
                                "Bought {} {}.",
                                ui_state.trade.amount,
                                security.name.to_lowername()
                            ),
                            level: MessageLevel::Info,
                        });
                    }

                    if close_clicked {
                        player.cash.amount += security.current() * owned as f32;
                        player.securities.retain(|s| s.name != security.name);

                        messages.write(MessageEv {
                            message: format!("Closed {} position.", security.name.to_lowername()),
                            level: MessageLevel::Info,
                        });
                    }

                    if sell_clicked {
                        player.cash.amount += security.current() * ui_state.trade.amount as f32;

                        let mut remaining = ui_state.trade.amount;
                        player
                            .securities
                            .iter_mut()
                            .filter(|s| s.name == security.name)
                            .sorted_by_key(|s| s.buy_date)
                            .for_each(|s| {
                                let to_deduct = remaining.min(s.amount);
                                s.amount -= to_deduct;
                                remaining -= to_deduct;
                            });

                        player.securities.retain(|s| s.amount > 0);

                        messages.write(MessageEv {
                            message: format!(
                                "Sold {} {}.",
                                ui_state.trade.amount,
                                security.name.to_lowername()
                            ),
                            level: MessageLevel::Info,
                        });
                    }
                });
            });
        });

        if modal.should_close() {
            ui_state.trade.active = false;
        }
    }
}

pub fn check_keys(keyboard: Res<ButtonInput<KeyCode>>, mut ui_state: ResMut<UiState>) {
    if keyboard.just_pressed(KeyCode::KeyO) {
        ui_state.tab = Tab::Overview;
    } else if keyboard.just_pressed(KeyCode::KeyT) {
        ui_state.trade.active = !ui_state.trade.active;
    }
}
