use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::instruments::commodities::CommodityName;
use crate::core::instruments::forex::CurrencyName;
use crate::core::politics::{Culture, Governance, Ideology, Orientation, Politics};

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
            CountryName::Australia => "\
                Australia is a developed, resource-rich economy with strong institutions, a \
                stable political environment, and a high standard of living. It is one of the \
                world's largest exporters of commodities like iron ore and gold, making its \
                economy closely tied to global demand—particularly from Asia. The country has \
                 a well-regulated financial system, a flexible labor market, and consistent \
                 economic growth, supported by services, mining, and agriculture.",
            CountryName::Brazil => "\
                Brazil is the largest economy in Latin America and an influential emerging market, \
                rich in natural resources and agricultural products such as coffee and cacao. Its \
                economy is diverse, with major sectors including agriculture, mining, energy, \
                manufacturing, and services. Brazil benefits from strong export ties, especially \
                to China, but faces challenges such as political instability, inflation, and \
                structural inefficiencies.",
            CountryName::Canada => "\
                Canada is a developed, high-income economy with strong institutions, abundant \
                natural resources, and a well-regulated financial system. It is one of the world's \
                leading exporters of oil, natural gas, and minerals, making its economy closely \
                tied to global commodity prices. Key sectors include energy, manufacturing, \
                services, and technology, with significant trade integration with the United States.",
            CountryName::China => "\
                China is the world's second-largest economy and a global manufacturing and export \
                powerhouse. It has undergone rapid industrialization and urbanization over the past \
                few decades, shifting from an investment-driven model toward greater domestic \
                consumption and innovation. China is also the largest trading partner for many \
                countries, particularly in Asia and Africa.",
            CountryName::EU => "\
                The European Union (EU) is a major economic bloc comprising 27 member states, with \
                a highly developed, integrated economy and a shared single market. It is one of \
                the world's largest trading entities, driven by diverse industries including \
                manufacturing, services, agriculture, and technology. The EU benefits from strong \
                regulatory frameworks, advanced infrastructure, and a skilled labor force.",
            CountryName::Japan => "\
                Japan is a highly developed and technologically advanced economy, ranking among \
                the largest in the world. It has a strong industrial base, particularly in \
                automobiles, electronics, robotics, and precision manufacturing. The country has \
                a high standard of living, strong infrastructure, and a well-educated workforce. \
                Japan relies heavily on exports and imports most of its energy and raw materials.",
            CountryName::Russia => "\
                Russia is a major emerging market and one of the world's largest producers of oil, \
                natural gas, and minerals, making its economy heavily reliant on energy exports. \
                It has a strong industrial base, particularly in defense, heavy machinery, and \
                metallurgy. While resource wealth supports its trade balance, the economy faces \
                challenges from international sanctions, limited diversification, and political \
                risk.",
            CountryName::SaudiArabia => "\
                Saudi Arabia is a resource-rich economy and the world's largest exporter of crude \
                oil, with petroleum accounting for the majority of government revenue and exports. \
                It has a centrally managed economy with significant state involvement, though \
                recent reforms under the Vision 2030 initiative aim to diversify sectors like \
                tourism, finance, and technology. The kingdom uses its sovereign wealth fund to \
                invest domestically and abroad.",
            CountryName::SouthAfrica => "\
                South Africa is the most industrialized economy in Africa, with key sectors \
                including mining, manufacturing, agriculture, and financial services. It is a \
                major global producer of gold, platinum, and other minerals, making the economy \
                sensitive to commodity prices. The country has a well-developed financial system \
                and infrastructure but faces structural challenges such as high unemployment, \
                inequality, and energy supply issues.",
            CountryName::Ukraine => "\
                Ukraine is an emerging market economy with strengths in agriculture, heavy \
                industry, and IT services. It is one of the world’s leading exporters of grain, \
                particularly wheat and corn, and has significant reserves of minerals and energy \
                resources. The economy has faced major disruptions due to ongoing conflict, \
                infrastructure damage, and reliance on foreign aid and international financial \
                support.",
            CountryName::USA => "\
                The United States has the world’s largest economy, characterized by a highly \
                diversified and innovation-driven structure. Key sectors include technology, \
                finance, healthcare, energy, and manufacturing. It has a strong consumer base, \
                deep capital markets, and global leadership in research and development. The \
                U.S. dollar serves as the dominant global reserve currency, and the Federal \
                Reserve influences global monetary conditions through its policy decisions.",
            CountryName::Venezuela => "\
                Venezuela is a resource-rich country with one of the largest proven oil reserves \
                in the world, making its economy heavily dependent on petroleum exports. Years \
                of hyperinflation, economic mismanagement, and political instability have severely \
                weakened its economy, leading to widespread poverty, emigration, and a shift toward \
                dollarization in everyday transactions.",
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

    /// Gross Domestic Product (GDP) in trillion euros
    pub gdp: f32,

    /// Political stance of this country
    pub politics: Politics,

    /// Commodities produced, with a dependency factor on the local currency
    pub production: HashMap<CommodityName, f32>,
}

