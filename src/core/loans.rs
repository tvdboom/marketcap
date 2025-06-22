use chrono::{Months, NaiveDate};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::constants::LOAN_STEP;
use crate::core::factors::Factor;
use crate::core::factors::credit_score::CreditScore;
use crate::core::global_economy::GlobalEconomy;
use crate::core::player::Player;
use crate::utils::EnhFloat;

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum LoanProvider {
    #[default]
    Bank,
    AlternativeLender,
}

impl LoanProvider {
    pub fn description(&self) -> &str {
        match self {
            LoanProvider::Bank => {
                "\
                Banks are the standard credit providers for companies. They use a company's \
                credit score to calculate the maximum principal and the loan's interest rate. \
                For decent credit scores, they offer better terms than alternative lenders."
            },
            LoanProvider::AlternativeLender => {
                "\
                Alternative credit providers offer loans were traditional institutions like \
                banks may hesitate, usually against a higher interest rate. Contrary to banks,\
                the credit score is not used in the calculations for the maximum principal nor \
                the interest rate."
            },
        }
    }

    pub fn max_principal(&self, enterprise_value: f32, credit_score: f32) -> u32 {
        match self {
            LoanProvider::Bank => {
                ((enterprise_value * (0.3 + 0.7 * credit_score / CreditScore::MAX as f32)) as u32
                    / LOAN_STEP)
                    * LOAN_STEP
            },
            LoanProvider::AlternativeLender => {
                ((enterprise_value * 0.5) as u32 / LOAN_STEP) * LOAN_STEP
            },
        }
    }

    pub fn interest(
        &self,
        global_interest_rate: f32,
        credit_score: f32,
        term: &Term,
        no_fee: bool,
    ) -> f32 {
        let mut interest = match self {
            LoanProvider::Bank => {
                global_interest_rate
                    + 0.8 * global_interest_rate * (1. - credit_score / CreditScore::MAX as f32)
                    + 0.1 * global_interest_rate * (5. - term.years() as f32)
            },
            LoanProvider::AlternativeLender => {
                global_interest_rate
                    + 0.6 * global_interest_rate
                    + 0.1 * global_interest_rate * (5. - term.years() as f32)
            },
        };

        if no_fee {
            interest = 1.15 * interest
        }

        interest.round1()
    }
}

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum LoanKind {
    #[default]
    Annuity,
    StraightLine,
}

impl LoanKind {
    pub fn description(&self) -> &str {
        match self {
            LoanKind::Annuity => {
                "\
                Monthly payment is the same every month. Early payments are mostly interest, \
                later payments are mostly principal."
            },
            LoanKind::StraightLine => {
                "Principal repayment portion is the same each month. The interest \
                shrinks over time, thus the payments decrease each month."
            },
        }
    }
}

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Term {
    OneYear,
    ThreeYears,
    #[default]
    FiveYears,
}

impl Term {
    pub fn years(&self) -> u32 {
        match self {
            Term::OneYear => 1,
            Term::ThreeYears => 3,
            Term::FiveYears => 5,
        }
    }

    pub fn n_installments(&self) -> u32 {
        12 * self.years()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TermLoan {
    /// Loan identifier
    pub id: String,

    /// Institution that provided the loan
    pub provider: LoanProvider,

    /// The amount of money borrowed
    pub principal: f32,

    /// The amount left to pay back
    pub outstanding: f32,

    /// Number of installments already paid
    pub n_installments: u32,

    /// The interest over the loan
    pub interest_rate: f32,

    /// Global interest rate at the time of the contract
    pub global_interest_rate: f32,

    /// Type of loan
    pub kind: LoanKind,

    /// The number of years to pay back the loan
    pub term: Term,

    /// Whether this is a prepayment-free loan
    pub no_fee: bool,

    /// Date of the first installment
    pub start_date: NaiveDate,
}

impl TermLoan {
    pub fn next_principal_component(&self) -> f32 {
        let principal = match self.kind {
            LoanKind::StraightLine => self.principal / self.term.n_installments() as f32,
            LoanKind::Annuity => self.next_installment_amount() - self.next_interest_component(),
        };

        principal.min(self.outstanding)
    }

    pub fn next_interest_component(&self) -> f32 {
        self.outstanding * self.interest_rate / 100. / 12.
    }

    pub fn next_installment_amount(&self) -> f32 {
        match self.kind {
            LoanKind::StraightLine => {
                self.next_principal_component() + self.next_interest_component()
            },
            LoanKind::Annuity => {
                let interest = self.interest_rate / 100. / 12.;
                let installments = (self.term.n_installments() - self.n_installments) as f32;

                self.outstanding * interest / (1. - (1. + interest).powf(-installments))
            },
        }
    }

    pub fn installments_left(&self) -> u32 {
        self.term.n_installments() - self.n_installments
    }

    pub fn maturity_date(&self) -> NaiveDate {
        self.start_date
            .checked_add_months(Months::new(self.installments_left()))
            .unwrap()
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarginLoan {
    pub debt: f32,
    pub collateral: f32,
    pub interest_rate: f32,
    pub margin_frac: f32,
}

impl MarginLoan {
    const INIT_MARGIN: f32 = 0.5;

    pub fn new(price: f32, economy: &GlobalEconomy, player: &Player) -> Self {
        Self {
            debt: price,
            collateral: Self::INIT_MARGIN * price,
            interest_rate: (1.5 * economy.interest.current()
                + 0.5 * economy.interest.current() * (1. - player.credit_score.relative()))
            .round1(),
            margin_frac: 0.35 - (0.1 * player.credit_score.relative()),
        }
    }

    pub fn max_loan_debt(economy: &GlobalEconomy, player: &Player) -> f32 {
        player.enterprise_value(&economy) / 2. * (0.3 + 0.7 * player.credit_score.relative())
            - player.margin_loan_debt()
    }

    pub fn margin(&self, amount: i32) -> f32 {
        match amount {
            n if n > 0 => (self.debt - self.collateral) / (1. - self.margin_frac) / amount as f32,
            n if n < 0 => (self.debt + self.collateral) / (1. + self.margin_frac) / -amount as f32,
            _ => 0.,
        }
    }

    pub fn interest(&self) -> f32 {
        self.interest_rate / 100. / 12. * self.debt
    }
}
