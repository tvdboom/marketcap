use crate::core::constants::{GREEN, LEFT_LABEL_FRAC, MIN_PRINCIPAL, TOP_LABEL_FRAC};
use crate::core::factors::Factor;
use crate::core::game_settings::GameSettings;
use crate::core::global_economy::GlobalEconomy;
use crate::core::loans::{LoanKind, LoanProvider};
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::states::GameState;
use crate::core::ui::state::{LoanTerm, Tab, UiState};
use crate::core::ui::utils::{CustomUi, TextSizes};
use crate::utils::{last_day_of_next_month, NameFromEnum};
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::widget_text::RichText;
use bevy_egui::egui::{
    Align, Button, CentralPanel, Color32, Frame, Layout, Margin, SidePanel, Slider, TopBottomPanel,
};
use egui_extras::{Column, TableBuilder};
use strum::IntoEnumIterator;

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
                ui.add_space(window.width() * 0.075);

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
                        player.outflow().floor(),
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
                ui.add_space(window.height() * 0.12);

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
    player: Res<Player>,
    window: Single<&Window>,
) {
    CentralPanel::default()
        .frame(
            Frame::new()
                .fill(game_settings.theme.get().bg_primary_color_visuals())
                .inner_margin(Margin::same(48)),
        )
        .show(contexts.ctx_mut(), |ui| match ui_state.tab {
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
                ui.horizontal(|ui| {
                    for tab in LoanProvider::iter() {
                        ui.selectable_value(
                            &mut ui_state.credit.provider,
                            tab,
                            RichText::new(format!("{}  {}", tab.emoji(), tab.to_name()))
                                .size(window.xl_size()),
                        );
                    }
                });

                ui.separator();

                ui.add_text(ui_state.credit.provider.description(), window.m_size());

                ui.separator();

                // Current loans with this provider
                let loans = player
                    .loans
                    .iter()
                    .filter(|l| l.provider == ui_state.credit.provider)
                    .collect::<Vec<_>>();

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.set_width(ui.available_width() * 0.5);

                        ui.add_text("New loan", window.l_size());

                        ui.add_text("Principal", window.m_size());
                        let max_principal = ui_state
                            .credit
                            .max_principal(player.enterprise_value(), player.credit_score.current());

                        ui.add(
                            Slider::new(&mut ui_state.credit.principal,MIN_PRINCIPAL..=max_principal)
                            .step_by(100.)
                        );

                        ui.add_text("Loan kind", window.m_size());
                        ui.horizontal(|ui| {
                            for item in LoanKind::iter() {
                                ui.selectable_value(
                                    &mut ui_state.credit.kind,
                                    item.clone(),
                                    RichText::new(item.to_name()).size(window.s_size()),
                                )
                                .on_hover_text(LoanKind::Annuity.description());
                            }
                        });

                        ui.add_text("Term", window.m_size());
                        ui.horizontal(|ui| {
                            for item in LoanTerm::iter() {
                                ui.selectable_value(
                                    &mut ui_state.credit.term,
                                    item.clone(),
                                    RichText::new(item.to_name()).size(window.s_size()),
                                );
                            }
                        });

                        ui.add_text("Conditions", window.m_size());
                        let interest = ui_state.credit.interest(
                            economy.interest.current(),
                            player.credit_score.current(),
                        );

                        let installment = ui_state.credit.installment(interest);
                        let start = last_day_of_next_month(economy.date);
                        
                        ui.add_text(format!("Interest rate: {:.1}%", interest), window.s_size()).on_hover_text("Percentage of the principal that must be paid as interest every year.");
                        ui.add_text(format!("First installment: {:.0} on {}", installment, start.format("%d-%m-%Y")), window.s_size()).on_hover_text("Amount to be paid back every month.");
                        ui.add_text(format!("Maturity date: {}", 5), window.s_size()).on_hover_text("Date on which the loan is fully repaid.");
                        
                        let button = ui
                            .add_enabled(
                                loans.iter().all(|l| l.outstanding < l.principal * 0.5),
                                Button::new("✏  Take the loan"),
                            )
                            .on_disabled_hover_text(
                                "You have an outstanding loan with this provider. \
                                You can only take a new loan when the remaining debt is \
                                less than 50% of the principal.",
                            );

                        if button.clicked() {
                            println!("check");
                        }
                    });

                    ui.vertical(|ui| {
                        ui.set_width(ui.available_width());

                        ui.add_text("Outstanding loans", window.l_size());

                        if loans.is_empty() {
                            ui.add_text(
                                "No outstanding loans with this provider.",
                                window.m_size(),
                            );
                        } else {
                            Frame::new()
                                .inner_margin(ui.spacing().menu_margin)
                                .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                                .show(ui, |ui| {
                                    TableBuilder::new(ui)
                                        .columns(Column::auto(), 5)
                                        .header(30., |mut header| {
                                            header.col(|ui| {
                                                ui.strong("Principal");
                                            });
                                            header.col(|ui| {
                                                ui.strong("Outstanding");
                                            });
                                            header.col(|ui| {
                                                ui.strong("Interest");
                                            });
                                            header.col(|ui| {
                                                ui.strong("Term");
                                            });
                                            header.col(|ui| {
                                                ui.strong("Kind");
                                            });
                                        })
                                        .body(|mut body| {
                                            for loan in loans.iter() {
                                                body.row(30., |mut row| {
                                                    row.col(|ui| {
                                                        ui.label(format!("{:.0}", loan.principal));
                                                    });
                                                    row.col(|ui| {
                                                        ui.label(format!(
                                                            "{:.0}",
                                                            loan.outstanding.floor()
                                                        ));
                                                    });
                                                    row.col(|ui| {
                                                        ui.label(format!(
                                                            "{:.1}%",
                                                            loan.interest_rate
                                                        ));
                                                    });
                                                    row.col(|ui| {
                                                        ui.label(
                                                            loan.term
                                                                .format("%d-%m-%Y")
                                                                .to_string(),
                                                        );
                                                    });
                                                    row.col(|ui| {
                                                        ui.label(loan.kind.to_name());
                                                    });
                                                });
                                            }
                                        });
                                });
                        }
                    });
                });
            }
            Tab::Policies => {
                ui.heading("Policies");
            }
        });
}
