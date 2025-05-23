use crate::core::constants::{DATE_FORMAT, LOAN_STEP};
use crate::core::factors::Factor;
use crate::core::global_economy::GlobalEconomy;
use crate::core::loans::{Loan, LoanKind, LoanProvider, LoanTerm};
use crate::core::messages::Messages;
use crate::core::player::Player;
use crate::core::ui::state::UiState;
use crate::core::ui::utils::{CustomUi, TextSizes};
use crate::utils::{NameFromEnum, first_day_in_two_months};
use bevy::prelude::*;
use bevy_egui::egui::{Button, Frame, RichText, Sense, Separator, Slider, Ui};
use egui_extras::{Column, TableBuilder};
use strum::IntoEnumIterator;

pub fn credit_panel(
    ui: &mut Ui,
    ui_state: &mut UiState,
    player: &mut Player,
    economy: &GlobalEconomy,
    messages: &mut Messages,
    window: &Window,
) {
    ui.horizontal(|ui| {
        for tab in LoanProvider::iter() {
            ui.selectable_value(
                &mut ui_state.credit.provider,
                tab,
                RichText::new(format!("{}  {}", tab.emoji(), tab.to_name())).size(window.xl_size()),
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
        .cloned()
        .collect::<Vec<_>>();

    // Current selected loan
    let loan = Loan {
        provider: ui_state.credit.provider,
        principal: ui_state.credit.principal as f32,
        outstanding: ui_state.credit.principal as f32,
        interest_rate: ui_state.credit.interest(
            economy.interest.current(),
            player.credit_score.current(),
            ui_state.credit.provider,
        ),
        kind: ui_state.credit.kind.clone(),
        term: ui_state.credit.term.clone(),
        start_date: first_day_in_two_months(economy.date),
        defaults: 0,
    };

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.set_width(ui.available_width() * 0.2);

            ui.add_text("New loan", window.l_size());

            ui.add_space(window.height() * 0.02);

            ui.add_text("Principal", window.m_size());
            let max_principal = ui_state.credit.max_principal(
                player.enterprise_value(),
                player.credit_score.current(),
                ui_state.credit.provider,
            );

            ui.spacing_mut().slider_width = window.width() * 0.13;
            let principal = ui_state.credit.principal;
            ui.add(
                Slider::new(&mut ui_state.credit.principal, 0..=max_principal)
                    .step_by(LOAN_STEP as f64)
                    .show_value(false)
                    .text(RichText::new(principal.to_string()).size(window.s_size())),
            )
            .on_hover_text(
                "The amount of money you want to borrow. The maximum amount you can \
                borrow is determined by the enterprise value and the credit score.",
            );

            ui.add_space(window.height() * 0.02);

            ui.add_text("Loan kind", window.m_size());
            ui.horizontal(|ui| {
                for item in LoanKind::iter() {
                    ui.selectable_value(
                        &mut ui_state.credit.kind,
                        item.clone(),
                        RichText::new(item.to_name()).size(window.s_size()),
                    )
                    .on_hover_text(item.description());
                }
            });

            ui.add_space(window.height() * 0.02);

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

            ui.add_space(window.height() * 0.02);

            // Check if the player has active loan with >50% outstanding
            let has_loans = loans.iter().any(|l| l.outstanding > l.principal * 0.5);

            let mut button = ui.add_enabled(
                ui_state.credit.principal > 0 && !has_loans,
                Button::new(RichText::new("✏  Take the loan").size(window.l_size())),
            );

            if has_loans {
                button = button.on_disabled_hover_text(
                    "You have an outstanding loan with this provider. \
                    You can only take a new loan when the remaining debt is \
                    less than 50% of the principal.",
                );
            }

            if button.clicked() {
                player.loans.push(loan.clone());
                player.cash.amount += loan.principal;
                messages.info("Loan acquired!");
            }
        });

        ui.add(
            Separator::default()
                .vertical()
                .shrink(window.height() * 0.06),
        );

        ui.vertical(|ui| {
            ui.set_width(ui.available_width() * 0.3);

            ui.add_text("", window.l_size());

            ui.add_space(window.height() * 0.02);

            ui.add_text("Conditions", window.m_size());
            ui.add_text(
                format!("Interest rate: {}%", loan.interest_rate),
                window.s_size(),
            )
            .on_hover_text("Percentage of the principal that must be paid as interest every year.");
            ui.add_text(
                format!(
                    "First installment amount: {:.0}",
                    loan.next_installment_amount(),
                ),
                window.s_size(),
            )
            .on_hover_text("Amount to be paid back every month.");
            ui.add_text(
                format!("Start date: {}", loan.start_date.format(DATE_FORMAT)),
                window.s_size(),
            )
            .on_hover_text("Starting date of the installments.");
            ui.add_text(
                format!(
                    "Maturity date: {}",
                    loan.maturity_date().format(DATE_FORMAT)
                ),
                window.s_size(),
            )
            .on_hover_text("Date on which the loan is fully repaid.");
        });

        ui.vertical(|ui| {
            ui.set_width(ui.available_width());

            ui.add_text("Outstanding loans", window.l_size());

            if loans.is_empty() {
                ui.add_text("No outstanding loans with this provider.", window.m_size());
            } else {
                Frame::new()
                    .inner_margin(ui.spacing().menu_margin)
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .show(ui, |ui| {
                        TableBuilder::new(ui)
                            .sense(Sense::click())
                            .columns(Column::remainder(), 6)
                            .header(30., |mut header| {
                                for col in [
                                    "Principal",
                                    "Outstanding",
                                    "Kind",
                                    "Interest",
                                    "Maturity",
                                    "Defaults",
                                ] {
                                    header.col(|ui| {
                                        ui.label(RichText::new(col).size(window.s_size()).strong());
                                    });
                                }
                            })
                            .body(|mut body| {
                                for loan in loans.iter() {
                                    body.row(30., |mut row| {
                                        row.col(|ui| {
                                            ui.add_text(
                                                loan.principal.to_string(),
                                                window.s_size(),
                                            );
                                        });
                                        row.col(|ui| {
                                            ui.add_text(
                                                loan.outstanding.floor().to_string(),
                                                window.s_size(),
                                            );
                                        });
                                        row.col(|ui| {
                                            ui.add_text(loan.kind.to_name(), window.s_size());
                                        });
                                        row.col(|ui| {
                                            ui.add_text(
                                                format!("{:.1}%", loan.interest_rate),
                                                window.s_size(),
                                            );
                                        });
                                        row.col(|ui| {
                                            ui.add_text(
                                                loan.maturity_date()
                                                    .format(DATE_FORMAT)
                                                    .to_string(),
                                                window.s_size(),
                                            );
                                        });
                                        row.col(|ui| {
                                            ui.add_text(loan.defaults.to_string(), window.s_size());
                                        });

                                        if row.response().clicked() {
                                            ui_state.credit.repay = Some(loan.clone());
                                        }
                                    });
                                }
                            });
                    });

                if let Some(loan) = &ui_state.credit.repay {
                    ui.add_space(window.height() * 0.02);

                    ui.add_text("Repay loan early", window.l_size());

                    ui.add_space(window.height() * 0.02);
                    
                    let button = ui.add(
                        Button::new(RichText::new("✏  Repay loan").size(window.l_size()))
                    );
                }
            }
        });
    });
}
