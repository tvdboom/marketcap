use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::instruments::commodities::CommodityName;

#[derive(EnumIter, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Country {
    Australia,
    Brazil,
    Canada,
    EU,
    Japan,
    China,
    Russia,
    Ukraine,
    USA,
    Venezuela,
}

impl Country {
    pub fn currency(&self) -> &'static str {
        match self {
            Country::Australia => "AUD",
            Country::Brazil => "BRL",
            Country::Canada => "CAD",
            Country::EU => "EUR",
            Country::Japan => "JPY",
            Country::China => "CNY",
            Country::Russia => "RUB",
            Country::Ukraine => "UAH",
            Country::USA => "USD",
            Country::Venezuela => "VES",
        }
    }

    pub fn production(&self) -> Vec<CommodityName> {
        match self {
            Country::Australia => vec![CommodityName::Aluminium, CommodityName::Iron],
            Country::Brazil => vec![CommodityName::Cocoa, CommodityName::Wheat],
            Country::Canada => vec![CommodityName::Oil, CommodityName::Wheat],
            Country::EU => vec![CommodityName::Oil, CommodityName::Wheat],
            Country::Japan => vec![CommodityName::Oil],
            Country::China => vec![
                CommodityName::Copper,
                CommodityName::Iron,
                CommodityName::Gold,
            ],
            Country::Russia => vec![CommodityName::Oil, CommodityName::LNG],
            Country::Ukraine => vec![CommodityName::Wheat],
            Country::USA => vec![
                CommodityName::Oil,
                CommodityName::Wheat,
                CommodityName::LNG,
            ],
            Country::Venezuela => vec![CommodityName::Oil, CommodityName::Cocoa],
        }
    }
}
