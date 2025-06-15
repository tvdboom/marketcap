use bevy::prelude::*;
use bevy_egui::egui::{Frame, Sense, Ui};
use egui_extras::{Column, TableBuilder};
use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::core::constants::{CURRENCY, DATE_FORMAT};
use crate::core::global_economy::GlobalEconomy;
use crate::core::loans::Loan;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::orders::{Order, OrderKind, OrderStatus};
use crate::core::player::{OwnedInstrument, Player};
use crate::core::ui::state::{CreditTab, ModalInfo, OrderOptions, OverviewTab, Tab, UiState};
use crate::core::ui::utils::CustomUi;
use crate::utils::NameFromEnum;

pub fn overview_panel(
    ui: &mut Ui,
    state: &mut UiState,
    economy: &GlobalEconomy,
    player: &mut Player,
    messages: &mut EventWriter<MessageEv>,
    window: &Window,
) {
    ui.horizontal(|ui| {
        for tab in OverviewTab::iter() {
            ui.selectable_value(
                &mut state.overview.tab,
                tab,
                format!("{}  {}", tab.emoji(), tab.to_name()),
            );
        }
    });

    ui.separator();

    match state.overview.tab {
        OverviewTab::Portfolio => {
            ui.add_combobox(
                "Commodities",
                [
                    OrderOptions::Name,
                    OrderOptions::Price,
                    OrderOptions::OwnedAmount,
                    OrderOptions::OwnedValue,
                ]
                .into(),
                &mut state.overview.commodities,
                window,
            );

            let mut commodities = player
                .commodities()
                .into_iter()
                .sorted_by(|a, b| match state.overview.commodities.order {
                    OrderOptions::Name => a.kind.lowername().cmp(&b.kind.lowername()),
                    OrderOptions::Price => economy
                        .get_current(&a.kind)
                        .partial_cmp(&economy.get_current(&b.kind))
                        .unwrap(),
                    OrderOptions::OwnedAmount => a.amount.cmp(&b.amount),
                    OrderOptions::OwnedValue => player
                        .get_value(&a.kind, economy)
                        .partial_cmp(&player.get_value(&b.kind, economy))
                        .unwrap(),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();

            if state.overview.commodities.descending {
                commodities.reverse();
            }

            if !commodities.is_empty() {
                commodity_overview(ui, state, &economy, commodities);
                ui.small("Click on a row to trade that commodity.");
            } else {
                ui.add_space(window.height() * 0.02);

                ui.label("You don't own any commodities yet.");

                ui.add_space(window.height() * 0.02);

                if ui.button("Buy commodities").clicked() {
                    state.tab = Tab::Commodities;
                }
            }
        },
        OverviewTab::OrderBook => {
            ui.add_combobox(
                "Pending orders",
                [
                    OrderOptions::Name,
                    OrderOptions::Created,
                    OrderOptions::Price,
                ]
                .into(),
                &mut state.overview.pending,
                window,
            );

            let mut pending = player
                .pending_orders()
                .into_iter()
                .sorted_by(|a, b| match state.overview.pending.order {
                    OrderOptions::Name => a.instrument.lowername().cmp(&b.instrument.lowername()),
                    OrderOptions::Created => a.created.cmp(&b.created),
                    OrderOptions::Price => a.threshold.cmp(&b.threshold),
                    _ => unreachable!(),
                })
                .cloned()
                .collect::<Vec<_>>();

            if state.overview.pending.descending {
                pending.reverse();
            }

            if pending.is_empty() {
                ui.add_space(window.height() * 0.02);

                ui.label("No pending orders.");
            } else {
                pending_order_table(ui, pending, economy, player, messages);
                ui.small("Click on a row to cancel the order.");
            }

            ui.add_space(window.height() * 0.05);

            ui.add_combobox(
                "Processed orders",
                [
                    OrderOptions::Name,
                    OrderOptions::Created,
                    OrderOptions::Processed,
                    OrderOptions::Price,
                    OrderOptions::Status,
                ]
                .into(),
                &mut state.overview.processed,
                window,
            );

            let mut processed = player
                .processed_orders()
                .into_iter()
                .sorted_by(|a, b| match state.overview.processed.order {
                    OrderOptions::Name => a.instrument.lowername().cmp(&b.instrument.lowername()),
                    OrderOptions::Created => a.created.cmp(&b.created),
                    OrderOptions::Price => a.threshold.cmp(&b.threshold),
                    OrderOptions::Processed => a.processed.cmp(&b.processed),
                    OrderOptions::Status => a.status.to_lowername().cmp(&b.status.to_lowername()),
                    _ => unreachable!(),
                })
                .cloned()
                .collect::<Vec<_>>();

            if state.overview.processed.descending {
                processed.reverse();
            }

            if processed.is_empty() {
                ui.add_space(window.height() * 0.02);

                ui.label("No processed orders.");
            } else {
                processed_order_table(ui, state, processed, economy);
                ui.small("Click on a row to recreate the order.");
            }
        },
        OverviewTab::Debts => {
            ui.heading("Outstanding loans");

            ui.add_space(window.height() * 0.02);

            if player.loans.is_empty() {
                ui.label("No outstanding loans.");

                ui.add_space(window.height() * 0.02);

                if ui.button("Take a new loan").clicked() {
                    state.tab = Tab::Credit;
                    state.credit.tab = CreditTab::NewLoan;
                }
            } else {
                loan_overview(ui, state, &player.loans);
                ui.small("Click on a row to repay the loan early.");
            }
        },
    }
}

pub fn commodity_overview(
    ui: &mut Ui,
    state: &mut UiState,
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
                            instrument.name(),
                            format!("{:.0} {CURRENCY}", instrument.current()),
                            format!("{} {}", owned.amount, instrument.unit()),
                            format!(
                                "{} {CURRENCY}",
                                (owned.amount as f32 * instrument.current()) as u32
                            ),
                            format!(
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
                                state.modal = Some(owned.kind.clone());
                            }
                        });
                    }
                });
        });
}

