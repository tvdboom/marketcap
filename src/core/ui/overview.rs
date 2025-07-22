use bevy::prelude::*;
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Frame, Image, ScrollArea, Sense, Ui};
use chrono::NaiveDate;
use egui_extras::{Column, TableBuilder};
use strum::IntoEnumIterator;

use crate::core::constants::{CURRENCY, DATE_FORMAT, NA};
use crate::core::derivatives::{Derivative, DerivativeAction, DerivativeKind};
use crate::core::events::EconomicEvent;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::instrument::InstrumentKind;
use crate::core::loans::TermLoan;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::orders::{Command, Order, OrderKind, OrderStatus};
use crate::core::player::{OwnedInstrument, Player};
use crate::core::research::TechName;
use crate::core::resources::ImageIds;
use crate::core::ui::state::{CreditTab, ModalInfo, OrderOptions, OverviewTab, Tab, UiState};
use crate::core::ui::utils::{CustomUi, toggle};
use crate::utils::{EnhFloat, NameFromEnum};

pub fn overview_panel(
    ui: &mut Ui,
    state: &mut UiState,
    economy: &GlobalEconomy,
    player: &mut Player,
    messages: &mut EventWriter<MessageEv>,
    images: &ImageIds,
    window: &Window,
) {
    ui.horizontal(|ui| {
        for tab in OverviewTab::iter() {
            if tab != OverviewTab::Derivatives || player.has_tech(&TechName::Futures) {
                ui.selectable_value(
                    &mut state.overview.tab,
                    tab,
                    format!("{}  {}", tab.emoji(), tab.to_name()),
                );
            }
        }
    });

    ui.separator();

    ScrollArea::vertical().show(ui, |ui| match state.overview.tab {
        OverviewTab::Portfolio => {
            let stocks = OrderOptions::sort_owned_instrument(
                player.stocks(),
                &state.overview.stocks,
                economy,
                player,
            );

            let bonds = OrderOptions::sort_owned_instrument(
                player.bonds(),
                &state.overview.bonds,
                economy,
                player,
            );

            let forex = OrderOptions::sort_owned_instrument(
                player.forex(),
                &state.overview.forex,
                economy,
                player,
            );

            let commodities = OrderOptions::sort_owned_instrument(
                player.commodities(),
                &state.overview.commodities,
                economy,
                player,
            );

            let crypto = OrderOptions::sort_owned_instrument(
                player.crypto(),
                &state.overview.crypto,
                economy,
                player,
            );

            if stocks.is_empty()
                && bonds.is_empty()
                && forex.is_empty()
                && commodities.is_empty()
                && crypto.is_empty()
            {
                ui.add_space(window.height() * 0.02);

                ui.label("No assets owned.");
            }

            if !stocks.is_empty() {
                ui.add_combobox(
                    "Stocks",
                    [
                        OrderOptions::Name,
                        OrderOptions::Price,
                        OrderOptions::OwnedAmount,
                        OrderOptions::OwnedValue,
                    ]
                    .into(),
                    &mut state.overview.stocks,
                    window,
                );

                instrument_table(ui, state, &economy, &player, stocks);
                ui.small("Click on a row to trade that stock.");
                ui.add_space(window.height() * 0.03);
            }

            if !bonds.is_empty() {
                ui.add_combobox(
                    "Bonds",
                    [
                        OrderOptions::Name,
                        OrderOptions::Maturity,
                        OrderOptions::OwnedAmount,
                        OrderOptions::OwnedValue,
                    ]
                    .into(),
                    &mut state.overview.bonds,
                    window,
                );

                instrument_table(ui, state, &economy, &player, bonds);
                ui.small("Click on a row to trade that bond.");
                ui.add_space(window.height() * 0.03);
            }

            if !forex.is_empty() {
                ui.add_combobox(
                    "Forex",
                    [
                        OrderOptions::Name,
                        OrderOptions::Price,
                        OrderOptions::OwnedAmount,
                        OrderOptions::OwnedValue,
                    ]
                    .into(),
                    &mut state.overview.bonds,
                    window,
                );

                instrument_table(ui, state, &economy, &player, forex);
                ui.small("Click on a row to trade that currency.");
                ui.add_space(window.height() * 0.03);
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

                instrument_table(ui, state, &economy, &player, commodities);
                ui.small("Click on a row to trade that commodity.");
                ui.add_space(window.height() * 0.03);
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

                instrument_table(ui, state, &economy, &player, crypto);
                ui.small("Click on a row to trade that crypto.");
            }
        },
        OverviewTab::OrderBook => {
            let pending =
                OrderOptions::sort_order(player.pending_orders(), &state.overview.pending_order);

            let processed = OrderOptions::sort_order(
                player.processed_orders(),
                &state.overview.processed_order,
            );

            if pending.is_empty() && processed.is_empty() {
                ui.add_space(window.height() * 0.02);

                ui.label("No orders placed.");
            }

            if !pending.is_empty() {
                ui.add_combobox(
                    "Pending orders",
                    [OrderOptions::Name, OrderOptions::Created, OrderOptions::Price].into(),
                    &mut state.overview.pending_order,
                    window,
                );

                pending_order_table(ui, pending, economy, player, messages);
                ui.small("Click on a row to cancel the order.");
            }

            ui.add_space(window.height() * 0.05);

            if !processed.is_empty() {
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
                    &mut state.overview.processed_order,
                    window,
                );

                processed_order_table(ui, state, processed, economy);
                ui.small("Click on a row to recreate the order.");
            }
        },
        OverviewTab::Derivatives => {
            let pending = OrderOptions::sort_derivative_mut(
                player.pending_derivatives_mut(),
                &state.overview.pending_derivative,
            );

            let no_pending = pending.is_empty();

            if !no_pending {
                ui.add_combobox(
                    "Pending derivatives",
                    [
                        OrderOptions::Name,
                        OrderOptions::Maturity,
                        OrderOptions::Kind,
                        OrderOptions::Action,
                        OrderOptions::OwnedAmount,
                        OrderOptions::OwnedValue,
                        OrderOptions::Price,
                        OrderOptions::Execute,
                    ]
                    .into(),
                    &mut state.overview.processed_order,
                    window,
                );

                pending_derivative_table(ui, state, economy, pending);
                ui.small("Click on a row to trade that instrument.");
            }

            let processed = OrderOptions::sort_derivative(
                player.processed_derivatives(),
                &state.overview.processed_derivative,
            );

            if !processed.is_empty() {
                ui.add_combobox(
                    "Processed derivatives",
                    [
                        OrderOptions::Name,
                        OrderOptions::Maturity,
                        OrderOptions::Kind,
                        OrderOptions::Status,
                        OrderOptions::OwnedAmount,
                        OrderOptions::OwnedValue,
                        OrderOptions::Price,
                        OrderOptions::Execute,
                    ]
                    .into(),
                    &mut state.overview.processed_order,
                    window,
                );

                processed_derivative_table(ui, state, economy, processed);
                ui.small("Click on a row to recreate the contract.");
            } else if no_pending {
                ui.add_space(window.height() * 0.02);

                ui.label("No derivatives traded.");
            }
        },
        OverviewTab::Debts => {
            let term_loans = OrderOptions::sort_term_loan(&player.loans, &state.overview.term_loan);

            let margin_loans = OrderOptions::sort_margin_loan(
                player.instruments.iter().filter(|o| o.loan.is_some()).collect(),
                &state.overview.margin_loan,
                economy,
            );

            if term_loans.is_empty() && margin_loans.is_empty() {
                ui.add_space(window.height() * 0.02);

                ui.label("No outstanding loans.");
            }

            if !term_loans.is_empty() {
                ui.add_combobox(
                    "Term loans",
                    [
                        OrderOptions::StartDate,
                        OrderOptions::Maturity,
                        OrderOptions::Provider,
                        OrderOptions::Principal,
                        OrderOptions::Outstanding,
                        OrderOptions::Installment,
                        OrderOptions::Interest,
                    ]
                    .into(),
                    &mut state.overview.term_loan,
                    window,
                );

                term_loan_table(ui, state, &term_loans);
                ui.small("Click on a row to repay the loan early.");
            }

            ui.add_space(window.height() * 0.05);

            if !margin_loans.is_empty() {
                ui.add_combobox(
                    "Margin loans",
                    [
                        OrderOptions::Name,
                        OrderOptions::Debt,
                        OrderOptions::Collateral,
                        OrderOptions::Interest,
                        OrderOptions::Price,
                        OrderOptions::Margin,
                    ]
                    .into(),
                    &mut state.overview.margin_loan,
                    window,
                );

                margin_loan_table(ui, state, &margin_loans, economy);
                ui.small("Click on a row to increase the loan's collateral.");
            }
        },
        OverviewTab::Events => {
            let mut active_events = economy.active_events();
            let mut historical_events = economy.historical_events();

            if active_events.is_empty() && historical_events.is_empty() {
                ui.add_space(window.height() * 0.02);

                ui.label("No events have occurred yet.");
            }

            if !active_events.is_empty() {
                ui.heading("Active events");
                event_table(ui, economy.date, true, &mut active_events, &images);
                ui.add_space(window.height() * 0.05);
            }

            if !historical_events.is_empty() {
                ui.heading("Historical events");
                event_table(ui, economy.date, false, &mut historical_events, &images);
            }
        },
    });
}

