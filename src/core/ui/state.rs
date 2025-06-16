use std::cmp::Ordering;

use bevy::prelude::*;
use itertools::Itertools;
use strum_macros::EnumIter;

use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::Instrument;
use crate::core::instruments::bonds::BondKind;
use crate::core::loans::{LoanKind, LoanProvider, Term};
use crate::core::orders::OrderKind;
use crate::core::player::{InstrumentKind, OwnedInstrument, Player};

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq)]
pub enum Tab {
    #[default]
    Overview,
    Stocks,
    Bonds,
    Forex,
    Commodities,
    Crypto,
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
            Tab::Commodities => "🌾",
            Tab::Crypto => "💸",
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
    Quality,
    Interest,
}

impl OrderOptions {
    fn order(
        a: &dyn Instrument,
        b: &dyn Instrument,
        state: &OrderByState,
        economy: &GlobalEconomy,
        player: &Player,
    ) -> Ordering {
        match state.order {
            OrderOptions::Name => a.lowername().cmp(&b.lowername()),
            OrderOptions::OwnedAmount => player
                .get_owned(&a.kind())
                .cmp(&player.get_owned(&b.kind())),
            OrderOptions::OwnedValue => player
                .get_value(&a.kind(), economy)
                .partial_cmp(&player.get_value(&b.kind(), economy))
                .unwrap_or(Ordering::Equal),
            OrderOptions::Price => a
                .current()
                .partial_cmp(&b.current())
                .unwrap_or(Ordering::Equal),
            OrderOptions::Volatility => a
                .volatility()
                .partial_cmp(&b.volatility())
                .unwrap_or(Ordering::Equal),
            OrderOptions::Quality => a.quality().cmp(&b.quality()),
            OrderOptions::Interest => a
                .interest()
                .partial_cmp(&b.interest())
                .unwrap_or(Ordering::Equal),
            _ => unreachable!(),
        }
    }

    pub fn sort<'a>(
        data: &mut Vec<&'a dyn Instrument>,
        state: &OrderByState,
        economy: &GlobalEconomy,
        player: &Player,
    ) -> Vec<&'a dyn Instrument> {
        let mut data = data
            .iter()
            .copied()
            .sorted_by(|&a, &b| Self::order(a, b, state, economy, player))
            .collect::<Vec<_>>();

        if state.descending {
            data.reverse();
        }

        data
    }

    pub fn sort_owned<'a>(
        data: &mut Vec<&'a OwnedInstrument>,
        state: &OrderByState,
        economy: &GlobalEconomy,
        player: &Player,
    ) -> Vec<&'a OwnedInstrument> {
        let mut data = data
            .iter()
            .copied()
            .sorted_by(|a, b| {
                let a = economy.get(&a.kind);
                let b = economy.get(&b.kind);
                Self::order(a, b, state, economy, &player)
            })
            .collect::<Vec<_>>();

        if state.descending {
            data.reverse();
        }

        data
    }
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
    pub crypto: OrderByState,
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
    pub lower_bound: bool,
}

#[derive(Resource, Default)]
pub struct UiState {
    pub tab: Tab,
    pub overview: OverviewState,
    pub bonds: BondState,
    pub commodities: OrderByState,
    pub cryptos: OrderByState,
    pub credit: CreditState,
    pub modal: Option<InstrumentKind>,
    pub modal_info: ModalInfo,
}
