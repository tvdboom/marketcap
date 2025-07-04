use crate::core::derivatives::OptionKind;
use crate::core::instruments::bonds::{BondIssuer, BondQuality};
use crate::core::instruments::commodities::CommodityName;
use crate::core::instruments::crypto::CryptoName;
use crate::core::instruments::forex::CurrencyName;
use crate::core::instruments::stocks::{Company, ESGRating};
use crate::core::orders::OrderKind;
use crate::core::sectors::SectorName;
use crate::utils::{DQueue, NameFromEnum, norm_cdf};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::f32::consts::E;
use strum::IntoEnumIterator;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum InstrumentKind {
    Stock(Company),
    Bond(BondIssuer),
    Forex(CurrencyName),
    Commodity(CommodityName),
    Crypto(CryptoName),
}

impl InstrumentKind {
    pub fn name(&self) -> String {
        match self {
            InstrumentKind::Stock(name) => name.to_name(),
            InstrumentKind::Bond(name) => name.to_name(),
            InstrumentKind::Forex(name) => name.to_name(),
            InstrumentKind::Commodity(name) => name.to_name(),
            InstrumentKind::Crypto(name) => name.to_name(),
        }
    }

    pub fn lowername(&self) -> String {
        match self {
            InstrumentKind::Stock(name) => name.to_lowername(),
            InstrumentKind::Bond(name) => name.to_lowername(),
            InstrumentKind::Forex(name) => name.to_lowername(),
            InstrumentKind::Commodity(name) => name.to_lowername(),
            InstrumentKind::Crypto(name) => name.to_lowername(),
        }
    }

    pub fn order_options(&self) -> Vec<OrderKind> {
        match self {
            InstrumentKind::Bond(_) => vec![OrderKind::MarketOrder],
            InstrumentKind::Forex(_) => vec![
                OrderKind::MarketOrder,
                OrderKind::LimitOrder,
                OrderKind::TrailingOrder,
            ],
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
    fn fullname(&self) -> String {
        self.name().to_string()
    }
    fn image(&self) -> String {
        self.name().to_lowercase()
    }
    fn description(&self) -> &str;
    fn kind(&self) -> InstrumentKind;
    fn all(&self) -> &DQueue<f32>;
    fn current(&self) -> f32 {
        *self.all().back().unwrap()
    }

    fn future_price(&self, interest: f32, years: f32) -> f32 {
        self.current()
            * E.powf(
                (interest / 100. + self.storage_cost() / 100. * 365. + self.volatility() / 100.)
                    * years,
            )
    }

    /// Calculates the price of an option using the Black-Scholes formula
    fn option_price(&self, strike_price: f32, interest: f32, years: f32, kind: OptionKind) -> f32 {
        let s = self.current();
        let k = strike_price;
        let t = years;
        let r = interest / 100.;
        let sigma = self.volatility() / 100. * f32::sqrt(365.);  // Convert daily volatility to annual

        let d1 = (f32::ln(s / k) + (r + 0.5 * sigma * sigma) * t) / (sigma * f32::sqrt(t));
        let d2 = d1 - sigma * f32::sqrt(t);

        match kind {
            OptionKind::Call => s * norm_cdf(d1) - k * E.powf(-r * t) * norm_cdf(d2),
            OptionKind::Put => k * E.powf(-r * t) * norm_cdf(-d2) - s * norm_cdf(-d1),
        }
    }

    /// Calculates the percentage difference from the average of the last 30 values
    fn diff(&self) -> f32 {
        // Add 30 initial values to ensure we always have at least 30 values
        let mut slice = vec![*self.all().front().unwrap(); 29];
        slice.extend(self.all().iter().collect::<Vec<_>>());

        let len = slice.len();
        let slice = &slice[len - 30..];

        let avg = slice.iter().sum::<f32>() / slice.len() as f32;

        if avg == 0.0 {
            0.0
        } else {
            (self.current() - avg) / avg * 100.
        }
    }

    fn symbol(&self) -> &str {
        ""
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

    /// Costs of holding this instrument per unit per day
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
