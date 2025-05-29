use bevy::prelude::*;
use bevy_egui::egui::{Button, Frame, Sense, Separator, Slider, Ui};
use chrono::Months;
use egui_extras::{Column, TableBuilder};
use strum::IntoEnumIterator;

use crate::core::constants::{DATE_FORMAT, LOAN_STEP};
use crate::core::factors::Factor;
use crate::core::global_economy::GlobalEconomy;
use crate::core::loans::{Loan, LoanKind, LoanProvider, LoanTerm};
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::player::Player;
use crate::core::ui::state::{CreditTab, UiState};
use crate::core::ui::utils::{CustomUi, TextSizes, add_text, toggle};
use crate::utils::{NameFromEnum, create_guid, first_day_in_two_months};

pub fn credit_panel(
    ui: &mut Ui,
    ui_state: &mut UiState,
    player: &mut Player,
    economy: &GlobalEconomy,
    message: &mut EventWriter<MessageEv>,
    window: &Window,
) {
    ui.horizontal(|ui| {
        for tab in CreditTab::iter() {
            ui.selectable_value(
                &mut ui_state.credit.tab,
                tab,
                add_text(
                    format!("{}  {}", tab.emoji(), tab.to_name()),
                    window.l_size(),
                ),
            );
        }
    });

    ui.separator();

    ui.add_text(
        "Credit refers to the ability of borrowing money, with the promise of repayment \
        in the future. It's a fundamental part of the financial system, allowing companies to \
        make purchases, invest, and manage expenses beyond their immediate cash availability.\n\n\
        If a company defaults on a loan (fails to pay an installment) four consecutive months, \
        its assets will be forcibly sold (usually for unfavorable terms) until there is enough \
        cash to pay back the complete loan. Six months after the start date of a loan, a company \
        can choose to repay the debt early, paying an additional fee to the provider to cover \
        missed earnings.",
        window.m_size(),
    );

    ui.separator();

    ui.add_space(window.height() * 0.02);

    match ui_state.credit.tab {
        CreditTab::Overview => {
            if player.loans.is_empty() {
                ui.add_text("Outstanding loans", window.l_size());

                ui.add_space(window.height() * 0.02);

                ui.add_text("No outstanding loans.", window.s_size());

                ui.add_space(window.height() * 0.02);

                let button = ui.add(Button::new(add_text("Take a new loan", window.m_size())));

                if button.clicked() {
                    ui_state.credit.tab = CreditTab::NewLoan;
                }
            } else {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.set_width(ui.available_width() * 0.78);

                        ui.add_text("Outstanding loans", window.l_size());

                        ui.add_space(window.height() * 0.02);

                        credit_overview(ui, ui_state, &player.loans, window);

                        ui.add_text("Click on a row to repay the loan early.", window.xs_size());
                    });

                    ui.add_space(window.width() * 0.02);

                    ui.vertical(|ui| {
                        if let Some(id) = &ui_state.credit.repay {
                            if let Some(loan) = player.loans.iter_mut().find(|l| l.id == *id) {
                                ui.add_text(format!("Repay loan {} early", loan.id), window.l_size());

                                ui.add_space(window.height() * 0.02);

                                let repay_amount = ui_state.credit.repay_amount;
                                let fee = if loan.no_fee {
                                    0.
                                } else if economy.interest.current() >= loan.global_interest_rate {
                                    loan.installments_left().min(12) as f32
                                        * repay_amount as f32
                                        * loan.interest_rate
                                        / 100.
                                        / 12.
                                } else {
                                    repay_amount as f32
                                        * (loan.interest_rate
                                            + (loan.global_interest_rate
                                                - economy.interest.current()))
                                        / 100.
                                        / 12.
                                        * loan.installments_left() as f32
                                };

                                ui.add_text("Amount", window.m_size());
                                
                                ui.spacing_mut().slider_width = window.width() * 0.09;
                                ui.add(
                                    Slider::new(
                                        &mut ui_state.credit.repay_amount,
                                        0..=loan.outstanding.min(player.cash.current() - fee)
                                            as u32,
                                    )
                                    .show_value(false)
                                    .text(add_text(repay_amount.to_string(), window.m_size())),
                                );

                                let costs = ui_state.credit.repay_amount as f32 + fee;
                                
                                ui.add_space(window.height() * 0.02);

                                ui.add_text(format!("Fee: {fee:.0}"), window.m_size())
                                    .on_hover_text(
                                        "If the global interest rate has increased since \
                                        the start of the loan, the fee consists of up to twelve \
                                        months of interest over the repaid amount (depending on \
                                        the number of installments left). If the global interest \
                                        rate has decreased, the fee consists of the agreed \
                                        interest plus the current difference multiplied by the \
                                        number of missed installments.",
                                    );
                                ui.add_text(format!("Total costs: {costs:.0}"), window.m_size())
                                    .on_hover_text("Total amount to be paid. Includes the repaid amount plus the repayment fee.");

                                ui.add_space(window.height() * 0.02);

                                let date_diff = economy.date > loan.start_date.checked_add_months(Months::new(6)).unwrap();
                                
                                let mut button = ui.add_enabled(
                                    ui_state.credit.repay_amount > 0 && date_diff,
                                    Button::new(add_text("Repay loan", window.m_size())),
                                );

                                if !date_diff {
                                    button = button.on_disabled_hover_text(
                                        "You can only repay a loan early six months after the start date.",
                                    );
                                }
                                
                                if button.clicked() {
                                    player.cash.amount -= costs;
                                    loan.outstanding -= ui_state.credit.repay_amount as f32;

                                    message.write(MessageEv {
                                        message: format!("You repaid {} of loan {}.", ui_state.credit.repay_amount, loan.id),
                                        level: MessageLevel::Info,
                                    });
                                }
                            } else {
                                ui_state.credit.repay = None;
                            }

                            // Remove loans that are fully repaid
                            player.loans.retain(|l| l.outstanding >= 1.);
                        }
                    });
                });
            }
        },
        CreditTab::NewLoan => {
            let loan = Loan {
                id: create_guid(),
                provider: ui_state.credit.provider,
                principal: ui_state.credit.principal as f32,
                outstanding: ui_state.credit.principal as f32,
                n_installments: 0,
                interest_rate: ui_state.credit.provider.interest(
                    economy.interest.current(),
                    player.credit_score.current(),
                    &ui_state.credit.term,
                    ui_state.credit.no_fee,
                ),
                global_interest_rate: economy.interest.current(),
                kind: ui_state.credit.kind.clone(),
                term: ui_state.credit.term.clone(),
                no_fee: ui_state.credit.no_fee,
                start_date: first_day_in_two_months(economy.date),
                defaults: 0,
            };

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.add_text("Provider", window.m_size());
                    ui.horizontal(|ui| {
                        for item in LoanProvider::iter() {
                            ui.selectable_value(
                                &mut ui_state.credit.provider,
                                item.clone(),
                                add_text(item.to_name(), window.s_size()),
                            )
                                .on_hover_text(item.description());
                        }
                    });

                    ui.add_text("Principal", window.m_size());
                    let max_principal = ui_state
                        .credit
                        .provider
                        .max_principal(player.enterprise_value(), player.credit_score.current());

                    ui.spacing_mut().slider_width = window.width() * 0.13;
                    let principal = ui_state.credit.principal;
                    ui.add(
                        Slider::new(&mut ui_state.credit.principal, 0..=max_principal)
                            .step_by(LOAN_STEP as f64)
                            .show_value(false)
                            .text(add_text(principal.to_string(), window.m_size())),
                    )
                        .on_hover_text(
                            "The amount of money you want to borrow. The maximum amount you \
                            can borrow is determined by the credit provider and the company's \
                            enterprise value.",
                        );
                    
                    ui.add_text("Kind", window.m_size());
                    ui.horizontal(|ui| {
                        for item in LoanKind::iter() {
                            ui.selectable_value(
                                &mut ui_state.credit.kind,
                                item.clone(),
                                add_text(item.to_name(), window.s_size()),
                            )
                                .on_hover_text(item.description());
                        }
                    });

                    ui.add_text("Term", window.m_size());
                    ui.horizontal(|ui| {
                        for item in LoanTerm::iter() {
                            ui.selectable_value(
                                &mut ui_state.credit.term,
                                item.clone(),
                                add_text(item.to_name(), window.s_size()),
                            )
                                .on_hover_text(
                                    "Longer terms reduce the monthly installment and the interest rate.");
                        }
                    });

                    ui.add_text("Prepayment-free loan", window.m_size());
                    ui.add(toggle(&mut ui_state.credit.no_fee)).on_hover_text(
                        "No early repayment fee for an increase in interest.",
                    );

                    // Check if the player has active loan with >50% outstanding
                    let has_loans = player
                        .loans
                        .iter()
                        .filter(|l| l.provider == ui_state.credit.provider)
                        .any(|l| l.outstanding > l.principal * 0.5);

                    let mut button = ui.add_enabled(
                        ui_state.credit.principal > 0 && !has_loans,
                        Button::new(add_text("Take the loan", window.m_size())),
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
                        player.loans.sort_by_key(|loan| loan.maturity_date());
                        player.cash.amount += loan.principal;
                        message.write(MessageEv {
                            message: format!("Loan {} acquired!", loan.id),
                            level: MessageLevel::Info,
                        });
                    }
                });

                ui.add(Separator::default().vertical());

                ui.vertical(|ui| {
                    ui.add_text("Conditions", window.m_size());

                    ui.add_text(
                        format!("Interest rate: {}%", loan.interest_rate),
                        window.s_size(),
                    )
                        .on_hover_text(
                            "Percentage of the outstanding amount that must be paid as \
                            interest every year.",
                        );

                    ui.add_text(
                        format!(
                            "Installment: {:.0}",
                            loan.next_installment_amount(),
                        ),
                        window.s_size(),
                    )
                        .on_hover_text(
                            "Amount to be paid back the first month. For the straight-line \
                            loan kind, the installments change over time."
                        );

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
            });
        },
        CreditTab::P2P => {},
    }
}

