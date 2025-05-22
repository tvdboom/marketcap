use crate::core::constants::MIN_PRINCIPAL;
use crate::core::loans::{LoanKind, LoanProvider};
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

#[derive(EnumIter, Clone, Debug, Default, PartialEq)]
pub enum LoanTerm {
    #[default]
    OneYear,
    ThreeYears,
    FiveYears,
}

impl LoanTerm {
    pub fn n_terms(&self) -> u32 {
        match self {
            LoanTerm::OneYear => 12,
            LoanTerm::ThreeYears => 36,
            LoanTerm::FiveYears => 60,
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
    /// The maximum amount of money that can be borrowed in steps of `MIN_PRINCIPAL`
    pub fn max_principal(&self, enterprise_value: f32, credit_score: f32) -> u32 {
        ((enterprise_value * (0.3 + 0.7 * credit_score / 100.)) as u32 / MIN_PRINCIPAL)
            * MIN_PRINCIPAL
    }

    /// The interest rate of the loan
    pub fn interest(&self, interest_rate: f32, credit_score: f32) -> f32 {
        interest_rate
            + 1.4 * interest_rate / 100. * (1. - credit_score / 100.)
            + 0.5 * interest_rate / 100. * (6. - self.term.n_terms() as f32)
    }

    /// Amount to pay on the first installment
    pub fn installment(&self, interest_rate: f32) -> f32 {
        self.kind.installment(
            self.principal,
            self.principal,
            interest_rate,
            self.term.n_terms(),
        )
    }
}

impl Default for LoanState {
    fn default() -> Self {
        Self {
            provider: LoanProvider::default(),
            principal: MIN_PRINCIPAL,
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
