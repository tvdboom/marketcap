use std::fmt::{Display, Formatter, Result};
use crate::core::attributes::attribute::Attribute;
use crate::core::attributes::global_economic_factor::GlobalEconomicFactor;

#[derive(Clone)]
pub struct GlobalInterestRate(Vec<f32>);

impl Default for GlobalInterestRate {
    fn default() -> Self {
        Self(Vec::from([(Self::MIN + Self::MAX) * 0.5]))
    }
}

impl Display for GlobalInterestRate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:.1}%", self.current())
    }
}

impl GlobalInterestRate {
    const MIN: f32 = 2.;
    const MAX: f32 = 10.;
    const ADJUSTMENT_RATE: f32 = 0.1;

    pub fn bump(&mut self, global_economic_factor: f32) {
        self.0.push(
            self.current()
                + Self::ADJUSTMENT_RATE
                    * (Self::MAX
                        - ((Self::MAX - Self::MIN) * global_economic_factor
                            / GlobalEconomicFactor::MAX)
                        - self.current()),
        )
    }
}

impl Attribute for GlobalInterestRate {
    fn image(&self) -> &str {
        "interest"
    }

    fn description(&self) -> String {
        "Global interest rate\n\n\
        The global interest rate is set by the central bank and determines the cost of \
        borrowing money. It rises when the economy struggles, making loans expensive, and \
        falls when the economy thrives, encouraging investment.\n\n\
        In the game, interest rates directly impact debt strategies. Players must decide \
        whether to take loans during cheap borrowing periods or avoid debt when rates rise."
            .to_string()
    }

    fn current(&self) -> f32 {
        *self.0.last().unwrap()
    }
}
