use crate::core::events::EventName;
use crate::core::factors::economy::Economy;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::core::player::Player;
use crate::core::research::TechName;
use crate::utils::{DQueue, NameFromEnum};
use itertools::Itertools;
use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Unit {
    Kilogram,
    Gram,
    Barrel,
    CubicMeter,
    MetricTon,
    MillionBritishThermalUnits,
}

impl Unit {
    pub fn abbr(&self) -> &'static str {
        match self {
            Unit::Kilogram => "kg",
            Unit::Gram => "g",
            Unit::Barrel => "bbl",
            Unit::CubicMeter => "m³",
            Unit::MetricTon => "t",
            Unit::MillionBritishThermalUnits => "MMBtu",
        }
    }
}

#[derive(EnumIter, Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum CommodityName {
    #[default]
    Aluminium,
    Cocoa,
    Coffee,
    Copper,
    Corn,
    Cotton,
    Ethanol,
    Gold,
    Iron,
    LNG,
    Oil,
    Silicon,
    Silver,
    Wheat,
}

impl CommodityName {
    pub fn is_food(&self) -> bool {
        matches!(
            self,
            CommodityName::Cocoa
                | CommodityName::Coffee
                | CommodityName::Corn
                | CommodityName::Wheat
        )
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Commodity {
    /// The name of the commodity
    pub name: CommodityName,

    /// Default price of the commodity
    pub base_price: f32,

    /// The prices over time
    pub prices: DQueue<f32>,

    /// The unit the commodity is traded in
    pub unit: Unit,

    /// Percentage of the base price that can change daily
    pub volatility: f32,

    /// Factor with which the price follows the global economy
    /// If positive, the price increases when the economy blooms.
    /// If negative, the price decreases when the economy is in recess.
    pub economy_factor: f32,

    /// Storage cost as percentage of the base price per unit per day
    pub storage_cost: f32,
}

impl Commodity {
    pub fn bump(&mut self, economy: f32, inflation: f32) -> f32 {
        self.base_price *= 1. + inflation / 100. / 365.;

        let volatility = self.base_price * self.volatility / 100.;
        let mut new_price = self.current() * (1. + inflation / 100. / 365.)
            + rng().random_range(-volatility..volatility);

        // If the economy is doing really well or poorly, it affects prices
        if economy < 30. || economy > 70. {
            new_price *= 1. + self.economy_factor * (economy - Economy::DEFAULT) / 300.;
        }

        // Adjust price to tend towards the base price
        // At 100% deviation, there's a 20% adjustment towards the base price
        // At 50% deviation, there's a 5% adjustment towards the base price
        let deviation = (new_price - self.base_price) / self.base_price;
        new_price *= 1. + -deviation * deviation.abs() / 5.;

        new_price = new_price.max(1.);

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

    fn description(&self) -> &str {
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
                a volatile commodity that greatly influences multiple sectors like technology, \
                military and transportation."
            },
            CommodityName::Corn => {
                "A staple agricultural commodity used for food, animal feed, and biofuels. \
                Corn is a relatively stable asset."
            },
            CommodityName::Cotton => {
                "A natural fiber used in textiles, cotton is a seasonal agricultural commodity \
                with moderate volatility. It's a primary raw material for the clothing industry."
            },
            CommodityName::Ethanol => {
                "A biofuel derived from corn and sugarcane, ethanol is a renewable energy source \
                frequently used in healthcare."
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
            CommodityName::Silver => {
                "A precious metal with industrial applications, silver is less stable than \
                gold but still a solid investment. Its price is influenced by industrial \
                demand, mining output, and economic conditions."
            },
            CommodityName::Wheat => {
                "A staple agricultural commodity essential for global food supply. Wheat \
                represents a relatively stable but seasonally influenced asset. Its price \
                is affected by weather patterns, crop yields and trade policies. Wheat is a \
                solid option for diversifying portfolios."
            },
        }
    }

    fn kind(&self) -> InstrumentKind {
        InstrumentKind::Commodity(self.name)
    }

    fn all(&self) -> &DQueue<f32> {
        &self.prices
    }

    fn storage_cost(&self, economy: &GlobalEconomy, player: &Player) -> f32 {
        self.storage_cost / 100.
            * self.base_price
            * if player.has_tech(&TechName::ReducedStorage) {
                0.8
            } else {
                1.
            }
            * if economy.active_events().iter().map(|e| e.name).contains(&EventName::StorageCosts) {
                1.5
            } else {
                1.
            }
    }

    fn volatility(&self) -> f32 {
        self.volatility
    }

    fn unit(&self) -> String {
        self.unit.abbr().to_string()
    }
}

pub fn start_commodities() -> Vec<Commodity> {
    vec![
        Commodity {
            name: CommodityName::Aluminium,
            base_price: 2200.,
            prices: DQueue::from([2200.]),
            unit: Unit::MetricTon,
            volatility: 0.6,
            economy_factor: 0.05,
            storage_cost: 0.1,
        },
        Commodity {
            name: CommodityName::Cocoa,
            base_price: 9762.,
            prices: DQueue::from([9762.]),
            unit: Unit::MetricTon,
            volatility: 6.2,
            economy_factor: -0.05,
            storage_cost: 0.3,
        },
        Commodity {
            name: CommodityName::Coffee,
            base_price: 3300.,
            prices: DQueue::from([3300.]),
            unit: Unit::MetricTon,
            volatility: 4.5,
            economy_factor: 0.04,
            storage_cost: 0.2,
        },
        Commodity {
            name: CommodityName::Copper,
            base_price: 9623.,
            prices: DQueue::from([9623.]),
            unit: Unit::MetricTon,
            volatility: 4.4,
            economy_factor: 0.05,
            storage_cost: 0.15,
        },
        Commodity {
            name: CommodityName::Corn,
            base_price: 215.,
            prices: DQueue::from([215.]),
            unit: Unit::MetricTon,
            volatility: 2.5,
            economy_factor: -0.02,
            storage_cost: 0.4,
        },
        Commodity {
            name: CommodityName::Cotton,
            base_price: 85.,
            prices: DQueue::from([85.]),
            unit: Unit::MetricTon,
            volatility: 3.5,
            economy_factor: 0.03,
            storage_cost: 0.8,
        },
        Commodity {
            name: CommodityName::Ethanol,
            base_price: 470.,
            prices: DQueue::from([470.]),
            unit: Unit::CubicMeter,
            volatility: 5.7,
            economy_factor: 0.06,
            storage_cost: 0.6,
        },
        Commodity {
            name: CommodityName::Gold,
            base_price: 93.,
            prices: DQueue::from([93.]),
            unit: Unit::Gram,
            volatility: 0.3,
            economy_factor: -0.01,
            storage_cost: 0.02,
        },
        Commodity {
            name: CommodityName::Iron,
            base_price: 125.,
            prices: DQueue::from([125.]),
            unit: Unit::MetricTon,
            volatility: 0.5,
            economy_factor: 0.08,
            storage_cost: 0.07,
        },
        Commodity {
            name: CommodityName::LNG,
            base_price: 13.,
            prices: DQueue::from([13.]),
            unit: Unit::MillionBritishThermalUnits,
            volatility: 7.2,
            economy_factor: 0.12,
            storage_cost: 1.,
        },
        Commodity {
            name: CommodityName::Oil,
            base_price: 65.,
            prices: DQueue::from([65.]),
            unit: Unit::Barrel,
            volatility: 5.,
            economy_factor: 0.09,
            storage_cost: 0.8,
        },
        Commodity {
            name: CommodityName::Silicon,
            base_price: 6000.,
            prices: DQueue::from([6000.]),
            unit: Unit::MetricTon,
            volatility: 6.5,
            economy_factor: 0.07,
            storage_cost: 0.25,
        },
        Commodity {
            name: CommodityName::Silver,
            base_price: 1030.,
            prices: DQueue::from([1030.]),
            unit: Unit::Kilogram,
            volatility: 0.5,
            economy_factor: -0.02,
            storage_cost: 0.03,
        },
        Commodity {
            name: CommodityName::Wheat,
            base_price: 201.,
            prices: DQueue::from([201.]),
            unit: Unit::MetricTon,
            volatility: 2.3,
            economy_factor: -0.05,
            storage_cost: 0.3,
        },
    ]
}
