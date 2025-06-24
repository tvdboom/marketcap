use serde::{Deserialize, Serialize};

use crate::core::constants::CURRENCY;
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
    const INTEREST_FRACTION_NEG: f32 = 9.5;

    pub fn bump(&mut self, global_interest_rate: f32) {
        self.current_interest = (global_interest_rate
            * if self.amount < 0. {
                Self::INTEREST_FRACTION_NEG
            } else {
                Self::INTEREST_FRACTION_POS
            })
        .round1();

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
            Cash: {}\n\
            Current interest: {:.1}%\n\
            Accumulated interest: {} {CURRENCY}",
            self.amount.signed(),
            if self.amount >= 0. {
                self.current_interest
            } else {
                -self.current_interest
            },
            self.accumulated_interest.signed()
        )
    }

    fn current(&self) -> f32 {
        self.amount
    }
}
