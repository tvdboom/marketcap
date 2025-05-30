use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::core::factors::Factor;

#[derive(Clone, Serialize, Deserialize)]
pub struct CreditScore {
    pub score: u8,
}

impl CreditScore {
    pub const MIN: u8 = 0;
    pub const MAX: u8 = 100;
}

impl Default for CreditScore {
    fn default() -> Self {
        CreditScore {
            score: (Self::MIN + Self::MAX) / 2,
        }
    }
}

impl Display for CreditScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.current())
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
        gradually. On the other hand, if the company defaults on a loan, the credit score drops \
        significantly."
            .to_string()
    }

    fn current(&self) -> f32 {
        self.score as f32
    }
}