pub fn instrument_table(
    ui: &mut Ui,
    state: &mut UiState,
    economy: &GlobalEconomy,
    player: &Player,
    instruments: Vec<&OwnedInstrument>,
) {
    let kind = instruments.first().unwrap().kind;

    let mut columns = vec![
        "Name",
        if matches!(kind, InstrumentKind::Bond(_)) {
            "Maturity"
        } else {
            "Market price"
        },
        "Owned",
        "Value",
    ];

    if matches!(kind, InstrumentKind::Commodity(_)) {
        columns.push("Storage cost");
    }

    Frame::new()
        .inner_margin(ui.spacing().menu_margin)
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .show(ui, |ui| {
            TableBuilder::new(ui)
                .id_salt(format!("instrument_{}", kind.to_name()))
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
                    for owned in &instruments {
                        let instrument = economy.get(&owned.kind);

                        let mut content = vec![
                            instrument.name(),
                            if matches!(kind, InstrumentKind::Bond(_)) {
                                owned.maturity_date.format(DATE_FORMAT).to_string()
                            } else {
                                format!("{}{CURRENCY}", instrument.current().clean())
                            },
                            format!("{} {}", owned.amount, instrument.unit()),
                            format!(
                                "{}{CURRENCY}",
                                (owned.amount as f32 * instrument.current()).clean()
                            ),
                        ];

                        if matches!(kind, InstrumentKind::Commodity(_)) {
                            content.push(format!(
                                "{}{CURRENCY}/month",
                                (30. * owned.amount as f32
                                    * instrument.storage_cost(economy, player))
                                .max(0.) as u32
                            ));
                        }

                        body.row(30., |mut row| {
                            for col in content {
                                row.col(|ui| {
                                    ui.label(col);
                                });
                            }

                            if row.response().clicked() && instrument.current() > 0. {
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
    let columns = ["Created", "Name", "Order", "Kind", "Amount", "Threshold", "Market price"];

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
                            match order.kind {
                                OrderKind::LimitOrder => {
                                    format!("{}{CURRENCY}", order.threshold.clean())
                                },
                                OrderKind::TrailingOrder => format!("{}%", order.threshold.clean()),
                                _ => NA.to_string(),
                            },
                            format!("{:.0}{CURRENCY}", instrument.current()),
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
                            if order.kind == OrderKind::ShortSell {
                                Command::Sell.to_name()
                            } else {
                                order.command.to_name()
                            },
                            order.kind.abbr(),
                            format!("{} {}", order.amount, instrument.unit()),
                            format!("{:.0}{CURRENCY}", order.price),
                            match order.kind {
                                OrderKind::LimitOrder => format!("{}{CURRENCY}", order.threshold),
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
                                    amount: order.amount as u32,
                                    limit_stop: order.threshold,
                                    trailing_stop: order.threshold as u32,
                                    lower_bound: order.lower_bound,
                                    loan: order.loan.is_some(),
                                    ..state.modal_info.clone()
                                };
                            }
                        });
                    }
                });
        });
}

