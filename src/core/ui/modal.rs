use bevy::prelude::{EventWriter, Res, ResMut, Single, Window};
use bevy_egui::EguiContexts;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{
    ComboBox, Id, Image, Modal, RichText, ScrollArea, Sense, Separator, Sides, Slider,
};
use chrono::NaiveDate;
use strum::IntoEnumIterator;

use crate::core::constants::CURRENCY;
use crate::core::countries::CountryName;
use crate::core::derivatives::{
    Derivative, DerivativeAction, DerivativeKind, DerivativeTerm, OptionKind,
};
use crate::core::factors::Factor;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::bonds::BondIssuer;
use crate::core::instruments::commodities::CommodityName;
use crate::core::instruments::crypto::CryptoName;
use crate::core::instruments::forex::CurrencyName;
use crate::core::instruments::instrument::InstrumentKind;
use crate::core::instruments::stocks::Company;
use crate::core::loans::MarginLoan;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::orders::{Command, Order, OrderEv, OrderKind, OrderStatus};
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::ui::state::{OverviewTab, Tab, UiState};
use crate::core::ui::utils::{CustomUi, toggle};
use crate::utils::{EnhFloat, NameFromEnum, create_guid};

pub fn trade_modal(
    mut contexts: EguiContexts,
    mut state: ResMut<UiState>,
    economy: Res<GlobalEconomy>,
    mut player: ResMut<Player>,
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
    let mut buy_put_clicked = false;
    let mut sell_clicked = false;
    let mut sell_call_clicked = false;
    let mut close_clicked = false;

    let instrument = economy.get(&kind);

    let owned = player.get_owned(&kind);
    let tab = state.modal_info.tab;
    let amount = state.modal_info.amount;
    let limit_price = state.modal_info.limit_stop;
    let trailing_stop = state.modal_info.trailing_stop;
    let storage_costs = (amount * 30) as f32 * instrument.storage_cost();

    let derivatives = player
        .derivatives
        .iter()
        .cloned()
        .filter(|d| {
            d.instrument == kind
                && d.kind
                    == if tab == OrderKind::Futures {
                        DerivativeKind::Future
                    } else {
                        DerivativeKind::Option
                    }
        })
        .collect::<Vec<_>>();

    let strike_price = if tab == OrderKind::Futures {
        instrument.future_price(
            economy.interest.current(),
            state.modal_info.derivative_term.years(),
        )
    } else {
        instrument.current() * (1. + state.modal_info.strike_percentage as f32 / 100.)
    };

    let call_price = instrument.option_price(
        strike_price,
        economy.interest.current(),
        state.modal_info.derivative_term.years(),
        OptionKind::Call,
    );

    let put_price = instrument.option_price(
        strike_price,
        economy.interest.current(),
        state.modal_info.derivative_term.years(),
        OptionKind::Put,
    );

    let mut total_price = instrument.current() * amount as f32;

    let loan = if state.modal_info.loan {
        let mut loan = MarginLoan::new(total_price, &economy, &player);

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

    let max_loan = MarginLoan::max_loan_debt(&economy, &player);
    let max_derivative_sell = Derivative::max_sell(&economy, &player);

    let modal = Modal::new(Id::new("modal")).show(contexts.ctx_mut(), |ui| {
        ui.set_width(window.width() * 0.6);

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ComboBox::from_id_salt("instrument")
                    .selected_text(format!("{}{}", if player.has_favourite(&kind) {"❤ "} else {""}, instrument.name()))
                    .show_ui(ui, |ui| {
                        let items: Vec<(InstrumentKind, String)> = match kind {
                            InstrumentKind::Stock(_) => Company::iter().map(|c| (InstrumentKind::Stock(c), c.to_name())).collect(),
                            InstrumentKind::Bond(issuer) => {
                                match issuer {
                                    BondIssuer::Government(_) => {
                                        CountryName::iter().map(|c| (InstrumentKind::Bond(BondIssuer::Government(c)), c.to_name())).collect()
                                    },
                                    BondIssuer::Corporate(_) => {
                                        Company::iter().map(|c| (InstrumentKind::Bond(BondIssuer::Corporate(c)), c.to_name())).collect()
                                    },
                                }
                            },
                            InstrumentKind::Forex(_) => CurrencyName::iter().map(|c| (InstrumentKind::Forex(c), c.to_name())).collect(),
                            InstrumentKind::Commodity(_) => CommodityName::iter().map(|c| (InstrumentKind::Commodity(c), c.to_name())).collect(),
                            InstrumentKind::Crypto(_) => CryptoName::iter().map(|c| (InstrumentKind::Crypto(c), c.to_name())).collect(),
                        };
        
                        for (instr, name) in items {
                            ui.selectable_value(
                                &mut state.modal,
                                Some(instr),
                                format!(
                                    "{}{}",
                                    if player.has_favourite(&instr) { "❤ " } else { "" },
                                    name
                                ),
                            );
                        }
                    });

                ui.add(Image::new(SizedTexture::new(
                    images.get(instrument.image().as_str()),
                    [window.height() * 0.2; 2],
                )))
                .on_hover_ui(|ui| {
                    ui.set_min_width(window.width() * 0.4);

                    ui.heading(instrument.name());
                    ui.add_space(window.height() * 0.01);
                    ui.label(instrument.description());

                    if !matches!(instrument.kind(), InstrumentKind::Bond(_)) {
                        ui.add_space(window.height() * 0.01);
                        ui.add_plot(instrument.all(), Some(player.pending_orders().into_iter().filter(|o| o.instrument == kind).collect()));
                    }
                });
            });

            ui.vertical(|ui| {
                ScrollArea::horizontal().show(ui, |ui| {
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
                });

                ui.add_space(window.height() * 0.02);

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(format!(
                                        "Price: {}{CURRENCY}{}",
                                        instrument.current().clean(),
                                        instrument.per_unit()
                                    ))
                                    .on_hover_text(format!("Current market price of {}.", instrument.lowername()));

                                    if !matches!(instrument.kind(), InstrumentKind::Bond(_)) {
                                        ui.add_indicator(instrument.diff());
                                    }
                                });

                                let clickable = ui.label(format!("Owned: {owned} {}", instrument.unit()))
                                    .on_hover_text("Amount of this instrument currently owned. Click to open the portfolio.")
                                    .interact(Sense::click());

                                if clickable.clicked() {
                                    state.modal = None;
                                    state.tab = Tab::Overview;
                                    state.overview.tab = OverviewTab::Portfolio;
                                }
                                
                                ui.label(format!(
                                    "Value: {}{CURRENCY}",
                                    player.get_value(&kind, &economy).clean()
                                ))
                                .on_hover_text(format!("Current market value of all owned {}.", instrument.lowername()));
                            });
                            
                            if !matches!(tab, OrderKind::MarketOrder | OrderKind::ShortSell) {
                                ui.add_space(window.width() * 0.01);
                                ui.add(Separator::default().vertical());
                                ui.add_space(window.width() * 0.01);

                                ui.vertical(|ui| {
                                    if matches!(tab, OrderKind::LimitOrder | OrderKind::TrailingOrder) {
                                        let price = if tab == OrderKind::LimitOrder {
                                            state.modal_info.limit_stop
                                        } else if state.modal_info.lower_bound {
                                            instrument.current() * (1. + trailing_stop as f32 / 100.)
                                        } else {
                                            instrument.current() * (1. - trailing_stop as f32 / 100.)
                                        };

                                        ui.label(format!("{} price: {}{CURRENCY}", tab.abbr(), price.clean()))
                                        .on_hover_text(
                                            "If the price surpasses this value, the order is executed.",
                                        );

                                        let n_orders = player.orders.iter().filter(|o| o.instrument == kind).count();

                                        let clickable = ui.label(format!("Orders placed: {n_orders}"))
                                            .on_hover_text("Amount of orders placed for this instrument. Click to open the order book.")
                                            .interact(Sense::click());

                                        if clickable.clicked() {
                                            state.modal = None;
                                            state.tab = Tab::Overview;
                                            state.overview.tab = OverviewTab::OrderBook;
                                        }
                                    } else {
                                        ui.label(format!(
                                            "{} price: {}{CURRENCY}{}  ({}{CURRENCY})",
                                            if tab == OrderKind::Futures {
                                                "Contract"
                                            } else {
                                                "Strike"
                                            },
                                            strike_price.clean(),
                                            instrument.per_unit(),
                                            (strike_price - instrument.current()).round().signed(),
                                        ))
                                            .on_hover_text(format!("Price for {} at the maturity date.", instrument.lowername()));

                                        let clickable = ui.label(format!("Owned: {}", derivatives.len()))
                                            .on_hover_text(format!("Amount of {} owned for this instrument. Click to open the derivatives overview.", tab.to_lowername()))
                                            .interact(Sense::click());

                                        if clickable.clicked() {
                                            state.modal = None;
                                            state.tab = Tab::Overview;
                                            state.overview.tab = OverviewTab::Derivatives;
                                        }

                                        ui.label(format!(
                                            "Abs. value: {}{CURRENCY}",
                                            derivatives.iter().map(|d| d.price * d.amount as f32 * if d.action == DerivativeAction::Bought { 1. } else { -1. }).sum::<f32>().signed(),
                                        ))
                                            .on_hover_text(
                                                format!("Sum of the {} prices of all {} {} (bought - sold).",
                                                        if tab == OrderKind::Futures {
                                                            "contract"
                                                        } else {
                                                            "strike"
                                                        },
                                                        instrument.lowername(),
                                                        tab.to_lowername()
                                                ));
                                    }
                                });
                            }
                        });
                        
                        ui.spacing_mut().slider_width = window.width() * 0.17;

                        ui.horizontal(|ui| {
                            ui.label("Quantity:");
                            
                            if tab == OrderKind::ShortSell {
                                ui.add(
                                    Slider::new(
                                        &mut state.modal_info.amount,
                                        0..=(max_loan / instrument.current()) as u32,
                                    )
                                        .show_value(false)
                                        .text(format!("{amount} {}", instrument.unit())),
                                ).on_hover_text(format!(
                                    "Amount of {} borrowed to go short. The maximum amount you can \
                                    go short depends on the AUM and the credit score.",
                                    instrument.lowername()
                                ))
                            } else if tab.is_derivative() {
                                let max = (strike_price / max_derivative_sell).max(player.cash.current() / strike_price) as u32;
                                
                                ui.add(
                                    Slider::new(&mut state.modal_info.amount,0..=max)
                                        .show_value(false)
                                        .text(format!("{amount} {}", instrument.unit())),
                                ).on_hover_text(format!(
                                    "Amount of {} traded in the {} contract. The maximum amount \
                                    you can sell depends on the AUM and the credit score.",
                                    instrument.lowername(), tab.to_lowername())
                                )
                            } else {
                                let max = if state.modal_info.loan {
                                    (max_loan / instrument.current()) as u32
                                } else {
                                    ((player.cash.current() / instrument.current()) as i32).max(owned.abs()) as u32
                                };
                                
                                ui.add(
                                    Slider::new(&mut state.modal_info.amount,0..=max)
                                        .show_value(false)
                                        .text(format!("{amount} {}", instrument.unit())),
                                )
                            };
                        });

                        if tab == OrderKind::LimitOrder {
                            ui.horizontal(|ui| {
                                ui.label("Limit price:");

                                ui.add(
                                    Slider::new(
                                        &mut state.modal_info.limit_stop,
                                        0.0..=instrument.current() * 2.,
                                    )
                                        .step_by(instrument.current() as f64 / 50.)
                                        .show_value(false)
                                        .text(format!("{}{CURRENCY}", limit_price.clean())),
                                )
                                .on_hover_text("If the price crosses this limit, the order is executed.");
                            });

                            ui.horizontal(|ui| {
                                ui.label("Bound:").on_hover_text(
                                    "Whether the limit stop is applied to the upper or lower bound.",
                                );
                                
                                let lower_bound = limit_price <= instrument.current();
                                ui.label(if lower_bound {
                                    "▼ Lower"
                                } else {
                                    "▲ Upper"
                                }).on_hover_text(if lower_bound {
                                    "Lower bound, i.e., the order is executed when te price is lower than the limit price."
                                } else {
                                    "Upper bound, i.e., the order is executed when the price is higher than the limit price."
                                });
                            });

                            total_price = limit_price * amount as f32
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

                            total_price = if state.modal_info.lower_bound {
                                (100 + trailing_stop) as f32 / 100. * amount as f32 * instrument.current()
                            } else {
                                (100 - trailing_stop) as f32 / 100. * amount as f32 * instrument.current()
                            }
                        } else if tab.is_derivative() {
                            if tab == OrderKind::Options {
                                ui.horizontal(|ui| {
                                    ui.label("Strike price:");

                                    let percentage = state.modal_info.strike_percentage as f32;
                                    ui.add(
                                        Slider::new(
                                            &mut state.modal_info.strike_percentage,
                                            -25..=25,
                                        )
                                            .step_by(5.)
                                            .show_value(false)
                                            .text(format!("{}%", percentage.signed())),
                                    )
                                    .on_hover_text(
                                        "Strike price for the option as percentage of the market price.");
                                });
                            }
                            
                            ui.horizontal(|ui| {
                                for label in DerivativeTerm::iter() {
                                    ui.selectable_value(
                                        &mut state.modal_info.derivative_term,
                                        label.clone(),
                                        RichText::new(label.to_name()).small(),
                                    ).on_hover_text("Time before the maturity of the derivative.");
                                }
                            });
                        }

                        if matches!(kind, InstrumentKind::Commodity(_)) && !tab.is_short_derivative() {
                            ui.label(format!("Storage costs: {storage_costs:.0}{CURRENCY}/month"))
                                .on_hover_text(
                                    "Storage costs for the selected amount. This amount is deducted \
                                    from the proceeds of a sale to pay for the open costs of the current \
                                    month.",
                                );
                        }

                        if tab == OrderKind::Futures {
                            ui.label(format!("Total price: {}{CURRENCY}", (strike_price * amount as f32).clean()));
                        } else if tab == OrderKind::Options {
                            ui.horizontal(|ui| {
                                ui.label(format!("Premium call: {}{CURRENCY}", (call_price * amount as f32).clean()))
                                    .on_hover_text("Price of the call option. This is the total cost for the buyer and the profit for the seller.");
                                ui.add_space(window.width() * 0.03);
                                ui.label(format!("Premium put: {}{CURRENCY}", (put_price * amount as f32).clean()))
                                    .on_hover_text("Price of the put option. This is the total cost for the buyer and the profit for the seller.");
                            });
                        } else if tab != OrderKind::ShortSell {
                            ui.label(format!("Total price: {}{CURRENCY}", total_price.clean()));
                        }

                        // Add spacing for equal height of modal
                        match tab {
                            OrderKind::MarketOrder => {
                                ui.label("");
                                ui.label("");
                            },
                            OrderKind::ShortSell => {
                                ui.label("");
                                ui.label("");
                                ui.label("");
                            }
                            _ => {}
                        }
                    });

                    if !matches!(instrument.kind(), InstrumentKind::Bond(_)) && !tab.is_derivative() {
                        ui.add_space(window.height() * 0.01);
                        ui.add(Separator::default().vertical());

                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("Margin loan: ").on_hover_text(
                                    "A margin loan is a type of loan that allows leverage on the \
                                    position. The investor borrows money from the broker to buy financial \
                                    instruments using their existing investments as collateral."
                                );

                                if tab != OrderKind::ShortSell {
                                    if owned >= 0 {
                                        state.modal_info.loan = state.modal_info.memory_loan;
                                    } else {
                                        state.modal_info.loan = false;
                                    }
                                } else {
                                    state.modal_info.loan = true;
                                }

                                let toggle = ui.add_enabled(
                                    tab != OrderKind::ShortSell && owned >= 0,
                                    toggle(&mut state.modal_info.loan),
                                );

                                if tab == OrderKind::ShortSell {
                                    toggle.on_disabled_hover_text("Short selling always requires a margin loan.");
                                } else {
                                    state.modal_info.memory_loan = state.modal_info.loan;

                                    if owned < 0 {
                                        toggle.on_disabled_hover_text(
                                            "Can't take a margin loan on a long position with a short position open.");
                                    }
                                }
                            });

                            if state.modal_info.loan {
                                ui.label(format!("Max. loan: {}{CURRENCY} ({} {})", max_loan.clean(), (max_loan / instrument.current()).floor(), instrument.lowername()))
                                    .on_hover_text(
                                        "Maximum amount that can be borrowed. This number depends \
                                        on the AUM and the credit score. Any other open margin loans \
                                        debts are subtracted from this amount.",
                                    );

                                ui.label(format!("Debt: {}{CURRENCY}", loan.debt.clean()))
                                    .on_hover_text("The size of the selected loan.");

                                ui.label(format!("Collateral: {}{CURRENCY}", loan.collateral.clean()))
                                    .on_hover_text(
                                        "Amount to be set aside as collateral for the borrowed shares. \
                                        If the short position is closed with losses, the remaining debt is \
                                        paid from this deposit.",
                                    );

                                //  Collateral at 50% + margin at 30%
                                let margin = loan.margin(if tab != OrderKind::ShortSell { amount as i32 } else { -(amount as i32) }).clean();
                                ui.label(format!("Margin: {margin}{CURRENCY} ({:.0}%)", loan.margin_frac * 100.))
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
                    }
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
                                    && player.cash.current() >= loan.collateral
                                    && (tab != OrderKind::MarketOrder || player.cash.current() >= total_price)
                                    && (tab != OrderKind::LimitOrder || limit_price < instrument.current())
                                    && (tab != OrderKind::Options || player.cash.current() >= call_price * amount as f32),
                                |ui| {
                                    let button = ui
                                        .add_modal_button(
                                            match tab {
                                                OrderKind::MarketOrder => "Buy",
                                                OrderKind::Futures => "Buy future",
                                                OrderKind::Options => "Buy call option",
                                                _ => "Place buy order"
                                            },
                                            &window
                                        );

                                    if button.clicked() {
                                        buy_clicked = true;
                                    }
                                },
                            );

                            if tab == OrderKind::Options {
                                ui.add_enabled_ui(
                                    amount > 0 && player.cash.current() >= put_price * amount as f32,
                                    |ui| {
                                        if ui.add_modal_button("Buy put option", &window).clicked() {
                                            buy_put_clicked = true;
                                        }
                                    },
                                );
                            }
                        }
                    },
                    |ui| {
                        if tab != OrderKind::ShortSell {
                            ui.add_enabled_ui(
                                total_price > 0.
                                    && (tab.is_derivative() || (owned > 0 && !state.modal_info.loan))
                                    && (tab != OrderKind::LimitOrder || limit_price > instrument.current()),
                                |ui| {

                                if !tab.is_derivative() {
                                    let mut button = ui
                                        .add_modal_button(
                                            if tab == OrderKind::MarketOrder {
                                                "Close position"
                                            } else {
                                                "Place close order"
                                            }, &window
                                        )
                                        .on_hover_text(format!(
                                            "Sell all owned {}.",
                                            instrument.lowername()
                                        ));

                                    if state.modal_info.loan  {
                                        button = button.on_disabled_hover_text("Can't sell with a margin loan.");
                                    } else if owned < amount as i32 {
                                        button = button.on_disabled_hover_text(format!(
                                            "No {} to sell.",
                                            instrument.lowername()
                                        ));
                                    }

                                    if button.clicked() {
                                        close_clicked = true;
                                    }
                                }

                                ui.add_enabled_ui(
                                    amount > 0
                                        && (tab != OrderKind::MarketOrder || owned >= amount as i32)
                                        && (tab != OrderKind::Futures || (max_derivative_sell / strike_price) as u32 >= amount),
                                    |ui| {
                                        let mut button = ui
                                            .add_modal_button(
                                                match tab {
                                                    OrderKind::MarketOrder => "Sell",
                                                    OrderKind::Futures => "Sell future",
                                                    OrderKind::Options => "Sell put option",
                                                    _ => "Place sell order"
                                                }, &window
                                            );

                                        if tab == OrderKind::Futures {
                                            button = button.on_hover_text(
                                                "Note that you are selling a future contract,\
                                                 taking the obligation to sell the underlying \
                                                 instrument at the maturity date. You are effectively \
                                                 going short on the instrument. It's not possible \
                                                 to sell an owned long future contract."
                                            );
                                        }
                                        
                                        if state.modal_info.loan  {
                                            button = button.on_disabled_hover_text("Can't sell with a margin loan.");
                                        } else if tab == OrderKind::Futures {
                                            button = button.on_disabled_hover_text(format!(
                                                "Not enough credit to sell {amount} future contracts.",
                                            ));
                                        } else if owned < amount as i32 {
                                            button = button.on_disabled_hover_text(format!(
                                                "Not enough {} to sell.",
                                                instrument.lowername()
                                            ));
                                        }

                                        if button.clicked() {
                                            sell_clicked = true;
                                        }
                                    },
                                );

                                if tab == OrderKind::Options {
                                    ui.add_enabled_ui(
                                        amount > 0,
                                        |ui| {
                                            if ui.add_modal_button("Sell call option", &window).clicked() {
                                                sell_call_clicked = true;
                                            }
                                        },
                                    );
                                }
                            });
                        }
                    },
                );
            });
        });
    });

    let command = if buy_clicked || buy_put_clicked {
        Some(Command::Buy)
    } else if sell_clicked || sell_call_clicked {
        Some(Command::Sell)
    } else if close_clicked {
        Some(Command::Close)
    } else {
        None
    };

    if let Some(command) = command {
        if tab.is_derivative() {
            let option_kind = match command {
                Command::Buy if buy_put_clicked => OptionKind::Put,
                Command::Buy => OptionKind::Call,
                Command::Sell if sell_call_clicked => OptionKind::Call,
                _ => OptionKind::Put,
            };

            // Pay/receive the option premium
            if tab == OrderKind::Options {
                let premium = if option_kind == OptionKind::Call {
                    call_price * amount as f32
                } else {
                    put_price * amount as f32
                };

                if command == Command::Buy {
                    player.cash.amount -= premium;
                } else {
                    player.cash.amount += premium;
                }
            }

            let derivative = Derivative {
                instrument: kind,
                kind: if tab == OrderKind::Futures {
                    DerivativeKind::Future
                } else {
                    DerivativeKind::Option
                },
                option_kind,
                action: if command == Command::Buy {
                    DerivativeAction::Bought
                } else {
                    DerivativeAction::Sold
                },
                term: state.modal_info.derivative_term.clone(),
                amount,
                price: strike_price,
                transaction_price: 0., // Real value is filled at maturity
                start_date: economy.date,
                execute: true,
                force_execute: false,
                status: OrderStatus::Pending,
            };

            message.write(MessageEv {
                message: format!(
                    "{} {} {} {}.",
                    derivative.action.to_name(),
                    amount,
                    instrument.lowername(),
                    tab.to_lowername(),
                ),
                level: MessageLevel::Info,
            });

            player.derivatives.push(derivative);
        } else {
            let mut order = Order {
                id: create_guid(),
                created: economy.date,
                instrument: kind,
                command,
                kind: tab,
                amount: if tab != OrderKind::ShortSell {
                    amount as i32
                } else {
                    -(amount as i32)
                },
                price: total_price
                    - if command == Command::Buy {
                        0.
                    } else {
                        storage_costs
                    },
                threshold: if tab == OrderKind::LimitOrder {
                    limit_price
                } else {
                    trailing_stop as f32
                },
                loan: state.modal_info.loan.then_some(loan),
                bound: instrument.current(),
                lower_bound: if tab == OrderKind::LimitOrder {
                    limit_price < instrument.current()
                } else {
                    state.modal_info.lower_bound
                },
                processed: NaiveDate::default(),
                status: OrderStatus::Executed,
            };

            if matches!(tab, OrderKind::MarketOrder | OrderKind::ShortSell) {
                order.processed = economy.date;
                order_ev.write(OrderEv {
                    id: order.id.clone(),
                    price: total_price,
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
    }

    if modal.should_close() {
        state.modal = None;
    }
}
