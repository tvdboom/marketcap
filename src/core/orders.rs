use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::player::InstrumentKind;
use crate::utils::NameFromEnum;

#[derive(EnumIter, Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum OrderKind {
    #[default]
    MarketOrder,
    LimitOrder,
    TrailingOrder,
    ShortSelling,
    Futures,
}

impl OrderKind {
    pub fn emoji(&self) -> &str {
        match self {
            OrderKind::MarketOrder => "🏪",
            OrderKind::LimitOrder => "♾",
            OrderKind::TrailingOrder => "🚶‍",
            OrderKind::ShortSelling => "📉",
            OrderKind::Futures => "🔮",
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
            OrderKind::ShortSelling => {
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
                in the future. "
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Order {
    Buy,
    Sell,
    Close,
}

impl Order {
    pub fn past(&self) -> &str {
        match self {
            Order::Buy => "bought",
            Order::Sell => "sold",
            Order::Close => "closed",
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderDirection {
    Upper,
    Lower,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderStatus {
    Executed,
    Failed,
    Canceled,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PendingOrder {
    pub id: String,
    pub instrument: InstrumentKind,
    pub order: Order,
    pub kind: OrderKind,
    pub direction: OrderDirection,
    pub amount: u32,
    pub threshold: u32,
}

impl PendingOrder {
    pub fn to_processed(
        &self,
        date: NaiveDate,
        status: OrderStatus,
        reason: &str,
    ) -> ProcessedOrder {
        ProcessedOrder {
            id: self.id.clone(),
            date,
            instrument: self.instrument.clone(),
            order: self.order.clone(),
            kind: self.kind.clone(),
            amount: self.amount,
            threshold: self.threshold,
            status,
            reason: reason.to_string(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProcessedOrder {
    pub id: String,
    pub date: NaiveDate,
    pub instrument: InstrumentKind,
    pub order: Order,
    pub kind: OrderKind,
    pub amount: u32,
    pub threshold: u32,
    pub status: OrderStatus,
    pub reason: String,
}
