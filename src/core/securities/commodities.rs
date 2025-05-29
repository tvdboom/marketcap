use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CommodityKind {
    Gold,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Commodity {
    /// The kind of commodity
    pub kind: CommodityKind,

    /// The prices of the commodity over time
    pub prices: Vec<f32>,

    /// Percentage of price that can change daily
    pub volatility: f32,

    /// How many months the commodity can be held before it degrades
    pub maturity: Option<u8>,
}

impl Commodity {
    pub fn description(&self) -> &str {
        match self.kind {
            CommodityKind::Gold => {
                "\
                A precious metal valued for its rarity, durability, and historical role as
                a store of value. Gold serves as a stable but slow-growing investment, and
                a hedge against inflation. While not highly volatile, gold retains value
                even during market crashes, making it a strategic asset in times of crisis.
                Gold doesn't degrade over time, allowing it to be held indefinitely."
            },
        }
    }

    pub fn current(&self) -> f32 {
        *self.prices.last().unwrap()
    }

    pub fn bump(&mut self, inflation: f32) -> f32 {
        let mut new_price = self.current()
            * (1. + inflation / 100. / 365.)
            * (1. + rng().random_range(-self.volatility / 100. ..self.volatility / 100.));

        // Gold is a special case since its price moves with the inflation
        if self.kind == CommodityKind::Gold {
            if inflation > 6. {
                // If inflation is high, gold's price increases significantly
                new_price *= 1. + inflation / 100.;
            } else if inflation < 2.5 {
                // If inflation is low, gold's price slightly decreases
                new_price *= 1. - inflation / 100.;
            }
        }

        self.prices.push(new_price);
        new_price
    }
}

pub fn start_commodities() -> Vec<Commodity> {
    vec![Commodity {
        kind: CommodityKind::Gold,
        prices: vec![93.],
        volatility: 3.,
        maturity: None,
    }]
}
