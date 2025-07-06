use std::cmp::Ordering;

use bevy::prelude::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::derivatives::{Derivative, DerivativeTerm};
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::bonds::BondKind;
use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::core::loans::{LoanKind, LoanProvider, Term, TermLoan};
use crate::core::orders::{Order, OrderKind};
use crate::core::player::{OwnedInstrument, Player};
use crate::utils::NameFromEnum;

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
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

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum OverviewTab {
    #[default]
    Portfolio,
    OrderBook,
    Derivatives,
    Debts,
}

impl OverviewTab {
    pub fn emoji(&self) -> &str {
        match self {
            OverviewTab::Portfolio => "📊",
            OverviewTab::OrderBook => "📋",
            OverviewTab::Derivatives => "🔮",
            OverviewTab::Debts => "💳",
        }
    }
}

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum OrderOptions {
    #[default]
    Name,
    Kind,
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
    Action,
    Volatility,
    Quality,
    Debt,
    Collateral,
    Interest,
    Margin,
    Execute,
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
        mut data: Vec<&'a dyn Instrument>,
        state: &OrderByState,
        economy: &GlobalEconomy,
        player: &Player,
    ) -> Vec<&'a dyn Instrument> {
        data.sort_by(|&a, &b| Self::reorder(a, b, state, economy, player));

        if state.descending {
            data.reverse();
        }

        data
    }

    pub fn sort_owned_instrument<'a>(
        mut data: Vec<&'a OwnedInstrument>,
        state: &OrderByState,
        economy: &GlobalEconomy,
        player: &Player,
    ) -> Vec<&'a OwnedInstrument> {
        data.sort_by(|a, b| {
            let a_econ = economy.get(&a.kind);
            let b_econ = economy.get(&b.kind);
            Self::reorder(a_econ, b_econ, state, economy, player)
        });

        if state.descending {
            data.reverse();
        }

        data
    }

    pub fn sort_order(data: Vec<&Order>, state: &OrderByState) -> Vec<Order> {
        let mut data: Vec<Order> = data.into_iter().cloned().collect();

        data.sort_by(|a, b| match state.order {
            Self::Name => a.instrument.lowername().cmp(&b.instrument.lowername()),
            Self::Created | Self::Processed => a.created.cmp(&b.created),
            Self::Price => a.threshold.partial_cmp(&b.threshold).unwrap(),
            Self::Status => a.status.cmp(&b.status),
            _ => unreachable!(),
        });

        if state.descending {
            data.reverse();
        }

        data
    }

    fn reorder_derivative(a: &Derivative, b: &Derivative, state: &OrderByState) -> Ordering {
        match state.order {
            Self::Name => a.instrument.name().cmp(&b.instrument.name()),
            Self::Kind => a.kind.to_name().cmp(&b.kind.to_name()),
            Self::Action => a.action.to_name().cmp(&b.action.to_name()),
            Self::Maturity => a.maturity_date().cmp(&b.maturity_date()),
            Self::OwnedAmount => a.amount.cmp(&b.amount),
            Self::OwnedValue => (a.amount as f32 * a.price)
                .partial_cmp(&(b.amount as f32 * b.price))
                .unwrap(),
            Self::Price => a.price.partial_cmp(&b.price).unwrap(),
            Self::Execute => a.execute.cmp(&b.execute),
            _ => unreachable!(),
        }
    }

    pub fn sort_derivative<'a>(
        mut data: Vec<&'a Derivative>,
        state: &OrderByState,
    ) -> Vec<&'a Derivative> {
        data.sort_by(|a, b| Self::reorder_derivative(a, b, state));

        if state.descending {
            data.reverse();
        }

        data
    }

    pub fn sort_derivative_mut<'a>(
        mut data: Vec<&'a mut Derivative>,
        state: &OrderByState,
    ) -> Vec<&'a mut Derivative> {
        data.sort_by(|a, b| Self::reorder_derivative(a, b, state));

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

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderByState {
    pub order: OrderOptions,
    pub descending: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OverviewState {
    pub tab: OverviewTab,
    pub stocks: OrderByState,
    pub commodities: OrderByState,
    pub crypto: OrderByState,
    pub pending_order: OrderByState,
    pub processed_order: OrderByState,
    pub pending_derivative: OrderByState,
    pub processed_derivative: OrderByState,
    pub term_loan: OrderByState,
    pub margin_loan: OrderByState,
}

impl Default for OverviewState {
    fn default() -> Self {
        Self {
            tab: OverviewTab::default(),
            stocks: OrderByState {
                order: OrderOptions::OwnedValue,
                descending: true,
            },
            commodities: OrderByState {
                order: OrderOptions::OwnedValue,
                descending: true,
            },
            crypto: OrderByState {
                order: OrderOptions::OwnedValue,
                descending: true,
            },
            pending_order: OrderByState {
                order: OrderOptions::Created,
                descending: true,
            },
            processed_order: OrderByState {
                order: OrderOptions::Processed,
                descending: true,
            },
            pending_derivative: OrderByState {
                order: OrderOptions::Maturity,
                descending: false,
            },
            processed_derivative: OrderByState {
                order: OrderOptions::Maturity,
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

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BondState {
    pub tab: BondKind,
    pub order_government: OrderByState,
    pub order_corporate: OrderByState,
}

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum CreditTab {
    #[default]
    NewLoan,
    RepayLoan,
    IncreaseCollateral,
    P2P,
}

impl CreditTab {
    pub fn emoji(&self) -> &str {
        match self {
            CreditTab::NewLoan => "✏",
            CreditTab::RepayLoan => "💰",
            CreditTab::IncreaseCollateral => "💲",
            CreditTab::P2P => "👤",
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreditState {
    pub tab: CreditTab,
    pub provider: LoanProvider,
    pub principal: u32,
    pub kind: LoanKind,
    pub term: Term,
    pub no_fee: bool,
    pub repay: Option<String>,
    pub repay_amount: u32,
    pub increase: Option<String>,
    pub collateral_amount: u32,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ModalInfo {
    pub tab: OrderKind,
    pub amount: u32,
    pub cds: bool,
    pub limit_stop: f32,
    pub trailing_stop: u32,
    pub lower_bound: bool,
    pub loan: bool,
    pub memory_loan: bool,
    pub derivative_term: DerivativeTerm,
    pub strike_percentage: i32,
}

#[derive(Resource, Clone, Default, Serialize, Deserialize)]
pub struct UiState {
    pub tab: Tab,
    pub overview: OverviewState,
    pub stocks: OrderByState,
    pub bonds: BondState,
    pub forex: OrderByState,
    pub commodities: OrderByState,
    pub cryptos: OrderByState,
    pub credit: CreditState,
    pub modal: Option<InstrumentKind>,
    pub modal_info: ModalInfo,
}