pub fn credit_overview(ui: &mut Ui, ui_state: &mut UiState, loans: &Vec<Loan>, window: &Window) {
    let columns = [
        "Id",
        "Maturity",
        "Provider",
        "Principal",
        "Outstanding",
        "Installment",
        "Interest",
        "Kind",
        "No fee",
        "Defaults",
    ];

    Frame::new()
        .inner_margin(ui.spacing().menu_margin)
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .show(ui, |ui| {
            TableBuilder::new(ui)
                .striped(false)
                .sense(Sense::click())
                .columns(Column::auto(), 1)
                .columns(Column::remainder(), columns.len() - 1)
                .header(30., |mut header| {
                    for col in columns {
                        header.col(|ui| {
                            ui.label(add_text(col, window.s_size()).strong());
                        });
                    }
                })
                .body(|mut body| {
                    for loan in loans {
                        let content = [
                            &loan.id,
                            &loan.maturity_date().format(DATE_FORMAT).to_string(),
                            &loan.provider.to_name(),
                            &loan.principal.to_string(),
                            &loan.outstanding.floor().to_string(),
                            &loan.next_installment_amount().floor().to_string(),
                            &format!("{}%", loan.interest_rate.to_string()),
                            &loan.kind.to_name(),
                            &loan.no_fee.to_string(),
                            &loan.defaults.to_string(),
                        ];

                        body.row(30., |mut row| {
                            for col in content {
                                row.col(|ui| {
                                    ui.add_text(col, window.s_size());
                                });
                            }

                            if row.response().clicked() {
                                ui_state.credit.repay = Some(loan.id.clone());
                            }
                        });
                    }
                });
        });
}
