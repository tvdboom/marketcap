use crate::core::constants::LOAN_STEP;
use crate::core::factors::credit_score::CreditScore;
use crate::core::loans::{LoanKind, LoanProvider, LoanTerm};
use crate::utils::Round1;
use bevy::prelude::*;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq)]
pub enum Tab {
    #[default]
    Home,
    Stocks,
    Bonds,
    Crypto,
    Commodities,
    Credit,
    Policies,
}

impl Tab {
    pub fn emoji(&self) -> &str {
        match self {
            Tab::Home => "🏠",
            Tab::Stocks => "📈",
            Tab::Bonds => "💵",
            Tab::Crypto => "💰",
            Tab::Commodities => "💎",
            Tab::Credit => "💳",
            Tab::Policies => "📜",
        }
    }
}

#[derive(Clone, Debug)]
pub struct LoanState {
    pub provider: LoanProvider,
    pub principal: u32,
    pub kind: LoanKind,
    pub term: LoanTerm,
}

impl LoanState {
    pub fn max_principal(
        &self,
        enterprise_value: f32,
        credit_score: f32,
        provider: LoanProvider,
    ) -> u32 {
        match provider {
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

    pub fn interest(&self, interest_rate: f32, credit_score: f32, provider: LoanProvider) -> f32 {
        match provider {
            LoanProvider::Bank => (interest_rate
                + 0.8 * interest_rate * (1. - credit_score / CreditScore::MAX as f32)
                + 0.1 * interest_rate * (5. - self.term.years() as f32))
                .round1(),
            LoanProvider::AlternativeLender => (interest_rate
                + 0.6 * interest_rate
                + 0.1 * interest_rate * (5. - self.term.years() as f32))
                .round1(),
        }
    }
}

impl Default for LoanState {
    fn default() -> Self {
        Self {
            provider: LoanProvider::default(),
            principal: 0,
            kind: LoanKind::default(),
            term: LoanTerm::default(),
        }
    }
}

#[derive(Resource, Clone, Debug, Default)]
pub struct UiState {
    pub tab: Tab,
    pub credit: LoanState,
}
