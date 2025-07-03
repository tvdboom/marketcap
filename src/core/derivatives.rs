use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::instrument::InstrumentKind;
use crate::core::orders::OrderStatus;
use crate::core::player::Player;
use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum DerivativeTerm {
    #[default]
    OneMonth,
    ThreeMonths,
    SixMonths,
    OneYear,
}

impl DerivativeTerm {
    pub fn days(&self) -> u32 {
        match self {
            DerivativeTerm::OneMonth => 30,
            DerivativeTerm::ThreeMonths => 90,
            DerivativeTerm::SixMonths => 180,
            DerivativeTerm::OneYear => 365,
        }
    }

    pub fn years(&self) -> f32 {
        match self {
            DerivativeTerm::OneMonth => 0.0833,
            DerivativeTerm::ThreeMonths => 0.25,
            DerivativeTerm::SixMonths => 0.5,
            DerivativeTerm::OneYear => 1.0,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum DerivativeKind {
    Future,
    Option,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum OptionKind {
    Call,
    Put,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum DerivativeAction {
    Bought,
    Sold,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Derivative {
    /// Instrument being traded
    pub instrument: InstrumentKind,

    /// Type of derivative, either a future or an option
    pub kind: DerivativeKind,

    /// Call or put option (only for options)
    pub option_kind: OptionKind,

    /// Action taken on the derivative, either bought or sold
    pub action: DerivativeAction,

    /// Term of the derivative, e.g., 1 month, 3 months, etc.
    pub term: DerivativeTerm,

    /// Number of contracts, options, or futures
    pub amount: u32,

    /// Strike price for options, or contract price for futures
    pub price: f32,

    /// Market price at the moment the transaction was made
    pub transaction_price: f32,

    /// Date when the derivative was created
    pub start_date: NaiveDate,

    /// Whether to execute the derivative at maturity (only for options)
    pub execute: bool,

    /// Whether the derivative is pending, executed or canceled
    pub status: OrderStatus,
}

impl Derivative {
    pub fn max_sell(economy: &GlobalEconomy, player: &Player) -> f32 {
        player.aum(&economy) / 2. * (0.3 + 0.7 * player.credit_score.relative())
            - player
                .pending_derivatives()
                .iter()
                .filter_map(|d| {
                    (d.action == DerivativeAction::Sold).then_some(d.amount as f32 * d.price)
                })
                .sum::<f32>()
    }

    pub fn maturity_date(&self) -> NaiveDate {
        self.start_date
            .checked_add_signed(Duration::days(self.term.days() as i64))
            .unwrap()
    }
}
