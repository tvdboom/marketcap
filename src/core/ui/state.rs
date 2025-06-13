use bevy::prelude::*;
use strum_macros::EnumIter;

use crate::core::instruments::bonds::BondKind;
use crate::core::loans::{LoanKind, LoanProvider, Term};
use crate::core::orders::OrderKind;
use crate::core::player::InstrumentKind;

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq)]
pub enum Tab {
    #[default]
    Overview,
    Stocks,
    Bonds,
    Forex,
    Crypto,
    Commodities,
    Credit,
    Policies,
}

impl Tab {
    pub fn emoji(&self) -> &str {
        match self {
            Tab::Overview => "🗺",
            Tab::Stocks => "📈",
            Tab::Bonds => "💵",
            Tab::Forex => "💱",
            Tab::Crypto => "💸",
            Tab::Commodities => "🌾",
            Tab::Credit => "💳",
            Tab::Policies => "📜",
        }
    }
}

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq)]
pub enum OverviewTab {
    #[default]
    Portfolio,
    OrderBook,
    Debts,
}

impl OverviewTab {
    pub fn emoji(&self) -> &str {
        match self {
            OverviewTab::Portfolio => "📊",
            OverviewTab::OrderBook => "📋",
            OverviewTab::Debts => "💳",
        }
    }
}

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq)]
pub enum OrderOptions {
    #[default]
    Name,
    Created,
    Processed,
    OwnedAmount,
    OwnedValue,
    Price,
    Status,
    Volatility,
    Interest,
}

#[derive(Default)]
pub struct OrderByState {
    pub order: OrderOptions,
    pub descending: bool,
}

#[derive(Default)]
pub struct OverviewState {
    pub tab: OverviewTab,
    pub commodities: OrderByState,
    pub pending: OrderByState,
    pub processed: OrderByState,
}

#[derive(Default)]
pub struct BondState {
    pub tab: BondKind,
    pub order: OrderByState,
}

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq)]
pub enum CreditTab {
    #[default]
    NewLoan,
    RepayLoan,
    P2P,
}

impl CreditTab {
    pub fn emoji(&self) -> &str {
        match self {
            CreditTab::NewLoan => "✏",
            CreditTab::RepayLoan => "💰",
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
    pub term: Term,
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
            term: Term::default(),
            no_fee: false,
            repay: None,
            repay_amount: 0,
        }
    }
}

#[derive(Default)]
pub struct ModalInfo {
    pub tab: OrderKind,
    pub amount: u32,
    pub limit_stop: u32,
    pub trailing_stop: u32,
}

#[derive(Resource, Default)]
pub struct UiState {
    pub tab: Tab,
    pub overview: OverviewState,
    pub bonds: BondState,
    pub commodities: OrderByState,
    pub credit: CreditState,
    pub modal: Option<InstrumentKind>,
    pub modal_info: ModalInfo,
}
