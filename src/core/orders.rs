use bevy::prelude::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::constants::NA;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::instrument::InstrumentKind;
use crate::core::loans::MarginLoan;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::player::{OwnedInstrument, Player};
use crate::utils::NameFromEnum;

#[derive(EnumIter, Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum OrderKind {
    #[default]
    MarketOrder,
    LimitOrder,
    TrailingOrder,
    ShortSell,
    Futures,
    Options,
}

impl OrderKind {
    pub fn emoji(&self) -> &str {
        match self {
            OrderKind::MarketOrder => "🏪",
            OrderKind::LimitOrder => "♾",
            OrderKind::TrailingOrder => "🚶‍",
            OrderKind::ShortSell => "📉",
            OrderKind::Futures => "⏱",
            OrderKind::Options => "📝",
        }
    }

    pub fn abbr(&self) -> String {
        self.to_name().split_whitespace().next().unwrap().to_owned()
    }

    pub fn is_derivative(&self) -> bool {
        matches!(self, OrderKind::Futures | OrderKind::Options)
    }

    pub fn is_short_derivative(&self) -> bool {
        matches!(
            self,
            OrderKind::ShortSell | OrderKind::Futures | OrderKind::Options
        )
    }

    pub fn description(&self) -> &str {
        match self {
            OrderKind::MarketOrder => "Buy or sell an instrument at the current market price.",
            OrderKind::LimitOrder => {
                "Set a specific price to buy or sell an instrument. The order automatically \
                executes when the instrument reaches that price. If there isn't enough cash \
                to buy or instruments to sell at the time of execution, the order is cancelled."
            },
            OrderKind::TrailingOrder => {
                "A trailing order is a stop order that automatically follows (or trails) the \
                market once the price of an instrument has begun moving in a favourable direction. \
                If the market later reverses direction, the stop price remains fixed, and the \
                order is executed when the limit price is reached. If there isn't enough cash \
                to buy or instruments to sell at the time of execution, the order is cancelled."
            },
            OrderKind::ShortSell => {
                "Short selling is a trading strategy where an investor bets against an instrument, \
                expecting its price to decline. First, the investor borrows shares from a broker \
                and immediately sells them at the current market price. If the stock price drops, \
                the investor can buy the shares back at a lower price and return them to the \
                broker, pocketing the difference as profit. The investor pays interest during \
                the time the shares are borrowed. If the stock price rises, the investor must \
                buy back the shares at a higher price, resulting in a loss."
            },
            OrderKind::Futures => {
                "Financial contracts to buy or sell instruments against a predetermined price \
                in the future. Buying a future (going long) automatically buys the underlying \
                instrument for the strike price at the maturity date. Selling a future (going \
                short) provides immediate cash and the obligation to deliver the underlying \
                instrument at the strike price at the maturity date. If the underlying instrument \
                is not owned, it's automatically bought at the current market price and it greatly \
                reduces the credit score.\n\n\
                Note that buying and selling the same amount of futures don't 'cancel' each other \
                out due to the different maturity dates. Futures in this game are hold-to-maturity \
                only, meaning that they can't be traded before the maturity date."
            },
            OrderKind::Options => {
                "Financial contracts that grant the right, but not the obligation, to buy or sell \
                instruments at a predetermined price at the maturity date. Buying a call option \
                gives the right to buy the underlying instrument at the strike price, while buying \
                a put option gives the right to sell it. Selling (writing) an option provides \
                immediate cash but creates a contingent obligation if the option is exercised by \
                the buyer.\n\n\
                Unlike futures, options can expire worthless if market prices don't reach favorable \
                levels, making them less risky for buyers but riskier for sellers. Exercising an \
                option automatically executes the corresponding buy or sell transaction at the \
                strike price. If the necessary instruments aren't available, they are acquired at \
                the current market price and the credit score is greatly reduced.\n\n\
                In this game, all options are European-style and hold-to-maturity only — they can \
                only be exercised at the maturity date and cannot be traded once purchased."
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Command {
    Buy,
    Sell,
    Close,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Executed,
    Failed(String),
    Canceled,
}

impl OrderStatus {
    pub fn reason(&self) -> String {
        match self {
            OrderStatus::Failed(reason) => reason.clone(),
            _ => NA.to_owned(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Order {
    /// Unique identifier for the order
    pub id: String,

    /// Date at which the order was created
    pub created: NaiveDate,

    /// Instrument that his being traded
    pub instrument: InstrumentKind,

    /// Trade command (buy, sell, close)
    pub command: Command,

    /// Kind of order
    pub kind: OrderKind,

    /// Amount of the instrument to trade
    pub amount: i32,

    /// Price at which the order is executed
    pub price: f32,

    /// The limit price or trailing percentage on which the trade has been executed
    pub threshold: f32,

    /// Upper or lower bound for trailing order
    pub bound: f32,

    /// Whether the threshold is an upper or lower limit
    pub lower_bound: bool,

    /// Margin loan taken for the order
    pub loan: Option<MarginLoan>,

    /// Date of the order execution
    pub processed: NaiveDate,

    /// Status of the order
    pub status: OrderStatus,
}

#[derive(Event)]
pub struct OrderEv {
    pub id: String,
    pub price: f32,
}

pub fn execute_orders(
    mut economy: ResMut<GlobalEconomy>,
    mut player: ResMut<Player>,
    mut order_ev: EventReader<OrderEv>,
    mut message: EventWriter<MessageEv>,
) {
    for OrderEv { id, price } in order_ev.read() {
        economy.economy.current_traded_volume += price.abs();

        let order = Order {
            price: *price,
            status: OrderStatus::Executed,
            ..player.orders.iter().find(|o| o.id == *id).unwrap().clone()
        };

        let instrument = economy.get(&order.instrument);

        let mut cash = player.cash.amount;
        match order.command {
            Command::Buy => {
                message.write(MessageEv {
                    message: format!(
                        "{} {} {}.",
                        if order.kind != OrderKind::ShortSell {
                            "Bought"
                        } else {
                            "Shorted"
                        },
                        order.amount.abs(),
                        instrument.lowername()
                    ),
                    level: MessageLevel::Info,
                });

                if let Some(owned) = player.get_mut(&order.instrument) {
                    let is_short = owned.amount < 0;
                    owned.amount += order.amount;

                    if let Some(order_loan) = &order.loan {
                        cash -= order_loan.collateral;

                        if let Some(existing_loan) = &mut owned.loan {
                            // If the order has a loan, add the stats
                            existing_loan.debt += order_loan.debt;
                            existing_loan.collateral += order_loan.collateral;
                            existing_loan.interest_rate = order_loan.interest_rate;
                            existing_loan.margin_frac = order_loan.margin_frac;
                        } else {
                            owned.loan = Some(order_loan.clone());
                        }
                    } else {
                        if let Some(existing_loan) = &mut owned.loan {
                            // If the order has no loan, but the instrument already has a loan...
                            if is_short {
                                // In a short position, buy back from the proceeds until it's closed
                                if owned.amount >= 0 {
                                    // The short position was closed
                                    cash += existing_loan.collateral + existing_loan.debt - price;

                                    message.write(MessageEv {
                                        message: format!(
                                            "Closed short position for {}. Collateral returned.",
                                            instrument.lowername()
                                        ),
                                        level: MessageLevel::Info,
                                    });

                                    owned.loan = None; // Remove the loan as it's closed
                                } else {
                                    // The short position remains open
                                    existing_loan.debt -= price;
                                }
                            } else {
                                // In a long position, nothing changes with the existing loan
                                cash -= price;
                            }
                        } else {
                            // No existing loan, pay from the cash
                            cash -= price;
                        }
                    }
                } else {
                    if let Some(loan) = &order.loan {
                        cash -= loan.collateral;
                    } else {
                        cash -= price;
                    }

                    player.instruments.push(OwnedInstrument {
                        kind: order.instrument.clone(),
                        amount: order.amount,
                        loan: order.loan,
                        warning: false,
                    });
                }
            },
            Command::Sell => {
                if let Some(owned) = player.get_mut(&order.instrument) {
                    owned.amount -= order.amount;

                    if let Some(loan) = &mut owned.loan {
                        // If the instrument has a loan, pay back the debt first
                        if *price >= loan.debt {
                            let remainder = *price - loan.debt;

                            cash += remainder + loan.collateral;
                            owned.loan = None;

                            message.write(MessageEv {
                                message: format!(
                                    "Repaid margin loan for {}. Collateral returned.",
                                    instrument.lowername()
                                ),
                                level: MessageLevel::Info,
                            });
                        } else {
                            loan.debt -= price;
                        }
                    } else {
                        // If no loan, just add the cash from the sale
                        cash += price;
                    }
                }

                message.write(MessageEv {
                    message: format!("Sold {} {}.", order.amount, instrument.lowername()),
                    level: MessageLevel::Info,
                });
            },
            Command::Close => {
                player.instruments.retain_mut(|o| {
                    if o.kind == order.instrument {
                        if let Some(loan) = &mut o.loan {
                            cash += *price - loan.debt + loan.collateral;
                        } else {
                            cash += price;
                        }

                        false
                    } else {
                        true
                    }
                });

                message.write(MessageEv {
                    message: format!("Closed {} position.", instrument.lowername()),
                    level: MessageLevel::Info,
                });
            },
        }

        player.cash.amount = cash;
        player.instruments.retain_mut(|o| o.amount != 0);
    }
}