pub fn start_countries() -> Vec<Country> {
    vec![
        Country {
            name: CountryName::Australia,
            currency: CurrencyName::AUD,
            market: MarketKind::DevelopedMarket,
            gdp: 1.496,
            politics: Politics {
                governance: Governance::Democracy,
                ideology: Ideology::Neutral,
                culture: Culture::Progressive,
                orientation: Orientation::Capitalism,
            },
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
            gdp: 1.864,
            politics: Politics {
                governance: Governance::SemiDemocracy,
                ideology: Ideology::Neutral,
                culture: Culture::Moderate,
                orientation: Orientation::Capitalism,
            },
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
            gdp: 1.918,
            politics: Politics {
                governance: Governance::Democracy,
                ideology: Ideology::Neutral,
                culture: Culture::Progressive,
                orientation: Orientation::Capitalism,
            },
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
            gdp: 16.07,
            politics: Politics {
                governance: Governance::Autocracy,
                ideology: Ideology::Left,
                culture: Culture::Conservative,
                orientation: Orientation::Socialism,
            },
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
            gdp: 16.61,
            politics: Politics {
                governance: Governance::Democracy,
                ideology: Ideology::Neutral,
                culture: Culture::Progressive,
                orientation: Orientation::Capitalism,
            },
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
            gdp: 3.43,
            politics: Politics {
                governance: Governance::Democracy,
                ideology: Ideology::Right,
                culture: Culture::Conservative,
                orientation: Orientation::Capitalism,
            },
            production: HashMap::from([(CommodityName::Wheat, 0.2)]),
        },
        Country {
            name: CountryName::Russia,
            currency: CurrencyName::RUB,
            market: MarketKind::RestrictedMarket,
            gdp: 1.859,
            politics: Politics {
                governance: Governance::Autocracy,
                ideology: Ideology::Left,
                culture: Culture::Conservative,
                orientation: Orientation::Socialism,
            },
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
            gdp: 1.058,
            politics: Politics {
                governance: Governance::Autocracy,
                ideology: Ideology::Right,
                culture: Culture::Conservative,
                orientation: Orientation::Capitalism,
            },
            production: HashMap::from([(CommodityName::Oil, 0.8), (CommodityName::LNG, 0.5)]),
        },
        Country {
            name: CountryName::SouthAfrica,
            currency: CurrencyName::ZAR,
            market: MarketKind::EmergingMarket,
            gdp: 0.351,
            politics: Politics {
                governance: Governance::Democracy,
                ideology: Ideology::Left,
                culture: Culture::Moderate,
                orientation: Orientation::Mixed,
            },
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
            gdp: 0.2,
            politics: Politics {
                governance: Governance::Democracy,
                ideology: Ideology::Neutral,
                culture: Culture::Moderate,
                orientation: Orientation::Mixed,
            },
            production: HashMap::from([(CommodityName::Wheat, 0.5), (CommodityName::Corn, 0.4)]),
        },
        Country {
            name: CountryName::USA,
            currency: CurrencyName::USD,
            market: MarketKind::DevelopedMarket,
            gdp: 26.09,
            politics: Politics {
                governance: Governance::Democracy,
                ideology: Ideology::Right,
                culture: Culture::Moderate,
                orientation: Orientation::Capitalism,
            },
            production: HashMap::from([
                (CommodityName::Oil, 0.3),
                (CommodityName::Corn, 0.3),
                (CommodityName::Ethanol, 0.25),
                (CommodityName::LNG, 0.25),
                (CommodityName::Wheat, 0.2),
                (CommodityName::Silver, 0.15),
            ]),
        },
        Country {
            name: CountryName::Venezuela,
            currency: CurrencyName::VES,
            market: MarketKind::RestrictedMarket,
            gdp: 0.098,
            politics: Politics {
                governance: Governance::Autocracy,
                ideology: Ideology::Left,
                culture: Culture::Moderate,
                orientation: Orientation::Socialism,
            },
            production: HashMap::from([
                (CommodityName::Oil, 0.6),
                (CommodityName::Coffee, 0.5),
                (CommodityName::Cocoa, 0.3),
                (CommodityName::Gold, 0.25),
            ]),
        },
    ]
}
