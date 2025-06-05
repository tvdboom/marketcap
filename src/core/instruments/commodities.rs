use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::factors::economy::Economy;
use crate::core::instruments::Instrument;
use crate::utils::NameFromEnum;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Unit {
    Gram,
    Barrel,
    MetricTon,
    MillionBritishThermalUnits,
}

impl Unit {
    pub fn abbr(&self) -> &'static str {
        match self {
            Unit::Gram => "g",
            Unit::Barrel => "bbl",
            Unit::MetricTon => "t",
            Unit::MillionBritishThermalUnits => "MMBtu",
        }
    }
}

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum CommodityName {
    #[default]
    Aluminium,
    Cocoa,
    Coffee,
    Copper,
    Corn,
    Gold,
    Iron,
    LNG,
    Oil,
    Silicon,
    Wheat,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Commodity {
    /// The name of the commodity
    pub name: CommodityName,

    /// The unit the commodity is traded in
    pub unit: Unit,

    /// The prices of the commodity over time
    pub prices: Vec<f32>,

    /// Percentage of price that can change daily
    pub volatility: f32,

    /// Factor with which the price follows the global economy
    /// If positive, the price increases when the economy blooms.
    /// If negative, the price decreases when the economy is in recess.
    pub economy_factor: f32,

    /// Storage cost price per unit per day
    pub storage_cost: f32,
}

impl Commodity {
    pub fn description(&self) -> &str {
        match self.name {
            CommodityName::Aluminium => {
                "A lightweight, durable metal used in construction, transportation, and \
                packaging. Aluminium is a stable commodity with very low volatility, \
                influenced by global demand, mining output, and energy costs. It is a \
                solid investment for those looking for exposure to industrial materials."
            },
            CommodityName::Cocoa => {
                "A key ingredient in chocolate and beauty products, cocoa is a seasonal \
                agricultural commodity with high volatility. Its price is influenced by \
                weather conditions and crop yields."
            },
            CommodityName::Coffee => {
                "A globally traded beverage commodity, coffee is subject to high volatility \
                due to weather conditions, crop yields, and global demand."
            },
            CommodityName::Copper => {
                "A versatile metal known for its high conductivity, durability and corrosion \
                resistance. Copper is widely used in electrical wiring and construction. Its \
                a volatile commodity that greatly influences multiple sectors like technology,\
                military and transportation."
            },
            CommodityName::Corn => {
                "A staple agricultural commodity used for food, animal feed, and biofuels. \
                Corn is a relatively stable asset."
            },
            CommodityName::Gold => {
                "A precious metal valued for its rarity, durability, and historical role as \
                a store of value. Gold serves as a stable but slow-growing investment, and \
                a hedge against inflation. While not highly volatile, gold retains value \
                even during market crashes, making it a strategic asset in times of crisis."
            },
            CommodityName::Iron => {
                "Iron is a strong and abundant metal. As the backbone of steel production, \
                its frequently used in construction and manufacturing. Iron is a stable \
                commodity with low volatility."
            },
            CommodityName::LNG => {
                "Liquid Natural Gas (LNG) is primarily methane that has been cooled for \
                easier storage and transport. Its a widely used commodity for power generation, \
                heating and transportation. LNG is highly volatile and the prices are greatly \
                influenced by geopolitical factors and supply chain disruptions. Its expensive \
                storage costs make it a high-risk investment."
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
            CommodityName::Silicon => {
                "A key component in electronics and solar panels, silicon is a highly volatile \
                commodity. Its price is influenced by technological advancements and global \
                demand for electronics."
            },
            CommodityName::Wheat => {
                "A staple agricultural commodity essential for global food supply. Wheat \
                represents a relatively stable but seasonally influenced asset. Its price \
                is affected by weather patterns, crop yields and trade policies. Wheat is a \
                solid option for diversifying portfolios."
            },
        }
    }

    pub fn bump(&mut self, economy: f32, inflation: f32) -> f32 {
        let mut new_price = self.current()
            * (1. + inflation / 100. / 365.)
            * (1. + rng().random_range(-self.volatility / 100. ..self.volatility / 100.));

        // If the economy is doing really good or bad, it affects prices
        if economy < 25. || economy > 75. {
            new_price *= 1. + self.economy_factor * (economy - Economy::DEFAULT) / 300.;
        }

        new_price = new_price.max(0.);

        self.prices.push(new_price);
        new_price
    }
}

impl Instrument for Commodity {
    fn name(&self) -> String {
        self.name.to_name()
    }

    fn lowername(&self) -> String {
        self.name.to_lowername()
    }

    fn all(&self) -> &Vec<f32> {
        &self.prices
    }

    fn current(&self) -> f32 {
        *self.prices.last().unwrap()
    }

    fn unit(&self) -> String {
        self.unit.abbr().to_string()
    }

    fn storage_cost(&self) -> f32 {
        self.storage_cost
    }
}

pub fn start_commodities() -> Vec<Commodity> {
    vec![
        Commodity {
            name: CommodityName::Aluminium,
            unit: Unit::MetricTon,
            prices: vec![2200.],
            volatility: 0.6,
            economy_factor: 0.05,
            storage_cost: 0.5,
        },
        Commodity {
            name: CommodityName::Cocoa,
            unit: Unit::MetricTon,
            prices: vec![9762.],
            volatility: 6.2,
            economy_factor: -0.05,
            storage_cost: 0.2,
        },
        Commodity {
            name: CommodityName::Coffee,
            unit: Unit::MetricTon,
            prices: vec![3300.],
            volatility: 4.5,
            economy_factor: 0.04,
            storage_cost: 0.5,
        },
        Commodity {
            name: CommodityName::Copper,
            unit: Unit::MetricTon,
            prices: vec![9623.],
            volatility: 4.4,
            economy_factor: 0.05,
            storage_cost: 0.6,
        },
        Commodity {
            name: CommodityName::Corn,
            unit: Unit::MetricTon,
            prices: vec![215.],
            volatility: 2.5,
            economy_factor: -0.02,
            storage_cost: 0.1,
        },
        Commodity {
            name: CommodityName::Gold,
            unit: Unit::Gram,
            prices: vec![93.],
            volatility: 0.3,
            economy_factor: -0.01,
            storage_cost: 0.05,
        },
        Commodity {
            name: CommodityName::Iron,
            unit: Unit::MetricTon,
            prices: vec![125.],
            volatility: 0.5,
            economy_factor: 0.08,
            storage_cost: 0.2,
        },
        Commodity {
            name: CommodityName::LNG,
            unit: Unit::MillionBritishThermalUnits,
            prices: vec![13.],
            volatility: 7.2,
            economy_factor: 0.12,
            storage_cost: 0.05,
        },
        Commodity {
            name: CommodityName::Oil,
            unit: Unit::Barrel,
            prices: vec![65.],
            volatility: 5.,
            economy_factor: 0.09,
            storage_cost: 0.1,
        },
        Commodity {
            name: CommodityName::Silicon,
            unit: Unit::MetricTon,
            prices: vec![6000.],
            volatility: 6.5,
            economy_factor: 0.07,
            storage_cost: 0.4,
        },
        Commodity {
            name: CommodityName::Wheat,
            unit: Unit::MetricTon,
            prices: vec![201.],
            volatility: 2.3,
            economy_factor: -0.05,
            storage_cost: 0.1,
        },
    ]
}
