use bevy::prelude::{EventWriter, Res, ResMut, Single, Window};
use bevy_egui::EguiContexts;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{ComboBox, Id, Image, Modal, Sense, Separator, Sides, Slider};
use chrono::NaiveDate;
use strum::IntoEnumIterator;

use crate::core::constants::CURRENCY;
use crate::core::factors::Factor;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::commodities::CommodityName;
use crate::core::instruments::crypto::CryptoName;
use crate::core::instruments::instrument::InstrumentKind;
use crate::core::loans::MarginLoan;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::orders::{Command, Order, OrderEv, OrderKind, OrderStatus};
use crate::core::player::Player;
use crate::core::resources::{Favourites, ImageIds};
use crate::core::ui::state::UiState;
use crate::core::ui::utils::{CustomUi, toggle};
use crate::utils::{EnhFloat, NameFromEnum, create_guid};

pub fn trade_modal(
    mut contexts: EguiContexts,
    mut state: ResMut<UiState>,
    economy: Res<GlobalEconomy>,
    mut player: ResMut<Player>,
    favourites: Res<Favourites>,
    mut order_ev: EventWriter<OrderEv>,
    mut message: EventWriter<MessageEv>,
    images: Res<ImageIds>,
    window: Single<&Window>,
) {
    let kind = if let Some(kind) = state.modal {
        kind
    } else {
        return;
    };

    let mut buy_clicked = false;
    let mut sell_clicked = false;
    let mut close_clicked = false;

    let instrument = economy.get(&kind);

    let owned = player.get_owned(&kind);
    let tab = state.modal_info.tab;
    let amount = state.modal_info.amount;
    let limit_stop = state.modal_info.limit_stop;
    let trailing_stop = state.modal_info.trailing_stop;
    let storage_costs = (amount * 30) as f32 * instrument.storage_cost();

    let mut price = instrument.current() * amount as f32;
    let loan = if state.modal_info.loan {
        let mut loan = MarginLoan::new(price, &economy, &player);

        // If the player already has a loan, use the largest interest rate and margin_frac
        if let Some(owned) = &player.get(&kind) {
            if let Some(l) = &owned.loan {
                loan.interest_rate = loan.interest_rate.max(l.interest_rate);
                loan.margin_frac = loan.margin_frac.max(l.margin_frac);
            }
        }

        loan
    } else {
        MarginLoan::default()
    };

    let modal = Modal::new(Id::new("modal")).show(contexts.ctx_mut(), |ui| {
        ui.set_min_width(window.width() * 0.5);

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ComboBox::from_id_salt("instrument")
                    .selected_text(format!("{}{}", if favourites.contains(&kind) {"❤ "} else {""}, instrument.name()))
                    .show_ui(ui, |ui| {
                        match kind {
                            InstrumentKind::Commodity(_) => {
                                for item in CommodityName::iter() {
                                    ui.selectable_value(
                                        &mut state.modal,
                                        Some(InstrumentKind::Commodity(item)),
                                        format!("{}{}", if favourites.contains(&InstrumentKind::Commodity(item)) {"❤ "} else {""}, item.to_name()),
                                    );
                                }
                            },
                            InstrumentKind::Crypto(_) => {
                                for item in CryptoName::iter() {
                                    ui.selectable_value(
                                        &mut state.modal,
                                        Some(InstrumentKind::Crypto(item)),
                                        format!("{}{}", if favourites.contains(&InstrumentKind::Crypto(item)) {"❤ "} else {""}, item.to_name()),
                                    );
                                }
                            },
                            _ => {}
                        }
                    });

                ui.add(Image::new(SizedTexture::new(
                    images.get(instrument.lowername().as_str()),
                    [window.height() * 0.2; 2],
                )));
            });

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    for tab in kind.order_options() {
                        ui.selectable_value(
                            &mut state.modal_info.tab,
                            tab,
                            format!("{}  {}", tab.emoji(), tab.to_name()),
                        )
                        .on_hover_text(tab.description());
                    }
                });

                ui.add_space(window.height() * 0.02);

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!(
                                "Price: {} {CURRENCY}{}",
                                instrument.current().clean(),
                                instrument.per_unit()
                            ));

                            ui.add_indicator(instrument.diff());
                        });

                        ui.label(format!("Owned: {owned} {}", instrument.unit()));
                        ui.label(format!(
                            "Value: {} {CURRENCY}",
                            player.get_value(&kind, &economy).clean()
                        ));

                        ui.spacing_mut().slider_width = window.width() * 0.17;

                        ui.horizontal(|ui| {
                            ui.label("Quantity:");

                            if tab == OrderKind::ShortSell {
                                ui.add(
                                    Slider::new(
                                        &mut state.modal_info.amount,
                                        0..=(MarginLoan::max_loan_debt(&economy, &player) / instrument.current()) as u32,
                                    )
                                        .show_value(false)
                                        .text(amount.to_string())
                                ).on_hover_text(
                                    "The maximum amount you can go short depends on the enterprise \
                                    value and the credit score."
                                );
                            } else {
                                ui.add(
                                    Slider::new(
                                        &mut state.modal_info.amount,
                                        0..=((player.cash.current() / instrument.current()) as i32).max(owned.abs()) as u32,
                                    )
                                        .show_value(false)
                                        .text(format!("{amount} {}", instrument.unit())),
                                );
                            }
                        });

                        if tab == OrderKind::LimitOrder {
                            ui.horizontal(|ui| {
                                ui.label("Limit stop:");

                                ui.add(
                                    Slider::new(
                                        &mut state.modal_info.limit_stop,
                                        0.0..=instrument.current() * 2.,
                                    )
                                        .step_by(instrument.current() as f64 / 50.)
                                        .show_value(false)
                                        .text(format!("{} {CURRENCY}", limit_stop.clean())),
                                )
                                .on_hover_text("If the price crosses this limit, the order is executed.");
                            });

                            price = limit_stop * amount as f32
                        } else if tab == OrderKind::TrailingOrder {
                            ui.horizontal(|ui| {
                                ui.label("Trailing stop:");

                                ui.add(
                                    Slider::new(&mut state.modal_info.trailing_stop, 5..=50)
                                        .show_value(false)
                                        .text(format!("{trailing_stop}%")),
                                )
                                .on_hover_text(
                                    "If the price evolves more than this percentage \
                                    away from the min/max price, the order is executed.",
                                );
                            });

                            ui.horizontal(|ui| {
                                ui.label("Bound:").on_hover_text(
                                    "Whether the trailing stop should be applied to the \
                                        upper or lower bound.",
                                );
                                let dir = ui.label(if state.modal_info.lower_bound {
                                    "▼ Lower"
                                } else {
                                    "▲ Upper"
                                }).on_hover_text(if state.modal_info.lower_bound {
                                    format!("Lower bound, i.e., the order is executed when te price surpasses {trailing_stop}% above the minimum.")
                                } else {
                                    format!("Upper bound, i.e., the order is executed when the price decreases {trailing_stop}% below the maximum.")
                                }).interact(Sense::click());

                                if dir.clicked() {
                                    state.modal_info.lower_bound = !state.modal_info.lower_bound;
                                }
                            });

                            price = if state.modal_info.lower_bound {
                                (100 + trailing_stop) as f32 / 100. * amount as f32 * instrument.current()
                            } else {
                                (100 - trailing_stop) as f32 / 100. * amount as f32 * instrument.current()
                            }
                        }

                        if matches!(kind, InstrumentKind::Commodity(_)) && tab != OrderKind::ShortSell {
                            ui.label(format!("Storage costs: {storage_costs:.0} {CURRENCY}/month"))
                                .on_hover_text(
                                    "Storage costs for the selected amount. This amount is deducted \
                                    from the proceeds of a sale to pay for the open costs of the current \
                                    month.",
                                );
                        }

                        if tab == OrderKind::TrailingOrder {
                            ui.label(format!("Trailing price: {} {CURRENCY}", if amount == 0 { 0. } else { (price / amount as f32).clean() }))
                                .on_hover_text(
                                    "If the price surpasses this value (greater for lower bound \
                                    or lesser for upper bound), the order is executed.",
                                );
                        }

                        if tab != OrderKind::ShortSell {
                            ui.label(format!("Total price: {} {CURRENCY}", price.clean()));
                        }
                    });

                    ui.add(Separator::default().vertical());
                    
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Margin loan: ").on_hover_text(
                                "A margin loan is a type of loan that allows leverage on the \
                                position. The investor borrows money from the broker to buy financial \
                                instruments using their existing investments as collateral."
                            );

                            if tab == OrderKind::ShortSell {
                                state.modal_info.loan = true;
                            }

                            ui.add_enabled(
                                tab != OrderKind::ShortSell,
                                toggle(&mut state.modal_info.loan),
                            ).on_disabled_hover_text("Short selling always requires a margin loan.");
                        });

                        if state.modal_info.loan {
                            ui.label(format!("Collateral: {} {CURRENCY}", loan.collateral.clean()))
                                .on_hover_text(
                                    "Amount to be set aside as collateral for the borrowed shares. \
                                    If the short position is closed with losses, the remaining debt is \
                                    paid from this deposit.",
                                );

                            //  Collateral at 50% + margin at 30%
                            let margin = loan.margin(if tab != OrderKind::ShortSell { amount as i32 } else {-(amount as i32)}).clean();
                            ui.label(format!("Margin: {margin} {CURRENCY} ({:.0}%)", loan.margin_frac * 100.))
                                .on_hover_text(
                                    "The maintenance margin is the price at which the short \
                                    position is automatically closed (forced liquidation). If this \
                                    happens, the losses are paid from the collateral.",
                                );

                            ui.label(format!("Interest: {:.1}%", loan.interest_rate))
                                .on_hover_text(
                                    "Interest to be paid to the broker for as long as the \
                                    position is open. The interest depends on the global interest \
                                    rate and the credit score. It is paid monthly from the cash \
                                    balance. If there is not enough cash available, it's deducted \
                                    from the collateral, reducing the margin limit. If a margin \
                                    loan already exists for this instrument, the largest interest \
                                    rate is used.",
                                );
                        }
                    });
                });

                ui.add_space(window.height() * 0.05);
                
                Sides::new().show(
                    ui,
                    |ui| {
                        if tab == OrderKind::ShortSell {
                            ui.add_enabled_ui(
                                amount > 0 && player.get_owned(&kind) <= 0 && player.cash.current() >= loan.collateral,
                                |ui| {
                                    let mut button = ui
                                        .add_modal_button("Open position", &window)
                                        .on_hover_text(format!(
                                            "Open short position for {} {}.",
                                            amount,
                                            instrument.lowername(),
                                        ));

                                    if player.get_owned(&kind) > 0 {
                                        button = button.on_disabled_hover_text(
                                            "Can't have a long and short position open on \
                                            the same instrument. First close the long position."
                                        );
                                    }

                                    if player.cash.current() < loan.collateral {
                                        button = button.on_disabled_hover_text("Not enough cash to pay the collateral.")
                                    }

                                    if button.clicked() {
                                        buy_clicked = true;
                                    }
                                },
                            );
                        } else {
                            ui.add_enabled_ui(
                                amount > 0
                                    && price > 0. // Price can be zero for dead cryptos
                                    && player.cash.current() >= loan.collateral
                                    && (tab != OrderKind::LimitOrder || limit_stop < instrument.current())
                                    && (tab != OrderKind::MarketOrder || player.cash.current() >= price),
                                |ui| {
                                    let button = ui
                                        .add_modal_button(
                                            if tab == OrderKind::MarketOrder {
                                                "Buy"
                                            } else {
                                                "Buy order"
                                            }, &window
                                        )
                                        .on_hover_text(format!(
                                            "Buy {} {}.",
                                            amount,
                                            instrument.lowername(),
                                        ));

                                    if button.clicked() {
                                        buy_clicked = true;
                                    }
                                },
                            );
                        }
                    },
                    |ui| {
                        if tab != OrderKind::ShortSell {
                            ui.add_enabled_ui(
                                price > 0.
                                    && owned > 0
                                    && (tab != OrderKind::LimitOrder || limit_stop > instrument.current()),
                                |ui| {
                                let button = ui
                                    .add_modal_button(
                                        if tab == OrderKind::MarketOrder {
                                            "Close position"
                                        } else {
                                            "Close order"
                                        }, &window
                                    )
                                    .on_hover_text(format!(
                                        "Sell all owned {}.",
                                        instrument.lowername()
                                    ))
                                    .on_disabled_hover_text(format!(
                                        "No {} to sell",
                                        instrument.lowername()
                                    ));

                                if button.clicked() {
                                    close_clicked = true;
                                }

                                ui.add_enabled_ui(
                                    amount > 0 && (tab != OrderKind::MarketOrder || owned >= amount as i32),
                                    |ui| {
                                        let button = ui
                                            .add_modal_button(
                                                if tab == OrderKind::MarketOrder {
                                                    "Sell"
                                                } else {
                                                    "Sell order"
                                                }, &window
                                            )
                                            .on_hover_text(format!(
                                                "Sell {} {}.",
                                                amount,
                                                instrument.lowername()
                                            ))
                                            .on_disabled_hover_text(format!(
                                                "Not enough {} to sell.",
                                                instrument.lowername(),
                                            ));

                                        if button.clicked() {
                                            sell_clicked = true;
                                        }
                                    },
                                );
                            });
                        }
                    },
                );
            });
        });
    });

    let command = if buy_clicked {
        Some(Command::Buy)
    } else if sell_clicked {
        Some(Command::Sell)
    } else if close_clicked {
        Some(Command::Close)
    } else {
        None
    };

    if let Some(command) = command {
        let mut order = Order {
            id: create_guid(),
            created: economy.date,
            instrument: kind,
            command,
            kind: tab,
            lower_bound: if tab == OrderKind::LimitOrder {
                limit_stop < instrument.current()
            } else {
                state.modal_info.lower_bound
            },
            amount: if tab != OrderKind::ShortSell {
                amount as i32
            } else {
                -(amount as i32)
            },
            price: price
                - if command == Command::Buy {
                    0.
                } else {
                    storage_costs
                },
            threshold: if tab == OrderKind::LimitOrder {
                limit_stop
            } else {
                trailing_stop as f32
            },
            loan: state.modal_info.loan.then_some(loan),
            bound: instrument.current(),
            processed: NaiveDate::default(),
            status: OrderStatus::Executed,
        };

        if matches!(tab, OrderKind::MarketOrder | OrderKind::ShortSell) {
            order.processed = economy.date;
            order_ev.write(OrderEv {
                id: order.id.clone(),
                price,
            });
        } else {
            order.status = OrderStatus::Pending;

            message.write(MessageEv {
                message: format!(
                    "Created {} {} order {}.",
                    tab.abbr().to_lowercase(),
                    command.to_lowername(),
                    order.id
                ),
                level: MessageLevel::Info,
            });
        }

        player.orders.push(order);
    }

    if modal.should_close() {
        state.modal = None;
    }
}
