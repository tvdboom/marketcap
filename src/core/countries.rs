use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::instruments::commodities::CommodityName;
use crate::core::instruments::forex::CurrencyName;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MarketKind {
    DevelopedMarket,
    EmergingMarket,
    RestrictedMarket,
}

impl MarketKind {
    pub fn description(&self) -> &str {
        match self {
            MarketKind::DevelopedMarket => {
                "Developed markets are advanced economies with stable political systems, \
                mature financial markets, high income levels, and well-established infrastructure. \
                They offer lower growth potential but are generally considered safer and more \
                stable for investment compared to emerging markets."
            },
            MarketKind::EmergingMarket => {
                "Emerging markets are economies in the process of rapid growth and \
                industrialization, typically characterized by improving infrastructure, \
                increasing foreign investment, and expanding middle-class populations. \
                They offer high growth potential but also come with greater economic and \
                political risks compared to developed markets."
            },
            MarketKind::RestrictedMarket => {
                "Restricted markets are economies with limited access to foreign investment due \
                to government controls, sanctions, or political instability. They often lack \
                transparency, have underdeveloped financial systems, and pose high risks, making \
                them less accessible or off-limits to many global investors."
            },
        }
    }
}

#[derive(EnumIter, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CountryName {
    Australia,
    Brazil,
    Canada,
    EU,
    Japan,
    China,
    Russia,
    SaudiArabia,
    SouthAfrica,
    Ukraine,
    USA,
    Venezuela,
}

impl CountryName {
    pub fn description(&self) -> &str {
        match self {
            CountryName::Australia => {
                "Australia is a developed market with a strong economy, rich in natural \
                resources like gold and iron."
            },
            CountryName::Brazil => {
                "Brazil is an emerging market known for its agricultural exports like coffee \
                and cocoa."
            },
            CountryName::Canada => {
                "Canada is a developed market with significant oil and gas reserves, as well \
                as a strong mining sector."
            },
            CountryName::EU => {
                "The European Union is a developed market with diverse economies, strong \
                regulations, and a common currency (Euro)."
            },
            CountryName::Japan => {
                "Japan is a developed market known for its advanced technology and manufacturing \
                sectors."
            },
            CountryName::China => {
                "China is an emerging market with rapid industrialization, significant production \
                of metals and commodities."
            },
            CountryName::Russia => {
                "Russia is a restricted market with vast natural resources, particularly in oil and gas."
            },
            CountryName::SaudiArabia => {
                "Saudi Arabia is an emerging market heavily reliant on oil exports."
            },
            CountryName::SouthAfrica => {
                "South Africa is an emerging market with rich mineral resources, including \
                gold and platinum."
            },
            CountryName::Ukraine => {
                "Ukraine is an emerging market known for its agricultural production, especially \
                wheat and corn."
            },
            CountryName::USA => {
                "The United States is a developed market with the largest economy \
                in the world, diverse industries, and significant global influence."
            },
            CountryName::Venezuela => {
                "Venezuela is a restricted market with large oil reserves, but faces economic \
                challenges and political instability."
            },
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Country {
    /// The name of the country
    pub name: CountryName,

    /// The local currency
    pub currency: CurrencyName,

    /// Type of market in terms of development
    pub market: MarketKind,

    /// Commodities produced, with a dependency factor on the local currency
    pub production: HashMap<CommodityName, f32>,
}

pub fn start_countries() -> Vec<Country> {
    vec![
        Country {
            name: CountryName::Australia,
            currency: CurrencyName::AustralianDollar,
            market: MarketKind::DevelopedMarket,
            production: HashMap::from([
                (CommodityName::Gold, 0.5),
                (CommodityName::Oil, 0.4),
                (CommodityName::Iron, 0.3),
                (CommodityName::Copper, 0.3),
            ]),
        },
        Country {
            name: CountryName::Brazil,
            currency: CurrencyName::Real,
            market: MarketKind::EmergingMarket,
            production: HashMap::from([
                (CommodityName::Coffee, 0.4),
                (CommodityName::Cocoa, 0.4),
                (CommodityName::Corn, 0.3),
                (CommodityName::Iron, 0.3),
                (CommodityName::Silicon, 0.3),
            ]),
        },
        Country {
            name: CountryName::Canada,
            currency: CurrencyName::CanadianDollar,
            market: MarketKind::DevelopedMarket,
            production: HashMap::from([
                (CommodityName::Oil, 0.5),
                (CommodityName::LNG, 0.4),
                (CommodityName::Gold, 0.3),
                (CommodityName::Aluminium, 0.3),
                (CommodityName::Wheat, 0.3),
            ]),
        },
        Country {
            name: CountryName::EU,
            currency: CurrencyName::Euro,
            market: MarketKind::DevelopedMarket,
            production: HashMap::from([
                (CommodityName::LNG, 0.2),
                (CommodityName::Silicon, 0.2),
                (CommodityName::Aluminium, 0.2),
                (CommodityName::Wheat, 0.2),
            ]),
        },
        Country {
            name: CountryName::Japan,
            currency: CurrencyName::Yen,
            market: MarketKind::DevelopedMarket,
            production: HashMap::from([(CommodityName::Wheat, 0.2)]),
        },
        Country {
            name: CountryName::China,
            currency: CurrencyName::Yuan,
            market: MarketKind::EmergingMarket,
            production: HashMap::from([
                (CommodityName::Iron, 0.5),
                (CommodityName::Gold, 0.4),
                (CommodityName::Aluminium, 0.3),
                (CommodityName::Copper, 0.3),
            ]),
        },
        Country {
            name: CountryName::Russia,
            currency: CurrencyName::Ruble,
            market: MarketKind::RestrictedMarket,
            production: HashMap::from([
                (CommodityName::Oil, 0.5),
                (CommodityName::LNG, 0.4),
                (CommodityName::Gold, 0.3),
                (CommodityName::Aluminium, 0.3),
                (CommodityName::Copper, 0.3),
            ]),
        },
        Country {
            name: CountryName::SaudiArabia,
            currency: CurrencyName::Riyal,
            market: MarketKind::EmergingMarket,
            production: HashMap::from([(CommodityName::Oil, 0.8), (CommodityName::LNG, 0.5)]),
        },
        Country {
            name: CountryName::SouthAfrica,
            currency: CurrencyName::Rand,
            market: MarketKind::EmergingMarket,
            production: HashMap::from([
                (CommodityName::Gold, 0.4),
                (CommodityName::Iron, 0.3),
                (CommodityName::Corn, 0.3),
                (CommodityName::Silicon, 0.3),
            ]),
        },
        Country {
            name: CountryName::Ukraine,
            currency: CurrencyName::Hryvnia,
            market: MarketKind::EmergingMarket,
            production: HashMap::from([(CommodityName::Wheat, 0.5), (CommodityName::Corn, 0.4)]),
        },
        Country {
            name: CountryName::USA,
            currency: CurrencyName::UnitedStatesDollar,
            market: MarketKind::DevelopedMarket,
            production: HashMap::from([
                (CommodityName::Oil, 0.5),
                (CommodityName::LNG, 0.4),
                (CommodityName::Corn, 0.3),
                (CommodityName::Wheat, 0.3),
            ]),
        },
        Country {
            name: CountryName::Venezuela,
            currency: CurrencyName::Bolivar,
            market: MarketKind::RestrictedMarket,
            production: HashMap::from([
                (CommodityName::Oil, 0.6),
                (CommodityName::Gold, 0.4),
                (CommodityName::Cocoa, 0.4),
                (CommodityName::Coffee, 0.4),
            ]),
        },
    ]
}
