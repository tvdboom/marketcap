use crate::core::constants::LOAN_STEP;
use crate::core::factors::credit_score::CreditScore;
use crate::utils::Round1;
use chrono::{Months, NaiveDate};
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq)]
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
                Banks are the standard credit providers for companies. The maximum amount \
                that can be borrowed depends on the company's enterprise value and credit score. \
                The interest rate is based on the global interest rate, the credit score and \
                the payback period, where a longer payback period reduces the loan's interest.\n\n\
                If a company defaults on a loan (fails to pay an installment) four consecutive \
                times, its assets will be forcibly sold (usually for unfavorable terms) until \
                there is enough cash to pay back the complete loan."
            }
            LoanProvider::AlternativeLender => {
                "\
                Alternative credit providers offer loans were traditional institutions like \
                banks may hesitate, usually against a higher interest rate. Contrary to banks,\
                the credit score is not used in the calculations for the maximum principal nor \
                the interest rate.\n\n\
                If a player defaults on a loan (fails to pay an installment) three consecutive \
                times, its assets will be forcibly sold (usually for unfavorable terms) until \
                there is enough cash to pay back the complete loan."
            }
        }
    }

    pub fn max_principal(&self, enterprise_value: f32, credit_score: f32) -> u32 {
        match self {
            LoanProvider::Bank => {
                ((enterprise_value * (0.3 + 0.7 * credit_score / CreditScore::MAX as f32)) as u32
                    / LOAN_STEP)
                    * LOAN_STEP
            }
            LoanProvider::AlternativeLender => {
                ((enterprise_value * 0.5) as u32 / LOAN_STEP) * LOAN_STEP
            }
        }
    }

    pub fn interest(&self, global_interest_rate: f32, credit_score: f32, term: &LoanTerm) -> f32 {
        match self {
            LoanProvider::Bank => (global_interest_rate
                + 0.8 * global_interest_rate * (1. - credit_score / CreditScore::MAX as f32)
                + 0.1 * global_interest_rate * (5. - term.years() as f32))
                .round1(),
            LoanProvider::AlternativeLender => (global_interest_rate
                + 0.6 * global_interest_rate
                + 0.1 * global_interest_rate * (5. - term.years() as f32))
                .round1(),
        }
    }
}

#[derive(EnumIter, Clone, Debug, Default, PartialEq)]
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
            }
            LoanKind::StraightLine => {
                "Principal repayment portion is the same each month. The interest \
                shrinks over time, thus the payments decrease each month."
            }
        }
    }
}

#[derive(EnumIter, Clone, Debug, Default, PartialEq)]
pub enum LoanTerm {
    OneYear,
    ThreeYears,
    #[default]
    FiveYears,
}

impl LoanTerm {
    pub fn years(&self) -> u32 {
        match self {
            LoanTerm::OneYear => 1,
            LoanTerm::ThreeYears => 3,
            LoanTerm::FiveYears => 5,
        }
    }

    pub fn n_installments(&self) -> u32 {
        12 * self.years()
    }
}

#[derive(Clone)]
pub struct Loan {
    /// Id of the loan
    pub id: String,

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

    /// The number of years to pay back the loan
    pub term: LoanTerm,

    /// Date of the first installment
    pub start_date: NaiveDate,

    /// Number of defaults on the loan
    pub defaults: u8,
}

impl Loan {
    pub fn next_principal_component(&self) -> f32 {
        match self.kind {
            LoanKind::StraightLine => self.principal / self.term.n_installments() as f32,
            LoanKind::Annuity => self.next_installment_amount() - self.next_interest_component(),
        }
    }

    pub fn next_interest_component(&self) -> f32 {
        self.outstanding * self.interest_rate / 100. / 12.
    }

    pub fn next_installment_amount(&self) -> f32 {
        match self.kind {
            LoanKind::StraightLine => {
                self.next_principal_component() + self.next_interest_component()
            }
            LoanKind::Annuity => {
                let interest = self.interest_rate / 100. / 12.;
                let installments = self.term.n_installments() as f32;

                self.principal * (interest * (1. + interest).powf(installments))
                    / ((1. + interest).powf(installments) - 1.)
            }
        }
    }

    pub fn maturity_date(&self) -> NaiveDate {
        let mut clone = self.clone();
        let mut installments = 0;

        while clone.next_installment_amount() > 0. && clone.outstanding >= 1. {
            clone.outstanding -= clone.next_principal_component();
            installments += 1;
        }

        clone
            .start_date
            .checked_add_months(Months::new(installments))
            .unwrap()
    }
}
