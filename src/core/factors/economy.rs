use rand::{Rng, rng};
use serde::{Deserialize, Serialize};

use crate::core::factors::Factor;
use crate::utils::DQueue;

#[derive(Clone, Serialize, Deserialize)]
pub struct Economy {
    /// The values of the economy over time
    pub values: DQueue<f32>,

    /// The total traded volume this month
    pub current_traded_volume: f32,

    /// The total traded volume of the last month
    /// A higher traded volume improves the economy and in reverse
    pub last_traded_volume: f32,
}

impl Default for Economy {
    fn default() -> Self {
        Self {
            values: DQueue::from([Self::DEFAULT]),
            current_traded_volume: 0.,
            last_traded_volume: f32::NAN,
        }
    }
}

impl Economy {
    pub const MIN: f32 = 0.;
    pub const MAX: f32 = 100.;
    pub const DEFAULT: f32 = (Self::MIN + Self::MAX) * 0.5;

    /// Maximum random daily fluctuation from the current value
    const FLUCTUATION: f32 = 1.5;

    pub fn bump(&mut self, aum: f32) -> f32 {
        let norm = (self.current() - Self::DEFAULT) / Self::MAX;

        let u = rng().random::<f32>();

        // When the economy goes very well or badly, tend to normalize
        let bias = match self.current() {
            n if n < 20. => u.powf(1. / (1. - norm * 3.)),
            n if n > 80. => u.powf(1. + norm * 3.),
            _ => u,
        };

        let fluctuation = bias * 2. * Self::FLUCTUATION - Self::FLUCTUATION;
        let mut value = (self.current() + fluctuation).clamp(Self::MIN, Self::MAX);

        // Small gain or loss depending on last month's traded volume
        // If the traded volume was 30% of the total AUM, the effect is 0%
        if !self.last_traded_volume.is_nan() {
            value *= 1. + ((self.last_traded_volume / aum) - 0.3) / 75.;
        }

        self.values.push(value);
        value
    }
}

impl Factor for Economy {
    fn image(&self) -> &str {
        "economic"
    }

    fn description(&self) -> String {
        "The global economy represents the overall financial health and activity of the \
        world. It fluctuates based on trade, market sentiment and events. A strong global \
        economy means higher consumer confidence, robust industry growth, and increased \
        investments. A weak global economy signals recessions, crises, or reduced spending, \
        making businesses expansion difficult.\n\n\
        In the game, the global economic factor (0-100) serves as a macro-scale indicator, \
        affecting stock markets and interest rates dynamically."
            .to_string()
    }

    fn current(&self) -> f32 {
        *self.values.back().unwrap()
    }
}
