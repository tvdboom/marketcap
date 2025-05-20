use crate::core::attributes::attribute::Attribute;

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

impl Attribute for CreditScore {
    fn image(&self) -> &str {
        "credit-score"
    }

    fn description(&self) -> String {
        "Credit score\n\n\
        The credit score is a measure of the player's creditworthiness, which is used by \
        banks and other credit providers to determine the interest rate for loans. A higher \
        score means lower interest rates and better loan conditions.\n\n\
        If the player has active loans and pays the installments, the credit score increases \
        gradually. On the contrary, if the player defaults on a loan, the credit score drops \
        significantly."
            .to_string()
    }

    fn current(&self) -> f32 {
        self.0
    }
}
