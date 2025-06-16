use bevy::prelude::*;
use bevy_egui::egui::{Frame, Sense, Ui};
use egui_extras::{Column, TableBuilder};
use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::core::constants::{CURRENCY, DATE_FORMAT, NA};
use crate::core::global_economy::GlobalEconomy;
use crate::core::loans::Loan;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::orders::{Order, OrderKind, OrderStatus};
use crate::core::player::{InstrumentKind, OwnedInstrument, Player};
use crate::core::ui::state::{CreditTab, ModalInfo, OrderOptions, OverviewTab, Tab, UiState};
use crate::core::ui::utils::CustomUi;
use crate::utils::{EnhFloat, NameFromEnum};

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
            let commodities = OrderOptions::sort_owned(
                &mut player.commodities(),
                &state.overview.commodities,
                economy,
                player,
            );

            let crypto = OrderOptions::sort_owned(
                &mut player.crypto(),
                &state.overview.crypto,
                economy,
                player,
            );

            if commodities.is_empty() && crypto.is_empty() {
                ui.add_space(window.height() * 0.02);

                ui.label("No assets owned.");
            }

            if !commodities.is_empty() {
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

                instrument_table(ui, state, &economy, commodities);
                ui.small("Click on a row to trade that commodity.");
            }

            if !crypto.is_empty() {
                ui.add_combobox(
                    "Cryptocurrencies",
                    [
                        OrderOptions::Name,
                        OrderOptions::Price,
                        OrderOptions::OwnedAmount,
                        OrderOptions::OwnedValue,
                    ]
                    .into(),
                    &mut state.overview.crypto,
                    window,
                );

                instrument_table(ui, state, &economy, crypto);
                ui.small("Click on a row to trade that crypto.");
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

            let pending =
                OrderOptions::sort_order(player.pending_orders(), &state.overview.pending);

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

            let processed =
                OrderOptions::sort_order(player.processed_orders(), &state.overview.processed);

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

pub fn instrument_table(
    ui: &mut Ui,
    state: &mut UiState,
    economy: &GlobalEconomy,
    instruments: Vec<&OwnedInstrument>,
) {
    let mut columns = vec!["Name", "Price", "Owned", "Value"];

    if matches!(
        instruments.first().unwrap().kind,
        InstrumentKind::Commodity(_)
    ) {
        columns.push("Storage cost");
    }

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
                    for owned in instruments.iter().sorted_by_key(|o| o.kind.lowername()) {
                        let instrument = economy.get(&owned.kind);

                        let mut content = vec![
                            instrument.name(),
                            format!("{} {CURRENCY}", instrument.current().clean()),
                            format!("{} {}", owned.amount, instrument.unit()),
                            format!(
                                "{} {CURRENCY}",
                                (owned.amount as f32 * instrument.current()) as u32
                            ),
                        ];

                        if matches!(
                            instruments.first().unwrap().kind,
                            InstrumentKind::Commodity(_)
                        ) {
                            content.push(format!(
                                "{} {CURRENCY}/month",
                                (30. * owned.amount as f32 * instrument.storage_cost()) as u32
                            ));
                        }

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
                            match order.kind {
                                OrderKind::LimitOrder => format!("{} {CURRENCY}", order.threshold),
                                OrderKind::TrailingOrder => format!("{}%", order.threshold),
                                _ => NA.to_string(),
                            },
                            match order.kind {
                                OrderKind::LimitOrder | OrderKind::TrailingOrder => {
                                    if order.lower_bound {
                                        "▼".to_string()
                                    } else {
                                        "▲".to_string()
                                    }
                                },
                                _ => NA.to_string(),
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
                                    trailing_stop: order.threshold as u32,
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
