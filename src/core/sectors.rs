use std::collections::HashMap;

use crate::core::factors::inflation::Inflation;
use crate::core::instruments::commodities::{Commodity, CommodityName};
use crate::core::instruments::instrument::Instrument;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Default, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SectorName {
    #[default]
    Energy,
    Fashion,
    Finance,
    Food,
    Healthcare,
    Materials,
    Military,
    Retail,
    Technology,
    Transport,
}

impl SectorName {
    pub fn emoji(&self) -> &str {
        match self {
            SectorName::Energy => "⚡",
            SectorName::Fashion => "👗",
            SectorName::Finance => "💰",
            SectorName::Food => "🍔",
            SectorName::Healthcare => "💊",
            SectorName::Materials => "🔨",
            SectorName::Military => "🔫",
            SectorName::Retail => "👤",
            SectorName::Technology => "💻",
            SectorName::Transport => "🚚",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            SectorName::Energy => {
                "Companies involved in the production and distribution of energy."
            },
            SectorName::Fashion => {
                "Apparel and fashion industry, including clothing and accessories."
            },
            SectorName::Finance => "Financial institutions, including banks and investment firms.",
            SectorName::Food => "Businesses related to food production and distribution.",
            SectorName::Healthcare => {
                "Healthcare providers, pharmaceuticals, and medical equipment."
            },
            SectorName::Materials => "Firms that produce raw materials and commodities.",
            SectorName::Military => "Defense contractors and military equipment manufacturers.",
            SectorName::Retail => "Companies that sell goods directly to consumers.",
            SectorName::Technology => {
                "Tech companies, including software and hardware manufacturers."
            },
            SectorName::Transport => "Logistics and transportation service providers.",
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Sector {
    pub name: SectorName,
    pub value: u8,
    pub commodities: HashMap<CommodityName, f32>,
}

impl Sector {
    pub const MIN: u8 = 0;
    pub const MAX: u8 = 100;

    pub fn bump(&mut self, inflation: f32, commodities: &Vec<Commodity>) {
        let mut new_value = self.value as f32
            * (1.0
                + commodities
                    .iter()
                    .map(|c| {
                        self.commodities
                            .get(&c.name)
                            .map(|weight| weight * c.current() - c.base_price)
                            .unwrap_or_default()
                    })
                    .sum::<f32>());

        // Finance is special since it doesn't depend on commodities, but on inflation
        if self.name == SectorName::Finance {
            new_value *= 1. + (inflation - Inflation::DEFAULT) / 100.;
        }

        // Adjust value to tend towards the middle
        let deviation = new_value - ((Self::MIN + Self::MAX) as f32 * 0.5);
        new_value *= 1. + -deviation * deviation.abs() / 5.;

        self.value = ((new_value / 100.) as u8).clamp(Self::MIN, Self::MAX);
    }

    pub fn update(&mut self, amount: i8) {
        self.value = ((self.value as i8 + amount) as u8).clamp(Self::MIN, Self::MAX);
    }
}

pub fn start_sectors() -> Vec<Sector> {
    vec![
        Sector {
            name: SectorName::Energy,
            value: 50,
            commodities: HashMap::from([
                (CommodityName::LNG, 0.4),
                (CommodityName::Oil, 0.4),
                (CommodityName::Ethanol, 0.2),
            ]),
        },
        Sector {
            name: SectorName::Fashion,
            value: 50,
            commodities: HashMap::from([
                (CommodityName::Cotton, 0.6),
                (CommodityName::Silver, 0.2),
                (CommodityName::Gold, 0.1),
            ]),
        },
        Sector {
            name: SectorName::Finance,
            value: 50,
            commodities: HashMap::new(),
        },
        Sector {
            name: SectorName::Food,
            value: 50,
            commodities: HashMap::from([
                (CommodityName::Wheat, 0.3),
                (CommodityName::Corn, 0.2),
                (CommodityName::Cocoa, 0.2),
                (CommodityName::Coffee, 0.2),
            ]),
        },
        Sector {
            name: SectorName::Healthcare,
            value: 50,
            commodities: HashMap::from([
                (CommodityName::Ethanol, 0.2),
                (CommodityName::LNG, 0.1),
                (CommodityName::Silver, 0.1),
                (CommodityName::Corn, 0.1),
                (CommodityName::Cotton, 0.1),
            ]),
        },
        Sector {
            name: SectorName::Materials,
            value: 50,
            commodities: HashMap::from([
                (CommodityName::Gold, 0.1),
                (CommodityName::Silver, 0.1),
                (CommodityName::Iron, 0.1),
                (CommodityName::Silicon, 0.1),
                (CommodityName::Aluminium, 0.1),
                (CommodityName::Copper, 0.1),
            ]),
        },
        Sector {
            name: SectorName::Military,
            value: 50,
            commodities: HashMap::from([
                (CommodityName::Iron, 0.5),
                (CommodityName::Aluminium, 0.3),
                (CommodityName::Oil, 0.3),
            ]),
        },
        Sector {
            name: SectorName::Retail,
            value: 50,
            commodities: HashMap::from([
                (CommodityName::Cotton, 0.2),
                (CommodityName::Silver, 0.1),
                (CommodityName::Aluminium, 0.1),
                (CommodityName::Coffee, 0.1),
                (CommodityName::Cocoa, 0.1),
            ]),
        },
        Sector {
            name: SectorName::Technology,
            value: 50,
            commodities: HashMap::from([
                (CommodityName::Silicon, 0.7),
                (CommodityName::Copper, 0.3),
            ]),
        },
        Sector {
            name: SectorName::Transport,
            value: 50,
            commodities: HashMap::from([
                (CommodityName::Iron, 0.4),
                (CommodityName::Aluminium, 0.25),
                (CommodityName::Oil, 0.25),
                (CommodityName::Copper, 0.1),
            ]),
        },
    ]
}
