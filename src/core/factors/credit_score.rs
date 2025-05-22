use crate::core::factors::Factor;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct CreditScore(f32);

impl CreditScore {
    const MIN: f32 = 0.;
    const MAX: f32 = 100.;
}

impl Default for CreditScore {
    fn default() -> Self {
        CreditScore((Self::MIN + Self::MAX) * 0.5)
    }
}

impl Display for CreditScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.0}", self.current().floor())
    }
}

impl Factor for CreditScore {
    fn image(&self) -> &str {
        "credit-score"
    }

    fn description(&self) -> String {
        "Credit score\n\n\
        The credit score (0-100) is a measure of the player's creditworthiness, which is used \
        by banks and brokers to determine the maximum principal and interest rate for loans. A \
        higher score means better loan conditions.\n\n\
        If the player has active loans and pays the installments, the credit score increases \
        gradually. On the other hand, if the player defaults on a loan, the credit score drops \
        significantly."
            .to_string()
    }

    fn current(&self) -> f32 {
        self.0
    }
}
