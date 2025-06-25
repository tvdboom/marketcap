use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::core::instruments::bonds::{BondIssuer, BondQuality};
use crate::core::instruments::commodities::CommodityName;
use crate::core::instruments::crypto::CryptoName;
use crate::core::instruments::stocks::{Company, ESGRating};
use crate::core::orders::OrderKind;
use crate::core::sectors::SectorName;
use crate::utils::NameFromEnum;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum InstrumentKind {
    Stock(Company),
    Bond(BondIssuer),
    Commodity(CommodityName),
    Crypto(CryptoName),
}

impl InstrumentKind {
    pub fn name(&self) -> String {
        match self {
            InstrumentKind::Stock(name) => name.to_name(),
            InstrumentKind::Bond(name) => name.to_name(),
            InstrumentKind::Commodity(name) => name.to_name(),
            InstrumentKind::Crypto(name) => name.to_name(),
        }
    }

    pub fn lowername(&self) -> String {
        match self {
            InstrumentKind::Stock(name) => name.to_lowername(),
            InstrumentKind::Bond(name) => name.to_lowername(),
            InstrumentKind::Commodity(name) => name.to_lowername(),
            InstrumentKind::Crypto(name) => name.to_lowername(),
        }
    }

    pub fn order_options(&self) -> Vec<OrderKind> {
        match self {
            InstrumentKind::Bond(_) => vec![OrderKind::MarketOrder],
            InstrumentKind::Commodity(_) => vec![
                OrderKind::MarketOrder,
                OrderKind::LimitOrder,
                OrderKind::TrailingOrder,
                OrderKind::ShortSell,
            ],
            InstrumentKind::Crypto(_) => vec![
                OrderKind::MarketOrder,
                OrderKind::LimitOrder,
                OrderKind::TrailingOrder,
            ],
            _ => OrderKind::iter().collect(),
        }
    }
}

pub trait Instrument {
    fn name(&self) -> String;
    fn lowername(&self) -> String;
    fn image(&self) -> String {
        self.lowername()
    }
    fn description(&self) -> &str;
    fn kind(&self) -> InstrumentKind;
    fn all(&self) -> &Vec<f32>;
    fn current(&self) -> f32;

    /// Calculates the percentage difference from the average of the last 30 values
    fn diff(&self) -> f32 {
        // Add 30 initial values to ensure we always have at least 30 values
        let mut slice = vec![self.all()[0]; 29];
        slice.extend(self.all());

        let len = slice.len();
        let slice = &slice[len - 30..];

        let avg = slice.iter().sum::<f32>() / slice.len() as f32;

        if avg == 0.0 {
            0.0
        } else {
            (self.current() - avg) / avg * 100.
        }
    }

    fn dividend(&self) -> f32 {
        0.0
    }
    fn esg(&self) -> ESGRating {
        ESGRating::AAA
    }
    fn interest(&self) -> f32 {
        0.0
    }
    fn market_cap(&self) -> f32 {
        0.
    }
    fn quality(&self) -> BondQuality {
        BondQuality::AAA
    }
    fn sector(&self) -> HashMap<SectorName, f32> {
        HashMap::new()
    }
    fn sentiment(&self) -> u8 {
        0
    }
    fn storage_cost(&self) -> f32 {
        0.
    }
    fn volatility(&self) -> f32 {
        0.
    }
    fn unit(&self) -> String {
        "".to_string()
    }
    fn per_unit(&self) -> String {
        if self.unit().is_empty() {
            "".to_string()
        } else {
            format!("/{}", self.unit())
        }
    }
}
