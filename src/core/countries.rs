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

#[derive(EnumIter, Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum CountryName {
    #[default]
    Australia,
    Brazil,
    Canada,
    China,
    EU,
    Japan,
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
            CountryName::China => {
                "China is an emerging market with rapid industrialization, significant production \
                of metals and commodities."
            },
            CountryName::EU => {
                "The European Union is a developed market with diverse economies, strong \
                regulations, and a common currency (Euro)."
            },
            CountryName::Japan => {
                "Japan is a developed market known for its advanced technology and manufacturing \
                sectors."
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
            currency: CurrencyName::AUD,
            market: MarketKind::DevelopedMarket,
            production: HashMap::from([
                (CommodityName::Cotton, 0.5),
                (CommodityName::Gold, 0.45),
                (CommodityName::Oil, 0.4),
                (CommodityName::Silver, 0.4),
                (CommodityName::Copper, 0.35),
                (CommodityName::Iron, 0.3),
            ]),
        },
        Country {
            name: CountryName::Brazil,
            currency: CurrencyName::BRL,
            market: MarketKind::EmergingMarket,
            production: HashMap::from([
                (CommodityName::Coffee, 0.4),
                (CommodityName::Cocoa, 0.4),
                (CommodityName::Corn, 0.3),
                (CommodityName::Silicon, 0.3),
                (CommodityName::Iron, 0.2),
                (CommodityName::Ethanol, 0.1),
            ]),
        },
        Country {
            name: CountryName::Canada,
            currency: CurrencyName::CAD,
            market: MarketKind::DevelopedMarket,
            production: HashMap::from([
                (CommodityName::Oil, 0.5),
                (CommodityName::LNG, 0.4),
                (CommodityName::Gold, 0.3),
                (CommodityName::Aluminium, 0.4),
                (CommodityName::Ethanol, 0.2),
                (CommodityName::Wheat, 0.15),
            ]),
        },
        Country {
            name: CountryName::China,
            currency: CurrencyName::CNY,
            market: MarketKind::EmergingMarket,
            production: HashMap::from([
                (CommodityName::Iron, 0.5),
                (CommodityName::Gold, 0.35),
                (CommodityName::Cotton, 0.35),
                (CommodityName::Aluminium, 0.35),
                (CommodityName::Silver, 0.3),
                (CommodityName::Copper, 0.25),
            ]),
        },
        Country {
            name: CountryName::EU,
            currency: CurrencyName::EUR,
            market: MarketKind::DevelopedMarket,
            production: HashMap::from([
                (CommodityName::Ethanol, 0.3),
                (CommodityName::Silicon, 0.25),
                (CommodityName::LNG, 0.2),
                (CommodityName::Aluminium, 0.2),
                (CommodityName::Wheat, 0.2),
            ]),
        },
        Country {
            name: CountryName::Japan,
            currency: CurrencyName::JPY,
            market: MarketKind::DevelopedMarket,
            production: HashMap::from([(CommodityName::Wheat, 0.2)]),
        },
        Country {
            name: CountryName::Russia,
            currency: CurrencyName::RUB,
            market: MarketKind::RestrictedMarket,
            production: HashMap::from([
                (CommodityName::Oil, 0.5),
                (CommodityName::LNG, 0.4),
                (CommodityName::Gold, 0.3),
                (CommodityName::Aluminium, 0.3),
                (CommodityName::Silver, 0.3),
                (CommodityName::Copper, 0.2),
            ]),
        },
        Country {
            name: CountryName::SaudiArabia,
            currency: CurrencyName::SAR,
            market: MarketKind::EmergingMarket,
            production: HashMap::from([(CommodityName::Oil, 0.8), (CommodityName::LNG, 0.5)]),
        },
        Country {
            name: CountryName::SouthAfrica,
            currency: CurrencyName::ZAR,
            market: MarketKind::EmergingMarket,
            production: HashMap::from([
                (CommodityName::Gold, 0.5),
                (CommodityName::Iron, 0.25),
                (CommodityName::Ethanol, 0.2),
                (CommodityName::Silicon, 0.15),
                (CommodityName::Corn, 0.15),
            ]),
        },
        Country {
            name: CountryName::Ukraine,
            currency: CurrencyName::UAH,
            market: MarketKind::EmergingMarket,
            production: HashMap::from([(CommodityName::Wheat, 0.5), (CommodityName::Corn, 0.4)]),
        },
        Country {
            name: CountryName::USA,
            currency: CurrencyName::USD,
            market: MarketKind::DevelopedMarket,
            production: HashMap::from([
                (CommodityName::Oil, 0.3),
                (CommodityName::Ethanol, 0.3),
                (CommodityName::LNG, 0.3),
                (CommodityName::Corn, 0.3),
                (CommodityName::Wheat, 0.3),
                (CommodityName::Silver, 0.2),
            ]),
        },
        Country {
            name: CountryName::Venezuela,
            currency: CurrencyName::VES,
            market: MarketKind::RestrictedMarket,
            production: HashMap::from([
                (CommodityName::Oil, 0.6),
                (CommodityName::Coffee, 0.5),
                (CommodityName::Cocoa, 0.3),
                (CommodityName::Gold, 0.25),
            ]),
        },
    ]
}
