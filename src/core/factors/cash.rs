use serde::{Deserialize, Serialize};

use crate::core::factors::Factor;

#[derive(Clone, Serialize, Deserialize)]
pub struct Cash {
    pub amount: f32,
    pub current_interest: f32,
    pub accumulated_interest: f32,
}

impl Cash {
    /// Fraction of the global interest rate that the bank pays on cash deposits
    const INTEREST_FRACTION: f32 = 0.5;

    pub fn bump(&mut self, global_interest_rate: f32) {
        self.current_interest = global_interest_rate * Self::INTEREST_FRACTION;
        self.accumulated_interest += self.amount * self.current_interest / 100. / 365.;
    }

    pub fn resolve(&mut self) {
        self.amount += self.accumulated_interest;
        self.accumulated_interest = 0.;
    }
}

impl Default for Cash {
    fn default() -> Self {
        Cash {
            amount: 10_000.,
            current_interest: 0.,
            accumulated_interest: 0.,
        }
    }
}

impl Factor for Cash {
    fn image(&self) -> &str {
        "cash"
    }

    fn description(&self) -> String {
        format!(
            "Cash represents the liquid assets the company possesses, funds that are immediately \
        available for spending, investing, or covering financial obligations. The bank pays \
        a low interest on cash deposits.\n\n\
        Current interest: {:.1}%\n\
        Accumulated interest: {}",
            self.current_interest,
            self.accumulated_interest.floor()
        )
    }

    fn current(&self) -> f32 {
        self.amount
    }
}
