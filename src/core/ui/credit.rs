use bevy::prelude::*;
use bevy_egui::egui::{Button, ComboBox, Separator, Slider, Ui};
use chrono::Months;
use strum::IntoEnumIterator;

use crate::core::constants::{CURRENCY, DATE_FORMAT, LOAN_STEP};
use crate::core::factors::Factor;
use crate::core::global_economy::GlobalEconomy;
use crate::core::loans::{LoanKind, LoanProvider, Term, TermLoan};
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::player::Player;
use crate::core::ui::state::{CreditTab, OverviewTab, Tab, UiState};
use crate::core::ui::utils::toggle;
use crate::utils::{NameFromEnum, create_guid, first_day_in_two_months};

pub fn credit_panel(
    ui: &mut Ui,
    state: &mut UiState,
    economy: &GlobalEconomy,
    player: &mut Player,
    message: &mut EventWriter<MessageEv>,
    window: &Window,
) {
    ui.horizontal(|ui| {
        for tab in CreditTab::iter() {
            ui.selectable_value(
                &mut state.credit.tab,
                tab,
                format!("{}  {}", tab.emoji(), tab.to_name()),
            );
        }
    });

    ui.separator();

    ui.label(
        "Credit refers to the ability of borrowing money, with the promise of repayment \
        in the future. It's a fundamental part of the financial system, allowing companies to \
        make purchases, invest, and manage expenses beyond their immediate cash availability.\n\n\
        Six months after the start date of a loan, a company can choose to repay the debt early, \
        paying an additional fee to the provider to cover missed earnings. A new loan can be \
        taken by the same provider only when the remaining debt is less than 50% of the principal.",
    );

    ui.separator();

    ui.add_space(window.height() * 0.02);

    match state.credit.tab {
        CreditTab::NewLoan => {
            let loan = TermLoan {
                id: create_guid(),
                provider: state.credit.provider,
                principal: state.credit.principal as f32,
                outstanding: state.credit.principal as f32,
                n_installments: 0,
                interest_rate: state.credit.provider.interest(
                    economy.interest.current(),
                    player.credit_score.current(),
                    &state.credit.term,
                    state.credit.no_fee,
                ),
                global_interest_rate: economy.interest.current(),
                kind: state.credit.kind.clone(),
                term: state.credit.term.clone(),
                no_fee: state.credit.no_fee,
                start_date: first_day_in_two_months(economy.date),
            };

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(ui.available_width() * 0.35);
                    
                    ui.label("Provider");
                    ui.horizontal(|ui| {
                        for item in LoanProvider::iter() {
                            ui.selectable_value(
                                &mut state.credit.provider,
                                item,
                                item.to_name(),
                            )
                                .on_hover_text(item.description());
                        }
                    });

                    ui.label("Principal");
                    let max_principal = state
                        .credit
                        .provider
                        .max_principal(player.enterprise_value(&economy), player.credit_score.current());

                    ui.spacing_mut().slider_width = window.width() * 0.13;
                    let principal = state.credit.principal;
                    ui.add(
                        Slider::new(&mut state.credit.principal, 0..=max_principal)
                            .step_by(LOAN_STEP as f64)
                            .show_value(false)
                            .text(principal.to_string()),
                    )
                        .on_hover_text(
                            "The amount of money you want to borrow. The maximum amount you \
                            can borrow is determined by the credit provider and the company's \
                            enterprise value."
                        );
                    
                    ui.label("Kind");
                    ui.horizontal(|ui| {
                        for item in LoanKind::iter() {
                            ui.selectable_value(
                                &mut state.credit.kind,
                                item,
                                item.to_name(),
                            )
                                .on_hover_text(item.description());
                        }
                    });

                    ui.label("Term");
                    ui.horizontal(|ui| {
                        for item in Term::iter() {
                            ui.selectable_value(
                                &mut state.credit.term,
                                item,
                                item.to_name(),
                            )
                                .on_hover_text(
                                    "Longer terms reduce the monthly installment and the interest rate.");
                        }
                    });

                    ui.label("Prepayment-free loan");
                    ui.add(toggle(&mut state.credit.no_fee)).on_hover_text(
                        "No early repayment fee for an increase in interest."
                    );

                    // Check if the player has active loan with >50% outstanding
                    let has_loans = player
                        .loans
                        .iter()
                        .filter(|l| l.provider == state.credit.provider)
                        .any(|l| l.outstanding > l.principal * 0.5);

                    let mut button = ui.add_enabled(
                        state.credit.principal > 0 && !has_loans,
                        Button::new("Take the loan"),
                    );

                    if has_loans {
                        button = button.on_disabled_hover_text(
                            "You have an outstanding loan with this provider. \
                            You can only take a new loan when the remaining debt is \
                            less than 50% of the principal."
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
                    ui.heading("Conditions");

                    ui.label(
                        format!("Interest rate: {:.1}%", loan.interest_rate),
                    )
                        .on_hover_text(
                            "Percentage of the outstanding amount that must be paid as \
                            interest every year."
                        );

                    ui.label(
                        format!(
                            "Installment: {:.0} {CURRENCY}",
                            loan.next_installment_amount(),
                        ),
                    )
                        .on_hover_text(
                            "Amount to be paid back the first month. For the straight-line \
                            loan kind, the installments change over time."
                        );

                    ui.label(format!("Start date: {}", loan.start_date.format(DATE_FORMAT)))
                        .on_hover_text("Starting date of the installments.");

                    ui.label(
                        format!(
                            "Maturity date: {}",
                            loan.maturity_date().format(DATE_FORMAT)
                        ),
                    )
                        .on_hover_text("Date on which the loan is fully repaid.");
                });
            });
        },
        CreditTab::RepayLoan => {
            if player.loans.is_empty() {
                ui.heading("Repay loan early");

                ui.add_space(window.height() * 0.02);

                ui.label("No outstanding loans.");

                ui.add_space(window.height() * 0.02);

                if ui.button("Take a new loan").clicked() {
                    state.credit.tab = CreditTab::NewLoan;
                }
            } else {
                let loans = player
                    .loans
                    .iter()
                    .map(|l| l.id.clone())
                    .collect::<Vec<_>>();

                // Assign the first loan to the repay field if it's not set
                if state.credit.repay.is_none() {
                    state.credit.repay = Some(loans[0].clone());
                }

                if let Some(id) = &state.credit.repay {
                    if let Some(loan) = player.loans.iter_mut().find(|l| l.id == *id) {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.set_width(ui.available_width() * 0.3);

                                ui.label("Loan id");
                                
                                ComboBox::from_id_salt("loan")
                                    .selected_text(&loan.id)
                                    .show_ui(ui, |ui| {
                                        for id in loans.iter() {
                                            ui.selectable_value(
                                                &mut state.credit.repay,
                                                Some(id.clone()),
                                                id,
                                            );
                                        }
                                    });

                                ui.add_space(window.height() * 0.02);

                                let repay_amount = state.credit.repay_amount;
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

                                ui.label("Amount");

                                ui.spacing_mut().slider_width = window.width() * 0.12;
                                ui.add(
                                    Slider::new(
                                        &mut state.credit.repay_amount,
                                        0..=loan.outstanding.min(player.cash.current() - fee)
                                            as u32,
                                    )
                                        .show_value(false)
                                        .text(format!("{repay_amount} {CURRENCY}")),
                                );

                                let costs = state.credit.repay_amount as f32 + fee;

                                ui.add_space(window.height() * 0.02);

                                ui.label(format!("Fee: {fee:.0} {CURRENCY}"))
                                    .on_hover_text(
                                        "If the global interest rate has increased since \
                                        the start of the loan, the fee consists of up to twelve \
                                        months of interest over the repaid amount (depending on \
                                        the number of installments left). If the global interest \
                                        rate has decreased, the fee consists of the agreed \
                                        interest plus the current difference multiplied by the \
                                        number of missed installments."
                                    );
                                ui.label(format!("Total costs: {costs:.0} {CURRENCY}"))
                                    .on_hover_text("Total amount to be paid. Includes the repaid amount plus the repayment fee.");

                                ui.add_space(window.height() * 0.02);

                                let date_diff = economy.date > loan.start_date.checked_add_months(Months::new(6)).unwrap();

                                let mut button = ui.add_enabled(
                                    state.credit.repay_amount > 0 && date_diff,
                                    Button::new("Repay loan"),
                                );

                                if !date_diff {
                                    button = button.on_disabled_hover_text(
                                        "You can only repay a loan early six months after the start date.",
                                    );
                                }

                                if button.clicked() {
                                    player.cash.amount -= costs;
                                    loan.outstanding -= state.credit.repay_amount as f32;

                                    message.write(MessageEv {
                                        message: format!("You repaid {} of loan {}.", state.credit.repay_amount, loan.id),
                                        level: MessageLevel::Info,
                                    });
                                    
                                    state.tab = Tab::Overview;
                                    state.overview.tab = OverviewTab::Debts;
                                }
                            });

                            ui.add(Separator::default().vertical());
        
                            ui.vertical(|ui| {
                                ui.heading("Details");
                                ui.label(format!("Start date: {}", loan.start_date.format(DATE_FORMAT)));
                                ui.label(format!("Maturity date: {}", loan.maturity_date().format(DATE_FORMAT)));
                                ui.label(format!("Provider: {}", loan.provider.to_name()));
                                ui.label(format!("Principal: {:.0} {CURRENCY}", loan.principal));
                                ui.label(format!("Outstanding: {:.0} {CURRENCY}", loan.outstanding));
                                ui.label(format!("Installment: {:.0} {CURRENCY}", loan.next_installment_amount()));
                                ui.label(format!("Interest rate: {:.1}%", loan.interest_rate));
                            });
                        });
                    } else {
                        state.credit.repay = None;
                    }

                    // Remove loans that are fully repaid
                    player.loans.retain(|l| l.outstanding >= 1.);
                }
            }
        },
        CreditTab::P2P => {},
    }
}
