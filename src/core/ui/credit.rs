use crate::core::constants::{DATE_FORMAT, LOAN_STEP};
use crate::core::factors::Factor;
use crate::core::global_economy::GlobalEconomy;
use crate::core::loans::{Loan, LoanKind, LoanTerm};
use crate::core::messages::Messages;
use crate::core::player::Player;
use crate::core::ui::state::{CreditTab, UiState};
use crate::core::ui::utils::{CustomUi, TextSizes};
use crate::utils::{NameFromEnum, create_guid, first_day_in_two_months};
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
        for tab in CreditTab::iter() {
            ui.selectable_value(
                &mut ui_state.credit.tab,
                tab,
                RichText::new(format!("{}  {}", tab.emoji(), tab.to_name())).size(window.xl_size()),
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
        cash to pay back the complete loan.\n\n\
        Six months after the start date of a loan, a company can choose to repay the debt early, \
        paying an additional fee to the provider to cover missed earnings. If the global interest \
        rate has increased since the start of the loan, the fee consists of two months of interest \
        over the repaid amount. If the global interest rate has decreased, the fee consists of \
        the difference in interest times the repaid installments.",
        window.m_size(),
    );

    ui.separator();

    match ui_state.credit.tab {
        CreditTab::OutstandingLoans => {
            ui.add_text("Outstanding loans", window.l_size());

            if player.loans.is_empty() {
                ui.add_text("No outstanding loans.", window.m_size());

                ui.add_space(window.height() * 0.02);

                let button = ui.add(Button::new(
                    RichText::new("Take a new loan").size(window.m_size()),
                ));

                if button.clicked() {
                    ui_state.credit.tab = CreditTab::NewLoan;
                }
            } else {
                credit_overview(ui, ui_state, &player.loans, window);

                // if let Some(loan) = &ui_state.credit.repay {
                //     ui.add_space(window.height() * 0.02);
                // 
                //     ui.add_text("Repay loan early", window.l_size());
                // 
                //     ui.add_space(window.height() * 0.02);
                // 
                //     let repay_amount = ui_state.credit.repay_amount;
                // 
                //     ui.spacing_mut().slider_width = window.width() * 0.13;
                //     ui.add(
                //         Slider::new(
                //             &mut ui_state.credit.repay_amount,
                //             0..=loan.outstanding as u32,
                //         )
                //         .show_value(false)
                //         .text(RichText::new(repay_amount.to_string()).size(window.s_size())),
                //     );
                // 
                //     ui.add_space(window.height() * 0.02);
                // 
                //     let button = ui.add_enabled(
                //         ui_state.credit.repay_amount > 0,
                //         Button::new(RichText::new("✏  Repay loan").size(window.m_size())),
                //     );
                // }
            }
        }
        CreditTab::NewLoan => {
            let loan = Loan {
                id: create_guid(),
                provider: ui_state.credit.provider,
                principal: ui_state.credit.principal as f32,
                outstanding: ui_state.credit.principal as f32,
                interest_rate: ui_state.credit.provider.interest(
                    economy.interest.current(),
                    player.credit_score.current(),
                    &ui_state.credit.term,
                ),
                kind: ui_state.credit.kind.clone(),
                term: ui_state.credit.term.clone(),
                start_date: first_day_in_two_months(economy.date),
                defaults: 0,
            };

            ui.add_text("New loan", window.l_size());

            ui.add_space(window.height() * 0.02);

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
            let loans = player
                .loans
                .iter()
                .filter(|l| l.provider == ui_state.credit.provider)
                .collect::<Vec<_>>();
            let has_loans = loans.iter().any(|l| l.outstanding > l.principal * 0.5);

            let mut button = ui.add_enabled(
                ui_state.credit.principal > 0 && !has_loans,
                Button::new(RichText::new("Take the loan").size(window.m_size())),
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
                messages.info("Loan acquired!");
            }

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
                .on_hover_text(
                    "Percentage of the principal that must be paid as interest every year.",
                );
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
        }
        CreditTab::P2P => {}
    }
}

pub fn credit_overview(ui: &mut Ui, ui_state: &mut UiState, loans: &Vec<Loan>, window: &Window) {
    let columns = [
        "Id",
        "Maturity",
        "Outstanding",
        "Principal",
        "Interest",
        "Kind",
        "Defaults",
    ];

    Frame::new()
        .inner_margin(ui.spacing().menu_margin)
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .show(ui, |ui| {
            TableBuilder::new(ui)
                .sense(Sense::click())
                .columns(Column::remainder(), columns.len())
                .header(30., |mut header| {
                    for col in columns {
                        header.col(|ui| {
                            ui.label(RichText::new(col).size(window.s_size()).strong());
                        });
                    }
                })
                .body(|mut body| {
                    for loan in loans {
                        let content = [
                            &loan.id,
                            &loan.maturity_date().format(DATE_FORMAT).to_string(),
                            &loan.outstanding.floor().to_string(),
                            &loan.principal.to_string(),
                            &loan.interest_rate.to_string(),
                            &loan.kind.to_name(),
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
