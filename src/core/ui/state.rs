use crate::core::loans::{LoanKind, LoanProvider, LoanTerm};
use bevy::prelude::*;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq)]
pub enum Tab {
    #[default]
    Home,
    Stocks,
    Bonds,
    Currencies,
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
            Tab::Currencies => "💰",
            Tab::Commodities => "💎",
            Tab::Credit => "💳",
            Tab::Policies => "📜",
        }
    }
}

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq)]
pub enum CurrencyTab {
    #[default]
    Forex,
    Crypto,
}

impl CurrencyTab {
    pub fn emoji(&self) -> &str {
        match self {
            CurrencyTab::Forex => "💱",
            CurrencyTab::Crypto => "💸",
        }
    }
}

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq)]
pub enum CreditTab {
    #[default]
    Overview,
    NewLoan,
    P2P,
}

impl CreditTab {
    pub fn emoji(&self) -> &str {
        match self {
            CreditTab::Overview => "🗺",
            CreditTab::NewLoan => "✏",
            CreditTab::P2P => "👤",
        }
    }
}
#[derive(Clone)]
pub struct CreditState {
    pub tab: CreditTab,
    pub provider: LoanProvider,
    pub principal: u32,
    pub kind: LoanKind,
    pub term: LoanTerm,
    pub no_fee: bool,
    pub repay: Option<String>,
    pub repay_amount: u32,
}

impl Default for CreditState {
    fn default() -> Self {
        Self {
            tab: CreditTab::default(),
            provider: LoanProvider::default(),
            principal: 0,
            kind: LoanKind::default(),
            term: LoanTerm::default(),
            no_fee: false,
            repay: None,
            repay_amount: 0,
        }
    }
}

#[derive(Resource, Clone, Default)]
pub struct UiState {
    pub tab: Tab,
    pub currencies: CurrencyTab,
    pub credit: CreditState,
    pub menu: bool,
}
