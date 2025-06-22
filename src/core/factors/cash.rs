use serde::{Deserialize, Serialize};

use crate::core::factors::Factor;
use crate::utils::EnhFloat;

#[derive(Clone, Serialize, Deserialize)]
pub struct Cash {
    pub amount: f32,
    pub current_interest: f32,
    pub accumulated_interest: f32,
}

impl Cash {
    /// Fraction of the global interest rate that the bank pays on positive cash deposits
    const INTEREST_FRACTION_POS: f32 = 0.5;

    /// Fraction of the global interest rate that the bank received on negative cash deposits
    const INTEREST_FRACTION_NEG: f32 = 2.0;

    pub fn bump(&mut self, global_interest_rate: f32) {
        if self.amount < 0. {
            self.current_interest = global_interest_rate * Self::INTEREST_FRACTION_NEG;
        } else {
            self.current_interest = global_interest_rate * Self::INTEREST_FRACTION_POS;
        }

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
            a low interest on positive cash deposits and charges a high interest for negative \
            cash deposits.\n\n\
            Current interest: {:.1}%\n\
            Accumulated interest: {}",
            self.current_interest,
            self.accumulated_interest.clean()
        )
    }

    fn current(&self) -> f32 {
        self.amount
    }
}
