use std::fmt::{Display, Formatter, Result};
use crate::core::attributes::attribute::Attribute;
use rand::{Rng, rng};
use crate::core::attributes::cash::Cash;

#[derive(Clone)]
pub struct GlobalEconomicFactor(Vec<f32>);

impl Default for GlobalEconomicFactor {
    fn default() -> Self {
        Self(Vec::from([(Self::MIN + Self::MAX) * 0.5]))
    }
}

impl Display for GlobalEconomicFactor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:.0}", self.current().floor())
    }
}

impl GlobalEconomicFactor {
    pub const MIN: f32 = 0.;
    pub const MAX: f32 = 100.;

    pub fn bump(&mut self) {
        self.0
            .push((self.current() + rng().random_range(-2.5..2.5)).clamp(Self::MIN, Self::MAX))
    }
}

impl Attribute for GlobalEconomicFactor {
    fn image(&self) -> &str {
        "economic"
    }

    fn description(&self) -> String {
        "Global economic factor\n\n\
        The global economy represents the overall financial health and activity of the \
        world. It fluctuates based on trade, market sentiment and events. A strong global \
        economy means higher consumer confidence, robust industry growth, and increased \
        investments. A weak global economy signals recessions, crises, or reduced spending, \
        making businesses expansion difficult.\n\n\
        In the game, the global economic factor (0-100) serves as a macro-scale indicator, \
        affecting stock markets and interest rates dynamically."
            .to_string()
    }

    fn current(&self) -> f32 {
        *self.0.last().unwrap()
    }
}
