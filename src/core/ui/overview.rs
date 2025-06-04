use bevy::prelude::Window;
use bevy_egui::egui::{Button, Frame, Sense, Ui};
use egui_extras::{Column, TableBuilder};
use strum::IntoEnumIterator;

use crate::core::constants::{CURRENCY, DATE_FORMAT};
use crate::core::loans::Loan;
use crate::core::player::Player;
use crate::core::ui::state::{CreditTab, OverviewTab, Tab, UiState};
use crate::core::ui::utils::{CustomUi, TextSizes, add_text};
use crate::utils::NameFromEnum;

pub fn overview_panel(ui: &mut Ui, ui_state: &mut UiState, player: &Player, window: &Window) {
    ui.horizontal(|ui| {
        for tab in OverviewTab::iter() {
            ui.selectable_value(
                &mut ui_state.overview,
                tab,
                add_text(
                    format!("{}  {}", tab.emoji(), tab.to_name()),
                    window.l_size(),
                ),
            );
        }
    });

    ui.separator();

    match ui_state.overview {
        OverviewTab::Portfolio => {
            ui.add_text("Commodities", window.l_size());

            loan_overview(ui, ui_state, &player.loans, window);

            ui.add_text("Click on a row to repay the loan early.", window.xs_size());
        },
        OverviewTab::OrderBook => {},
        OverviewTab::Debts => {
            ui.add_text("Outstanding loans", window.l_size());

            ui.add_space(window.height() * 0.02);

            if player.loans.is_empty() {
                ui.add_text("No outstanding loans.", window.s_size());

                ui.add_space(window.height() * 0.02);

                let button = ui.add(Button::new(add_text("Take a new loan", window.m_size())));

                if button.clicked() {
                    ui_state.tab = Tab::Credit;
                    ui_state.credit.tab = CreditTab::NewLoan;
                }
            } else {
                loan_overview(ui, ui_state, &player.loans, window);

                ui.add_text("Click on a row to repay the loan early.", window.xs_size());
            }
        },
    }
}

pub fn loan_overview(ui: &mut Ui, ui_state: &mut UiState, loans: &Vec<Loan>, window: &Window) {
    let columns = [
        "Id",
        "Start date",
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
                .columns(Column::remainder(), columns.len())
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
                            &loan.start_date.format(DATE_FORMAT).to_string(),
                            &loan.maturity_date().format(DATE_FORMAT).to_string(),
                            &loan.provider.to_name(),
                            &format!("{} {CURRENCY}", &loan.principal),
                            &format!("{} {CURRENCY}", &loan.outstanding.floor()),
                            &format!("{} {CURRENCY}", &loan.next_installment_amount().floor()),
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
                                ui_state.tab = Tab::Credit;
                                ui_state.credit.tab = CreditTab::RepayLoan;
                                ui_state.credit.repay = Some(loan.id.clone());
                            }
                        });
                    }
                });
        });
}
