use serde::{Deserialize, Serialize};

use crate::core::factors::Factor;

#[derive(Clone, Serialize, Deserialize)]
pub struct CreditScore {
    score: u8,
}

impl CreditScore {
    const MIN: u8 = 0;
    pub const MAX: u8 = 100;

    pub fn increase(&mut self, has_tech: bool) {
        if self.score < Self::MAX {
            self.score += if has_tech {
                2
            } else {
                1
            };
        }
    }

    pub fn decrease(&mut self, has_tech: bool) {
        self.score = self
            .score
            .saturating_sub(if has_tech {
                6
            } else {
                12
            })
            .max(Self::MIN);
    }

    pub fn relative(&self) -> f32 {
        self.score as f32 / Self::MAX as f32
    }
}

impl Default for CreditScore {
    fn default() -> Self {
        CreditScore {
            score: (Self::MIN + Self::MAX) / 2,
        }
    }
}

impl Factor for CreditScore {
    fn image(&self) -> &str {
        "credit-score"
    }

    fn description(&self) -> String {
        "The credit score (0-100) is a measure of the company's creditworthiness, which is used \
        by banks and brokers to determine the maximum principal and interest rate for loans. A \
        higher score means better loan conditions.\n\n\
        If the company has active loans and pays the installments, the credit score increases \
        gradually. On the other hand, if the company has negative cash deposits, the credit score \
        drops significantly."
            .to_string()
    }

    fn current(&self) -> f32 {
        self.score as f32
    }
}
