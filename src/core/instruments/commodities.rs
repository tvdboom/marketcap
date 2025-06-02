use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Serialize, Deserialize)]
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
            Unit::MetricTon => "ton",
            Unit::MillionBritishThermalUnits => "MMBtu",
        }
    }
}

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum CommodityName {
    #[default]
    Aluminium,
    Cocoa,
    Copper,
    Gold,
    Iron,
    LNG,
    Oil,
    Wheat,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Commodity {
    /// The name of the security
    pub name: CommodityName,

    /// The unit the commodity is traded in
    pub unit: Unit,

    /// The prices of the security over time
    pub prices: Vec<f32>,

    /// Percentage of price that can change daily
    pub volatility: f32,

    /// Storage cost price per unit per day
    pub storage: f32,
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
            CommodityName::Copper => {
                "A versatile metal known for its high conductivity, durability and corrosion \
                resistance. Copper is widely used in electrical wiring and construction. Its \
                a volatile commodity that greatly influences multiple sectors like technology,\
                military and transportation."
            },
            CommodityName::Gold => {
                "A precious metal valued for its rarity, durability, and historical role as \
                a store of value. Gold serves as a stable but slow-growing investment, and \
                a hedge against inflation. While not highly volatile, gold retains value \
                even during market crashes, making it a strategic asset in times of crisis. \
                Gold doesn't degrade over time, allowing it to be held indefinitely."
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
            name: CommodityName::Aluminium,
            unit: Unit::MetricTon,
            prices: vec![2200.],
            volatility: 0.6,
            storage: 0.5,
        },
        Commodity {
            name: CommodityName::Cocoa,
            unit: Unit::MetricTon,
            prices: vec![9762.],
            volatility: 6.2,
            storage: 0.2,
        },
        Commodity {
            name: CommodityName::Copper,
            unit: Unit::MetricTon,
            prices: vec![9623.],
            volatility: 4.4,
            storage: 0.6,
        },
        Commodity {
            name: CommodityName::Gold,
            unit: Unit::Gram,
            prices: vec![93.],
            volatility: 1.,
            storage: 0.05,
        },
        Commodity {
            name: CommodityName::Iron,
            unit: Unit::MetricTon,
            prices: vec![125.],
            volatility: 0.5,
            storage: 0.3,
        },
        Commodity {
            name: CommodityName::LNG,
            unit: Unit::MetricTon,
            prices: vec![13.],
            volatility: 14.2,
            storage: 1.0,
        },
        Commodity {
            name: CommodityName::Oil,
            unit: Unit::Barrel,
            prices: vec![65.],
            volatility: 5.,
            storage: 0.5,
        },
        Commodity {
            name: CommodityName::Wheat,
            unit: Unit::MetricTon,
            prices: vec![201.],
            volatility: 2.3,
            storage: 0.1,
        },
    ]
}
