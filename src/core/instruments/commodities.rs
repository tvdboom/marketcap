use crate::core::countries::CountryName;
use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum CommodityName {
    #[default]
    Aluminium,
    Cocoa,
    Copper,
    Gold,
    Iron,
    LiquidNaturalGas,
    Oil,
    Wheat,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Commodity {
    /// The name of the security
    pub name: CommodityName,

    /// The acronym of the unit the commodity is traded in
    pub unit: &'static str,

    /// The full name of unit the commodity is traded in
    pub full_unit: &'static str,
    
    /// The prices of the security over time
    pub prices: Vec<f32>,

    /// Percentage of price that can change daily
    pub volatility: f32,

    /// How many days before it matures
    pub maturity: Option<u32>,

    /// Countries where the commodity is produced
    pub production: Vec<CountryName>,
}

impl Commodity {
    pub fn description(&self) -> &str {
        match self.name {
            CommodityName::Aluminium => {
                "A lightweight, durable metal used in construction, transportation, and \
                packaging. Aluminium is a stable commodity with moderate volatility, \
                influenced by global demand, mining output, and energy costs. It is a \
                solid investment for those looking for exposure to industrial materials."
            },
            CommodityName::Cocoa => {
                "A key ingredient in chocolate and beauty products, cocoa is a seasonal agricultural commodity \
                with moderate volatility. Its price is influenced by weather conditions, \
                crop yields, and global demand for chocolate products. Cocoa can be a \
                stable investment but may experience price spikes during poor harvests."
            },
            CommodityName::Gold => {
                "A precious metal valued for its rarity, durability, and historical role as \
                a store of value. Gold serves as a stable but slow-growing investment, and \
                a hedge against inflation. While not highly volatile, gold retains value \
                even during market crashes, making it a strategic asset in times of crisis. \
                Gold doesn't degrade over time, allowing it to be held indefinitely."
            },
            CommodityName::Oil => {
                "A high-demand fossil fuel crucial to the energy sector. Oil is a volatile \
                commodity influenced by geopolitical tensions, supply disruptions, OPEC \
                decisions, and economic cycles. Its price can spike during conflicts or \
                shortages but also drop sharply during recessions or oversupply. Oil is a \
                high-risk, high-reward investment that can generate large profits or losses \
                quickly, making it ideal for aggressive traders or those hedging industrial \
                operations."
            },
            CommodityName::Wheat => {
                "A staple agricultural commodity essential for global food supply. Wheat \
                represents a relatively stable but seasonally influenced asset. Its price \
                is affected by weather patterns, crop yields and trade policies. Wheat is a \
                solid option for diversifying portfolios."
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
        if self.name == CommodityName::Gold {
            if inflation > 6. {
                // If inflation is high, gold's price increases significantly
                new_price *= 1. + inflation / 200.;
            } else if inflation < 2.5 {
                // If inflation is low, gold's price slightly decreases
                new_price *= 1. - inflation / 200.;
            }
        }

        new_price = new_price.max(0.);

        self.prices.push(new_price);
        new_price
    }
}

pub fn start_commodities() -> Vec<Commodity> {
    vec![
        Commodity {
            name: CommodityName::Gold,
            unit: "g",
            full_unit: "gram",
            prices: vec![93.],
            volatility: 1.,
            maturity: None,
            production: vec![
                CountryName::China,
                CountryName::Russia,
                CountryName::Australia,
                CountryName::USA,
                CountryName::Canada,
            ],
        },
        Commodity {
            name: CommodityName::Gold,
            unit: "g",
            full_unit: "gram",
            prices: vec![93.],
            volatility: 1.,
            maturity: None,
            production: vec![
                CountryName::China,
                CountryName::Russia,
                CountryName::Australia,
                CountryName::USA,
                CountryName::Canada,
            ],
        },
        Commodity {
            name: CommodityName::Oil,
            unit: "bbl",
            full_unit: "barrel",
            prices: vec![65.],
            volatility: 5.,
            maturity: Some(365),
            production: vec![
                CountryName::USA,
                CountryName::Russia,
                CountryName::Canada,
                CountryName::Venezuela,
            ],
        },
        Commodity {
            name: CommodityName::Wheat,
            unit: "ton",
            full_unit: "metric ton",
            prices: vec![7.],
            volatility: 2.3,
            maturity: Some(280),
            production: vec![
                CountryName::China,
                CountryName::Russia,
                CountryName::USA,
                CountryName::EU,
                CountryName::Ukraine,
            ],
        },
    ]
}
