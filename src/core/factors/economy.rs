use std::fmt::{Display, Formatter, Result};

use rand::{Rng, rng};
use serde::{Deserialize, Serialize};

use crate::core::factors::Factor;

#[derive(Clone, Serialize, Deserialize)]
pub struct Economy(pub Vec<f32>);

impl Default for Economy {
    fn default() -> Self {
        Self(Vec::from([Self::DEFAULT]))
    }
}

impl Display for Economy {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:.0}", self.current().floor())
    }
}

impl Economy {
    pub const MIN: f32 = 0.;
    pub const MAX: f32 = 100.;
    pub const DEFAULT: f32 = (Self::MIN + Self::MAX) * 0.5;

    /// Maximum random daily fluctuation from the current value
    const FLUCTUATION: f32 = 2.0;

    pub fn bump(&mut self) -> f32 {
        let norm = (self.current() - Self::DEFAULT) / Self::MAX;

        let u = rng().random::<f32>();

        // When the economy goes very well or badly, tend to normalize
        let bias = match self.current() {
            n if n < 20. => u.powf(1. / (1. - norm * 3.)),
            n if n > 80. => u.powf(1. + norm * 3.),
            _ => u,
        };

        let fluctuation = bias * 2. * Self::FLUCTUATION - Self::FLUCTUATION;
        let value = (self.current() + fluctuation).clamp(Self::MIN, Self::MAX);

        self.0.push(value);
        value
    }
}

impl Factor for Economy {
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