pub fn pending_order_table(
    ui: &mut Ui,
    orders: Vec<Order>,
    economy: &GlobalEconomy,
    player: &mut Player,
    messages: &mut EventWriter<MessageEv>,
) {
    let columns = [
        "Created",
        "Name",
        "Order",
        "Kind",
        "Amount",
        "Limit price",
        "Current price",
    ];

    Frame::new()
        .inner_margin(ui.spacing().menu_margin)
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .show(ui, |ui| {
            TableBuilder::new(ui)
                .id_salt("pending_order_table")
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
                            order.created.format(DATE_FORMAT).to_string(),
                            order.instrument.name(),
                            order.command.to_name(),
                            order.kind.abbr(),
                            format!("{} {}", order.amount, instrument.unit()),
                            format!("{:.0} {CURRENCY}", order.threshold),
                            format!("{:.0} {CURRENCY}", instrument.current()),
                        ];

                        body.row(30., |mut row| {
                            for col in content {
                                row.col(|ui| {
                                    ui.label(col);
                                });
                            }

                            if row.response().clicked() {
                                if let Some(order) =
                                    player.orders.iter_mut().find(|o| o.id == order.id)
                                {
                                    order.status = OrderStatus::Canceled;
                                }

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

pub fn processed_order_table(
    ui: &mut Ui,
    state: &mut UiState,
    orders: Vec<Order>,
    economy: &GlobalEconomy,
) {
    let columns = [
        "Created",
        "Processed",
        "Name",
        "Order",
        "Kind",
        "Amount",
        "Price",
        "Threshold",
        "Bound",
        "Status",
        "Reason",
    ];

    Frame::new()
        .inner_margin(ui.spacing().menu_margin)
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .show(ui, |ui| {
            TableBuilder::new(ui)
                .id_salt("processed_order_table")
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
                            order.created.format(DATE_FORMAT).to_string(),
                            order.processed.format(DATE_FORMAT).to_string(),
                            order.instrument.name(),
                            order.command.to_name(),
                            order.kind.abbr(),
                            format!("{} {}", order.amount, instrument.unit()),
                            format!("{:.0} {CURRENCY}", order.price),
                            if order.kind == OrderKind::LimitOrder {
                                format!("{} {CURRENCY}", order.threshold)
                            } else {
                                format!("{}%", order.threshold)
                            },
                            if order.lower_bound {
                                "▼".to_string()
                            } else {
                                "▲".to_string()
                            },
                            order.status.to_name(),
                            order.status.reason(),
                        ];

                        body.row(30., |mut row| {
                            for col in content {
                                row.col(|ui| {
                                    ui.label(col);
                                });
                            }

                            if row.response().clicked() {
                                state.modal = Some(order.instrument.clone());
                                state.modal_info = ModalInfo {
                                    tab: order.kind.clone(),
                                    amount: order.amount,
                                    limit_stop: order.threshold,
                                    trailing_stop: order.threshold,
                                    lower_bound: order.lower_bound,
                                };
                            }
                        });
                    }
                });
        });
}

pub fn loan_overview(ui: &mut Ui, state: &mut UiState, loans: &Vec<Loan>) {
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
                            loan.id.clone(),
                            loan.start_date.format(DATE_FORMAT).to_string(),
                            loan.maturity_date().format(DATE_FORMAT).to_string(),
                            loan.provider.to_name(),
                            format!("{} {CURRENCY}", &loan.principal),
                            format!("{:.0} {CURRENCY}", &loan.outstanding),
                            format!("{:.0} {CURRENCY}", &loan.next_installment_amount()),
                            format!("{}%", loan.interest_rate.to_string()),
                            loan.kind.to_name(),
                            loan.no_fee.to_string(),
                            loan.defaults.to_string(),
                        ];

                        body.row(30., |mut row| {
                            for col in content {
                                row.col(|ui| {
                                    ui.label(col);
                                });
                            }

                            if row.response().clicked() {
                                state.tab = Tab::Credit;
                                state.credit.tab = CreditTab::RepayLoan;
                                state.credit.repay = Some(loan.id.clone());
                            }
                        });
                    }
                });
        });
}
