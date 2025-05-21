use crate::core::factors::Factor;
use crate::core::factors::economy::Economy;
use crate::core::factors::interest::Interest;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Inflation(Vec<f32>);

impl Default for Inflation {
    fn default() -> Self {
        Self(Vec::from([Self::DEFAULT]))
    }
}

impl Display for Inflation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:.1}%", self.current())
    }
}

impl Inflation {
    pub const MIN: f32 = 1.;
    pub const MAX: f32 = 10.;
    pub const DEFAULT: f32 = 3.;
    pub const ADJUSTMENT_ECONOMY_RATE: f32 = 0.25;
    pub const ADJUSTMENT_INTEREST_RATE: f32 = 0.05;

    pub fn bump(&mut self, economy: f32, interest: f32) -> f32 {
        let value = (self.current()
            + Self::ADJUSTMENT_ECONOMY_RATE
                * (Self::MIN + Self::MAX * (economy / Economy::MAX).powf(2.2) - self.current())
            + Self::ADJUSTMENT_INTEREST_RATE * Inflation::DEFAULT * (Interest::DEFAULT - interest))
            .clamp(Self::MIN, Self::MAX);

        self.0.push(value);
        value
    }
}

impl Factor for Inflation {
    fn image(&self) -> &str {
        "inflation"
    }

    fn description(&self) -> String {
        "Inflation\n\n\
        Inflation is the gradual increase in the price of goods and services over time, \
        reducing the purchasing power of money. As inflation rises, business expenses \
        become more expensive.\n\n\
        Inflation is tied to the global economic factor, where a thriving economy has a \
        higher chance of seeing inflation rise. Inflation is also affected by increases in \
        money supply (taking loans) and government policies."
            .to_string()
    }

    fn current(&self) -> f32 {
        *self.0.last().unwrap()
    }
}