pub fn pending_derivative_table(
    ui: &mut Ui,
    state: &mut UiState,
    economy: &GlobalEconomy,
    derivatives: Vec<&mut Derivative>,
) {
    let columns = vec![
        "Name",
        "Maturity",
        "Kind",
        "Action",
        "Amount",
        "Market price",
        "Contract price",
        "Value",
        "Execute",
    ];

    Frame::new()
        .inner_margin(ui.spacing().menu_margin)
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .show(ui, |ui| {
            TableBuilder::new(ui)
                .id_salt("pending_derivative_table")
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
                    for derivative in derivatives {
                        let instrument = economy.get(&derivative.instrument);

                        let content = vec![
                            instrument.name(),
                            derivative.maturity_date().format(DATE_FORMAT).to_string(),
                            derivative.kind.to_name(),
                            derivative.action.to_name(),
                            derivative.amount.to_string(),
                            format!("{}{CURRENCY}", instrument.current().clean()),
                            format!(
                                "{}{CURRENCY} ({}{CURRENCY})",
                                derivative.price.clean(),
                                (derivative.price - instrument.current()).signed()
                            ),
                            format!(
                                "{}{CURRENCY}",
                                (derivative.amount as f32 * derivative.price).clean()
                            ),
                        ];

                        body.row(30., |mut row| {
                            for col in content {
                                row.col(|ui| {
                                    ui.label(col);
                                });
                            }

                            row.col(|ui| {
                                let button = ui
                                    .add_enabled(
                                        derivative.kind == DerivativeKind::Option
                                            && derivative.action == DerivativeAction::Bought,
                                        toggle(&mut derivative.execute),
                                    )
                                    .on_hover_text(
                                        "Whether to execute the option at maturity. Note that \
                                    manually changing the value will no longer update the option \
                                    automatically when prices changes unfavorably.",
                                    );

                                if button.clicked() {
                                    // The button will no longer be updated automatically
                                    derivative.force_execute = true;
                                }
                            });

                            if row.response().clicked() {
                                state.modal = Some(derivative.instrument.clone());
                                state.modal_info.tab = if derivative.kind == DerivativeKind::Future
                                {
                                    OrderKind::Futures
                                } else {
                                    OrderKind::Options
                                };
                            }
                        });
                    }
                });
        });
}

