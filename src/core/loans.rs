use bevy::prelude::Resource;
use chrono::NaiveDate;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq)]
pub enum LoanProvider {
    #[default]
    Bank,
    AlternativeLender,
}

impl LoanProvider {
    pub fn emoji(&self) -> &str {
        match self {
            LoanProvider::Bank => "🏦",
            LoanProvider::AlternativeLender => "💸",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            LoanProvider::Bank => {
                "\
                Banks are the standard credit providers for companies. The maximum amount \
                that can be borrowed depends on the player's enterprise value and credit score. \
                The interest rate is based on the global interest rate, the credit score and \
                the payback period, where a longer payback period reduces the loan's interest.\n\n\
                If a player defaults on the loan (fails to pay it back), the debt is accumulated \
                for next month. If the player defaults three consecutive months, its assets will \
                be forcibly sold (usually for unfavorable terms) until there is enough cash to \
                pay back the complete loan."
            }
            LoanProvider::AlternativeLender => "\
                Alternative credit providers offer loans were traditional institutions like \
                banks may hesitate, usually against a higher interest rate. Contrary to banks,\
                the credit score is not used in the calculations for the maximum principal nor \
                the interest rate.\n\n\
                If a player defaults on the loan (fails to pay it back), the debt is accumulated \
                for next month. If the player defaults three consecutive months, its assets will \
                be forcibly sold (usually for unfavorable terms) until there is enough cash to \
                pay back the complete loan.",
        }
    }
}

#[derive(EnumIter, Clone, Debug, Default, PartialEq)]
pub enum LoanKind {
    #[default]
    StraightLine,
    Annuity,
}

impl LoanKind {
    pub fn description(&self) -> &str {
        match self {
            LoanKind::StraightLine => {
                "Principal repayment portion is the same each month. The interest \
                shrinks over time, thus the payments decrease each month."
            }
            LoanKind::Annuity => {
                "\
                Monthly payment is the same every month. Early payments are mostly interest, \
                later payments are mostly principal."
            }
        }
    }

    pub fn installment(
        &self,
        principal: u32,
        outstanding: u32,
        interest_rate: f32,
        n_terms: u32,
    ) -> f32 {
        match self {
            LoanKind::StraightLine => {
                principal as f32 / n_terms as f32 + (outstanding as f32 * interest_rate / 12.)
            }
            LoanKind::Annuity => {
                (principal as f32 * (interest_rate / 12.))
                    / (1. - (1. + interest_rate / 12.).powf(-(n_terms as f32)))
            }
        }
    }
}

#[derive(Clone)]
pub struct Loan {
    /// Institution that provided the loan
    pub provider: LoanProvider,

    /// The amount of money borrowed
    pub principal: f32,

    /// The amount left to pay back
    pub outstanding: f32,

    /// The interest over the loan
    pub interest_rate: f32,

    /// Type of loan
    pub kind: LoanKind,

    /// The date when the loan will be repaid fully
    pub term: NaiveDate,
    
    /// Start date
    pub start_date: NaiveDate,
}
