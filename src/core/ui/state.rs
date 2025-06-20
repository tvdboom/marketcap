use std::cmp::Ordering;

use bevy::prelude::*;
use itertools::Itertools;
use strum_macros::EnumIter;

use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::bonds::BondKind;
use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::core::loans::{LoanKind, LoanProvider, Term, TermLoan};
use crate::core::orders::{Order, OrderKind};
use crate::core::player::{OwnedInstrument, Player};
use crate::utils::NameFromEnum;

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

#[derive(EnumIter, Clone, Copy, Debug, PartialEq)]
pub enum OrderOptions {
    Name,
    Created,
    StartDate,
    Maturity,
    Provider,
    Principal,
    Outstanding,
    Installment,
    Processed,
    OwnedAmount,
    OwnedValue,
    Price,
    Status,
    Volatility,
    Quality,
    Debt,
    Collateral,
    Interest,
    Margin,
    Defaults,
}

impl OrderOptions {
    fn reorder(
        a: &dyn Instrument,
        b: &dyn Instrument,
        state: &OrderByState,
        economy: &GlobalEconomy,
        player: &Player,
    ) -> Ordering {
        match state.order {
            Self::Name => a.lowername().cmp(&b.lowername()),
            Self::OwnedAmount => player
                .get_owned(&a.kind())
                .cmp(&player.get_owned(&b.kind())),
            Self::OwnedValue => player
                .get_value(&a.kind(), economy)
                .partial_cmp(&player.get_value(&b.kind(), economy))
                .unwrap_or(Ordering::Equal),
            Self::Price => a
                .current()
                .partial_cmp(&b.current())
                .unwrap_or(Ordering::Equal),
            Self::Volatility => a
                .volatility()
                .partial_cmp(&b.volatility())
                .unwrap_or(Ordering::Equal),
            Self::Quality => a.quality().cmp(&b.quality()),
            Self::Interest => a
                .interest()
                .partial_cmp(&b.interest())
                .unwrap_or(Ordering::Equal),
            _ => unreachable!(),
        }
    }

    pub fn sort_instrument<'a>(
        data: &mut Vec<&'a dyn Instrument>,
        state: &OrderByState,
        economy: &GlobalEconomy,
        player: &Player,
    ) -> Vec<&'a dyn Instrument> {
        let mut data = data
            .iter()
            .copied()
            .sorted_by(|&a, &b| Self::reorder(a, b, state, economy, player))
            .collect::<Vec<_>>();

        if state.descending {
            data.reverse();
        }

        data
    }

    pub fn sort_owned_instrument<'a>(
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
                Self::reorder(a, b, state, economy, &player)
            })
            .collect::<Vec<_>>();

        if state.descending {
            data.reverse();
        }

        data
    }

    pub fn sort_order(data: Vec<&Order>, state: &OrderByState) -> Vec<Order> {
        let mut data = data
            .into_iter()
            .cloned()
            .sorted_by(|a, b| match state.order {
                Self::Name => a.instrument.lowername().cmp(&b.instrument.lowername()),
                Self::Created => a.created.cmp(&b.created),
                Self::Processed => a.created.cmp(&b.created),
                Self::Price => a.threshold.partial_cmp(&b.threshold).unwrap(),
                Self::Status => a.status.cmp(&b.status),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        if state.descending {
            data.reverse();
        }

        data
    }

    pub fn sort_term_loan(data: &Vec<TermLoan>, state: &OrderByState) -> Vec<TermLoan> {
        let mut data = data
            .into_iter()
            .cloned()
            .sorted_by(|a, b| match state.order {
                Self::StartDate => a.start_date.cmp(&b.start_date),
                Self::Maturity => a.maturity_date().cmp(&b.maturity_date()),
                Self::Provider => a.provider.to_name().cmp(&b.provider.to_name()),
                Self::Principal => a.principal.partial_cmp(&b.principal).unwrap(),
                Self::Outstanding => a.outstanding.partial_cmp(&b.outstanding).unwrap(),
                Self::Installment => a
                    .next_installment_amount()
                    .partial_cmp(&b.next_installment_amount())
                    .unwrap(),
                Self::Interest => a.interest_rate.partial_cmp(&b.interest_rate).unwrap(),
                Self::Defaults => a.defaults.cmp(&b.defaults),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        if state.descending {
            data.reverse();
        }

        data
    }

    pub fn sort_margin_loan(
        data: Vec<&OwnedInstrument>,
        state: &OrderByState,
        economy: &GlobalEconomy,
    ) -> Vec<OwnedInstrument> {
        let mut data = data
            .into_iter()
            .cloned()
            .sorted_by(|a, b| {
                let l1 = a.loan.as_ref().unwrap();
                let l2 = b.loan.as_ref().unwrap();
                match state.order {
                    Self::Name => a.kind.lowername().cmp(&b.kind.lowername()),
                    Self::Debt => l1.debt.partial_cmp(&l2.debt).unwrap(),
                    Self::Collateral => l1.collateral.partial_cmp(&l2.collateral).unwrap(),
                    Self::Interest => l1.interest().partial_cmp(&l2.interest()).unwrap(),
                    Self::Price => l1.interest().partial_cmp(&l2.interest()).unwrap(),
                    Self::Margin => economy
                        .get_price(&a.kind)
                        .partial_cmp(&economy.get_price(&b.kind))
                        .unwrap(),
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<_>>();

        if state.descending {
            data.reverse();
        }

        data
    }
}

pub struct OrderByState {
    pub order: OrderOptions,
    pub descending: bool,
}

pub struct OverviewState {
    pub tab: OverviewTab,
    pub commodities: OrderByState,
    pub crypto: OrderByState,
    pub pending: OrderByState,
    pub processed: OrderByState,
    pub term_loan: OrderByState,
    pub margin_loan: OrderByState,
}

impl Default for OverviewState {
    fn default() -> Self {
        Self {
            tab: OverviewTab::default(),
            commodities: OrderByState {
                order: OrderOptions::OwnedValue,
                descending: true,
            },
            crypto: OrderByState {
                order: OrderOptions::OwnedValue,
                descending: true,
            },
            pending: OrderByState {
                order: OrderOptions::Created,
                descending: true,
            },
            processed: OrderByState {
                order: OrderOptions::Processed,
                descending: true,
            },
            term_loan: OrderByState {
                order: OrderOptions::Principal,
                descending: true,
            },
            margin_loan: OrderByState {
                order: OrderOptions::Debt,
                descending: true,
            },
        }
    }
}

pub struct BondState {
    pub tab: BondKind,
    pub order: OrderByState,
}

impl Default for BondState {
    fn default() -> Self {
        Self {
            tab: BondKind::Government,
            order: OrderByState {
                order: OrderOptions::Name,
                descending: false,
            },
        }
    }
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
    pub limit_stop: f32,
    pub trailing_stop: u32,
    pub lower_bound: bool,
    pub loan: bool,
}

#[derive(Resource)]
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

impl Default for UiState {
    fn default() -> Self {
        Self {
            tab: Tab::default(),
            overview: OverviewState::default(),
            bonds: BondState::default(),
            commodities: OrderByState {
                order: OrderOptions::Name,
                descending: false,
            },
            cryptos: OrderByState {
                order: OrderOptions::Name,
                descending: false,
            },
            credit: CreditState::default(),
            modal: None,
            modal_info: ModalInfo::default(),
        }
    }
}
