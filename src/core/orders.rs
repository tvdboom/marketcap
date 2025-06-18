use bevy::prelude::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::constants::NA;
use crate::core::global_economy::GlobalEconomy;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::player::{InstrumentKind, OwnedInstrument, Player};
use crate::utils::NameFromEnum;

#[derive(EnumIter, Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum OrderKind {
    #[default]
    MarketOrder,
    LimitOrder,
    TrailingOrder,
    ShortSell,
    Derivatives,
}

impl OrderKind {
    pub fn emoji(&self) -> &str {
        match self {
            OrderKind::MarketOrder => "🏪",
            OrderKind::LimitOrder => "♾",
            OrderKind::TrailingOrder => "🚶‍",
            OrderKind::ShortSell => "📉",
            OrderKind::Derivatives => "🔮",
        }
    }

    pub fn abbr(&self) -> String {
        self.to_name().split_whitespace().next().unwrap().to_owned()
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
            OrderKind::Derivatives => {
                "Financial contracts to buy or sell instruments against a predetermined price \
                in the future. "
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

    /// Interest rate for short selling
    pub interest: f32,

    /// Fraction of margin for short selling
    pub margin_frac: f32,

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
    economy: Res<GlobalEconomy>,
    mut player: ResMut<Player>,
    mut order_ev: EventReader<OrderEv>,
    mut message: EventWriter<MessageEv>,
) {
    for OrderEv { id, price } in order_ev.read() {
        let order = Order {
            price: *price,
            status: OrderStatus::Executed,
            ..player.orders.iter().find(|o| o.id == *id).unwrap().clone()
        };

        let instrument = economy.get(&order.instrument);

        match order.command {
            Command::Buy => {
                if let Some(owned) = player.get_mut(&order.instrument) {
                    owned.amount += order.amount;

                    if owned.collateral > 0. {
                        // Buy instrument in a short position from the collateral
                        owned.collateral -= price;
                        if owned.amount == 0 {
                            // When short position is closed, return collateral
                            player.cash.amount += owned.collateral;

                            message.write(MessageEv {
                                message: format!(
                                    "Closed short position for {}. Collateral returned.",
                                    instrument.lowername()
                                ),
                                level: MessageLevel::Info,
                            });
                        }
                    } else {
                        player.cash.amount -= price;
                    }
                } else {
                    player.instruments.push(OwnedInstrument {
                        kind: order.instrument.clone(),
                        amount: order.amount,
                        start_price: price / order.amount as f32,
                        interest: order.interest,
                        margin_frac: order.margin_frac,
                        collateral: 1.5 * price,
                        warning: false,
                    });

                    if order.kind == OrderKind::ShortSell {
                        player.cash.amount -= order.price * 0.5; // Collateral payment
                    } else {
                        player.cash.amount -= price;
                    }
                }

                message.write(MessageEv {
                    message: format!(
                        "{} {} {}.",
                        if order.kind != OrderKind::ShortSell {
                            "Bought"
                        } else {
                            "Opened short position for"
                        },
                        order.amount.abs(),
                        instrument.lowername()
                    ),
                    level: MessageLevel::Info,
                });
            },
            Command::Sell => {
                if let Some(owned) = player.get_mut(&order.instrument) {
                    owned.amount -= order.amount;
                }

                player.cash.amount += price;

                message.write(MessageEv {
                    message: format!("Sold {} {}.", order.amount, instrument.lowername()),
                    level: MessageLevel::Info,
                });
            },
            Command::Close => {
                player
                    .instruments
                    .retain_mut(|o| o.kind != order.instrument);

                player.cash.amount += price;

                message.write(MessageEv {
                    message: format!("Closed {} position.", instrument.lowername()),
                    level: MessageLevel::Info,
                });
            },
        }

        player.instruments.retain_mut(|o| o.amount != 0);
    }
}
