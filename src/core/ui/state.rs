use crate::core::instruments::commodities::CommodityName;
use crate::core::loans::{LoanKind, LoanProvider, LoanTerm};
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
pub enum BondTab {
    #[default]
    Overview,
    Government,
    Corporate,
}

impl BondTab {
    pub fn emoji(&self) -> &str {
        match self {
            BondTab::Overview => "🗺",
            BondTab::Government => "💼",
            BondTab::Corporate => "🏢",
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

#[derive(Default, PartialEq)]
pub enum ActiveModal {
    #[default]
    Commodity,
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
                "Set a specific price to buy or sell an instrument. The order will only execute \
                if the instrument reaches that price."
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

#[derive(Default)]
pub struct CommodityModal {
    pub active: bool,
    pub name: CommodityName,
    pub amount: u32,
    pub tab: TradeTab,
    pub price: u32,
}

#[derive(Resource, Default)]
pub struct UiState {
    pub tab: Tab,
    pub bonds: BondTab,
    pub credit: CreditState,
    pub active_modal: Option<ActiveModal>,
    pub commodity_modal: CommodityModal,
}
