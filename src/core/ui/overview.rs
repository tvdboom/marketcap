use bevy::prelude::*;
use bevy_egui::egui::{Align, ComboBox, Frame, Layout, Sense, Ui};
use egui_extras::{Column, TableBuilder};
use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::core::constants::{CURRENCY, DATE_FORMAT};
use crate::core::global_economy::GlobalEconomy;
use crate::core::loans::Loan;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::player::{OwnedInstrument, Player};
use crate::core::ui::state::{CreditTab, OrderOptions, OverviewTab, Tab, UiState};
use crate::utils::NameFromEnum;

pub fn overview_panel(
    ui: &mut Ui,
    ui_state: &mut UiState,
    economy: &GlobalEconomy,
    player: &mut Player,
    messages: &mut EventWriter<MessageEv>,
    window: &Window,
) {
    ui.horizontal(|ui| {
        for tab in OverviewTab::iter() {
            ui.selectable_value(
                &mut ui_state.overview.tab,
                tab,
                format!("{}  {}", tab.emoji(), tab.to_name()),
            );
        }
    });

    ui.separator();

    match ui_state.overview.tab {
        OverviewTab::Portfolio => {
            let commodities = player.commodities();
            if !commodities.is_empty() {
                ui.heading("Commodities");

                ui.add_space(window.height() * 0.02);

                commodity_overview(ui, ui_state, &economy, player.commodities());
                ui.small("Click on a row to trade that commodity.");
            } else {
                ui.add_space(window.height() * 0.02);

                ui.label("You don't own any instruments yet.");

                ui.add_space(window.height() * 0.02);

                if ui.button("Buy stocks").clicked() {
                    ui_state.tab = Tab::Stocks;
                }
            }
        },
        OverviewTab::OrderBook => {
            if player.orders.is_empty() {
                ui.add_space(window.height() * 0.02);

                ui.label("No orders in the order book.");
            } else {
                order_overview(ui, ui_state, economy, player, messages);
                ui.small("Click on a row to cancel the order.");
            }
        },
        OverviewTab::Debts => {
            ui.heading("Outstanding loans");

            ui.add_space(window.height() * 0.02);

            if player.loans.is_empty() {
                ui.label("No outstanding loans.");

                ui.add_space(window.height() * 0.02);

                if ui.button("Take a new loan").clicked() {
                    ui_state.tab = Tab::Credit;
                    ui_state.credit.tab = CreditTab::NewLoan;
                }
            } else {
                loan_overview(ui, ui_state, &player.loans);
                ui.small("Click on a row to repay the loan early.");
            }
        },
    }
}

pub fn commodity_overview(
    ui: &mut Ui,
    ui_state: &mut UiState,
    economy: &GlobalEconomy,
    commodities: Vec<&OwnedInstrument>,
) {
    let columns = ["Name", "Price", "Owned", "Value", "Storage costs"];

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
                            ui.strong(col);
                        });
                    }
                })
                .body(|mut body| {
                    for owned in commodities.iter().sorted_by_key(|o| o.kind.lowername()) {
                        let instrument = economy.get(&owned.kind);

                        let content = [
                            &instrument.name(),
                            &format!("{} {CURRENCY}", instrument.current()),
                            &format!("{} {}", owned.amount, instrument.unit()),
                            &format!(
                                "{} {CURRENCY}",
                                (owned.amount as f32 * instrument.current()) as u32
                            ),
                            &format!(
                                "{} {CURRENCY}/month",
                                (30. * owned.amount as f32 * instrument.storage_cost()) as u32
                            ),
                        ];

                        body.row(30., |mut row| {
                            for col in content {
                                row.col(|ui| {
                                    ui.label(col);
                                });
                            }

                            if row.response().clicked() {
                                ui_state.active_modal = Some(owned.kind.clone());
                            }
                        });
                    }
                });
        });
}

pub fn order_overview(
    ui: &mut Ui,
    ui_state: &mut UiState,
    economy: &GlobalEconomy,
    player: &mut Player,
    messages: &mut EventWriter<MessageEv>,
) {
    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
        ComboBox::from_id_salt("order")
            .selected_text("Order by")
            .show_ui(ui, |ui| {
                for order in [
                    OrderOptions::Alphabetical,
                    OrderOptions::HighestPrice,
                    OrderOptions::LowestPrice,
                ] {
                    ui.selectable_value(
                        &mut ui_state.overview.order_book_order,
                        order,
                        order.to_name(),
                    );
                }
            });
    });

    let columns = [
        "Id",
        "Name",
        "Order",
        "Amount",
        "Limit price",
        "Current price",
    ];

    let orders =
        player
            .orders
            .iter()
            .cloned()
            .sorted_by(|a, b| match ui_state.overview.order_book_order {
                OrderOptions::Alphabetical => {
                    a.instrument.lowername().cmp(&b.instrument.lowername())
                },
                OrderOptions::LowestPrice => a.price.cmp(&b.price),
                OrderOptions::HighestPrice => b.price.cmp(&a.price),
                _ => unreachable!(),
            });

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
                            ui.strong(col);
                        });
                    }
                })
                .body(|mut body| {
                    for order in orders {
                        let instrument = economy.get(&order.instrument);

                        let content = [
                            &order.id,
                            &order.instrument.name(),
                            &order.order.to_name(),
                            &format!("{} {}", order.amount, instrument.unit()),
                            &format!("{:.0} {CURRENCY}", order.price),
                            &format!("{:.0} {CURRENCY}", instrument.current()),
                        ];

                        body.row(30., |mut row| {
                            for col in content {
                                row.col(|ui| {
                                    ui.label(col);
                                });
                            }

                            if row.response().clicked() {
                                player.orders.retain(|o| o.id != order.id);

                                messages.write(MessageEv {
                                    message: format!("Canceled order {}.", order.id),
                                    level: MessageLevel::Info,
                                });
                            }
                        });
                    }
                });
        });
}

pub fn loan_overview(ui: &mut Ui, ui_state: &mut UiState, loans: &Vec<Loan>) {
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
                            ui.strong(col);
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
                            &format!("{:.0} {CURRENCY}", &loan.outstanding),
                            &format!("{:.0} {CURRENCY}", &loan.next_installment_amount()),
                            &format!("{}%", loan.interest_rate.to_string()),
                            &loan.kind.to_name(),
                            &loan.no_fee.to_string(),
                            &loan.defaults.to_string(),
                        ];

                        body.row(30., |mut row| {
                            for col in content {
                                row.col(|ui| {
                                    ui.label(col);
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
