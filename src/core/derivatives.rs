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
    Month,
    Quarter,
    Year,
}

impl DerivativeTerm {
    pub fn days(&self) -> u32 {
        match self {
            DerivativeTerm::Month => 30,
            DerivativeTerm::Quarter => 90,
            DerivativeTerm::Year => 365,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum DerivativeKind {
    Future,
    Option,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum DerivativeAction {
    Bought,
    Sold,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Derivative {
    pub instrument: InstrumentKind,
    pub kind: DerivativeKind,
    pub action: DerivativeAction,
    pub term: DerivativeTerm,
    pub amount: u32,
    pub price: f32,
    pub start_date: NaiveDate,
    pub execute: bool,
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
