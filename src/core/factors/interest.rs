use crate::core::factors::Factor;
use crate::core::factors::inflation::Inflation;
use crate::utils::Round1;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Serialize, Deserialize)]
pub struct Interest {
    pub rate: Vec<f32>,
    pub next_rate: Option<f32>,
}

impl Default for Interest {
    fn default() -> Self {
        Self {
            rate: Vec::from([Self::DEFAULT]),
            next_rate: None,
        }
    }
}

impl Display for Interest {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:.1}%", self.current())
    }
}

impl Interest {
    pub const MIN: f32 = 1.;
    pub const MAX: f32 = 10.;
    pub const DEFAULT: f32 = 3.;
    pub const ADJUSTMENT_INFLATION_RATE: f32 = 0.9;

    pub fn resolve(&mut self, inflation: f32) {
        if let Some(rate) = self.next_rate {
            self.rate.push(rate);
            self.next_rate = None;
        } else {
            // Calculate the next interest rate
            let value = (self.current()
                + Self::ADJUSTMENT_INFLATION_RATE * -(Inflation::DEFAULT - inflation))
                .round1()
                .clamp(Self::MIN, Self::MAX);

            self.next_rate = Some(value);
        }
    }
}

impl Factor for Interest {
    fn image(&self) -> &str {
        "interest"
    }

    fn description(&self) -> String {
        format!(
            "Global interest rate\n\n\
            The global interest rate is set by the central bank and determines the cost of \
            borrowing money. It rises when inflation is high, making loans expensive, and \
            falls when inflation is low, encouraging investment.\n\n\
            In the game, interest rates directly impact debt strategies. Players must try to \
            take loans during cheap borrowing periods and avoid debt when rates rise. \
            The interest rate is updated bi-monthly. At the start of every month, the rate is \
            either updated or the next rate is calculated.\
            {}",
            if let Some(rate) = self.next_rate {
                format!("\n\nNext rate: {:.1}%", rate)
            } else {
                "".to_string()
            }
        )
    }

    fn current(&self) -> f32 {
        *self.rate.last().unwrap()
    }
}
