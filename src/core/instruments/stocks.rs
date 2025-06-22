use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use crate::core::countries::CountryName;
use crate::core::instruments::bonds::{Bond, BondIssuer, BondKind, BondQuality};
use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::core::loans::Term;
use crate::utils::NameFromEnum;

#[derive(EnumIter, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CompanyName {
    Apple,
    Boeing,
    GoldManSachs,
    Inditex,
    LockheedMartin,
    LVMH,
    Maersk,
    Moderna,
    Nestle,
    Nvidia,
    Pfizer,
    RioTinto,
    Shell,
    Toyota,
    Unilever,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Stock {
    /// The issuer of th stock
    pub issuer: CompanyName,

    /// Default price of the stock
    pub base_price: f32,

    /// The prices over time
    pub prices: Vec<f32>,

    /// Percentage of the base price that can change daily
    pub volatility: f32,
}


impl Instrument for Stock {
    fn name(&self) -> String {
        self.issuer.to_name()
    }

    fn lowername(&self) -> String {
        self.issuer.to_lowername()
    }
    
    fn description(&self) -> &str {
        ""
    }

    fn kind(&self) -> InstrumentKind {
        InstrumentKind::Stock(self.issuer)
    }

    fn all(&self) -> &Vec<f32> {
        &self.prices
    }

    fn current(&self) -> f32 {
        *self.prices.last().unwrap()
    }

    fn volatility(&self) -> f32 {
        self.volatility
    }
}

pub fn start_stocks() -> Vec<Stock> {
    vec![
        Stock {
            issuer: CompanyName::Boeing,
            base_price: 100.,
            prices: vec![100.],
            volatility: 3.0,
        },
    ]
}
