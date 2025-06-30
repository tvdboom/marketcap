use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use crate::core::instruments::instrument::InstrumentKind;

#[derive(Clone, Default, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
pub enum DerivativeKind {
    Future,
    Option,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Derivative {
    pub instrument: InstrumentKind,
    pub kind: DerivativeKind,
    pub term: DerivativeTerm,
    pub price: f32,
    pub start_date: NaiveDate,
}

impl Derivative {
    pub fn maturity_date(&self) -> NaiveDate {
        self.start_date
            .checked_add_signed(Duration::days(self.term.days() as i64))
            .unwrap()
    }
}
