use crate::core::instruments::bonds::BondKind;
use crate::core::loans::{LoanKind, LoanProvider, Term};
use crate::core::player::InstrumentKind;
use bevy::prelude::*;
use strum_macros::EnumIter;

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

#[derive(Default)]
pub struct OverviewState {
    pub tab: OverviewTab,
    pub order_book_order: OrderOptions,
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

#[derive(EnumIter, Clone, Copy, Default, Debug, PartialEq)]
pub enum TradeTab {
    #[default]
    MarketOrder,
    LimitOrder,
    ShortSelling,
    Futures,
}

impl TradeTab {
    pub fn emoji(&self) -> &str {
        match self {
            TradeTab::MarketOrder => "🏪",
            TradeTab::LimitOrder => "♾",
            TradeTab::ShortSelling => "📉",
            TradeTab::Futures => "🔮",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            TradeTab::MarketOrder => "Buy or sell an instrument at the current market price.",
            TradeTab::LimitOrder => {
                "Set a specific price to buy or sell an instrument. The order will automatically \
                execute if the instrument reaches that price. If there isn't enough cash at the \
                time of execution, the order will be cancelled."
            },
            TradeTab::ShortSelling => {
                "Short selling is a trading strategy where an investor bets against an instrument, \
                expecting its price to decline. First, the investor borrows shares from a broker \
                and immediately sells them at the current market price. If the stock price drops, \
                the investor can buy the shares back at a lower price and return them to the \
                broker, pocketing the difference as profit. The investor pays interest during \
                the time the shares are borrowed. If the stock price rises, the investor must \
                buy back the shares at a higher price, resulting in a loss."
            },
            TradeTab::Futures => {
                "Financial contracts to buy or sell instruments against a predetermined price \
                in the future. "
            },
        }
    }
}

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq)]
pub enum OrderOptions {
    #[default]
    Alphabetical,
    OwnedAmount,
    OwnedValue,
    LowestPrice,
    HighestPrice,
    LowestVolatility,
    HighestVolatility,
    LowestInterest,
    HighestInterest,
}

#[derive(Default)]
pub struct ModalInfo {
    pub tab: TradeTab,
    pub order: OrderOptions,
    pub amount: u32,
    pub limit_price: u32,
}

#[derive(Resource, Default)]
pub struct UiState {
    pub tab: Tab,
    pub overview: OverviewState,
    pub bonds: BondKind,
    pub credit: CreditState,
    pub active_modal: Option<InstrumentKind>,
    pub bond_modal: ModalInfo,
    pub commodity_modal: ModalInfo,
}