pub fn processed_derivative_table(
    ui: &mut Ui,
    state: &mut UiState,
    economy: &GlobalEconomy,
    derivatives: Vec<&Derivative>,
) {
    let columns =
        vec!["Name", "Maturity", "Kind", "Action", "Amount", "Transaction price", "Contract price"];

    Frame::new()
        .inner_margin(ui.spacing().menu_margin)
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .show(ui, |ui| {
            TableBuilder::new(ui)
                .id_salt("processed_derivative_table")
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
                    for derivative in derivatives {
                        let instrument = economy.get(&derivative.instrument);

                        let content = vec![
                            instrument.name(),
                            derivative.maturity_date().format(DATE_FORMAT).to_string(),
                            derivative.kind.to_name(),
                            derivative.action.to_name(),
                            derivative.amount.to_string(),
                            format!("{}{CURRENCY}", derivative.transaction_price.clean()),
                            format!(
                                "{}{CURRENCY} ({}{CURRENCY})",
                                derivative.price.clean(),
                                (derivative.price - derivative.transaction_price).signed()
                            ),
                        ];

                        body.row(30., |mut row| {
                            for col in content {
                                row.col(|ui| {
                                    ui.label(col);
                                });
                            }

                            if row.response().clicked() {
                                state.modal = Some(derivative.instrument.clone());
                                state.modal_info = ModalInfo {
                                    tab: if derivative.kind == DerivativeKind::Future {
                                        OrderKind::Futures
                                    } else {
                                        OrderKind::Options
                                    },
                                    amount: derivative.amount,
                                    derivative_term: derivative.term.clone(),
                                    ..state.modal_info.clone()
                                };
                            }
                        });
                    }
                });
        });
}

pub fn term_loan_table(ui: &mut Ui, state: &mut UiState, loans: &Vec<TermLoan>) {
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
    ];

    Frame::new()
        .inner_margin(ui.spacing().menu_margin)
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .show(ui, |ui| {
            TableBuilder::new(ui)
                .id_salt("term_loan_table")
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
                            format!("{}{CURRENCY}", &loan.principal),
                            format!("{:.0}{CURRENCY}", &loan.outstanding),
                            format!("{:.0}{CURRENCY}", &loan.next_installment_amount()),
                            format!("{:.1}%", loan.interest_rate),
                            loan.kind.to_name(),
                            loan.no_fee.to_string(),
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

pub fn margin_loan_table(
    ui: &mut Ui,
    state: &mut UiState,
    loans: &Vec<OwnedInstrument>,
    economy: &GlobalEconomy,
) {
    let columns = [
        "Id",
        "Name",
        "Amount",
        "Debt",
        "Collateral",
        "Interest",
        "Margin frac",
        "Market price",
        "Margin",
    ];

    Frame::new()
        .inner_margin(ui.spacing().menu_margin)
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .show(ui, |ui| {
            TableBuilder::new(ui)
                .id_salt("margin_loan_table")
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
                    for owned in loans {
                        let instrument = economy.get(&owned.kind);
                        let loan = owned.loan.as_ref().unwrap();
                        let content = [
                            loan.id.clone(),
                            instrument.name(),
                            format!("{} {}", owned.amount, instrument.unit()),
                            format!("{}{CURRENCY}", loan.debt.clean()),
                            format!("{}{CURRENCY}", loan.collateral.clean()),
                            format!("{:.1}%", loan.interest_rate),
                            format!("{:.0}%", loan.margin_frac * 100.),
                            format!("{}{CURRENCY}", economy.get_price(&owned.kind).clean()),
                            format!("{}{CURRENCY}", loan.margin(owned.amount).clean()),
                        ];

                        body.row(30., |mut row| {
                            for col in content {
                                row.col(|ui| {
                                    ui.label(col);
                                });
                            }

                            if row.response().clicked() {
                                state.tab = Tab::Credit;
                                state.credit.tab = CreditTab::IncreaseCollateral;
                                state.credit.increase = Some(loan.id.clone());
                            }
                        });
                    }
                });
        });
}

pub fn event_table(
    ui: &mut Ui,
    today: NaiveDate,
    active: bool,
    events: &mut Vec<&EconomicEvent>,
    images: &ImageIds,
) {
    let columns = vec!["Name", "Start date", "Duration", "Description"];

    // Sort events by start date, newest first
    events.sort_by(|a, b| b.start_date.cmp(&a.start_date));

    Frame::new()
        .inner_margin(ui.spacing().menu_margin)
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .show(ui, |ui| {
            ui.set_min_height((90. * events.len() as f32).min(500.));

            TableBuilder::new(ui)
                .id_salt(if active {
                    "event_table_active"
                } else {
                    "event_table_historical"
                })
                .striped(false)
                .column(Column::initial(300.))
                .columns(Column::initial(100.), 2)
                .column(Column::remainder())
                .header(30., |mut header| {
                    for col in columns {
                        header.col(|ui| {
                            ui.strong(col);
                        });
                    }
                })
                .body(|mut body| {
                    for event in events {
                        let content = vec![
                            event.title(),
                            event.start_date.format(DATE_FORMAT).to_string(),
                            if event.duration == 1 {
                                NA.to_string()
                            } else {
                                format!(
                                    "{} days",
                                    (today - event.start_date)
                                        .num_days()
                                        .min(event.duration as i64)
                                )
                            },
                            event.description(),
                        ];

                        body.row(30., |mut row| {
                            for (i, col) in content.iter().enumerate() {
                                row.col(|ui| {
                                    ui.label(col);

                                    if i == 0 {
                                        ui.add(Image::new(SizedTexture::new(
                                            images.get(event.image().as_str()),
                                            [160., 90.],
                                        )));
                                    }
                                });
                            }
                        });
                    }
                });
        });
}
