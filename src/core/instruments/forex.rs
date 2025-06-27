use serde::{Deserialize, Serialize};
use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use strum_macros::EnumIter;
use crate::utils::NameFromEnum;

#[derive(EnumIter, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CurrencyName {
    AustralianDollar,
    Real,
    CanadianDollar,
    Euro,
    Yen,
    Yuan,
    Ruble,
    Riyal,
    Rand,
    Hryvnia,
    UnitedStatesDollar,
    Bolivar,
}

impl CurrencyName {
    pub fn symbol(&self) -> &str {
        match self {
            CurrencyName::AustralianDollar => "A$",
            CurrencyName::Real => "R$",
            CurrencyName::CanadianDollar => "C$",
            CurrencyName::Euro => "€",
            CurrencyName::Yen => "¥",
            CurrencyName::Yuan => "¥",
            CurrencyName::Ruble => "₽",
            CurrencyName::Riyal => "﷼",
            CurrencyName::Rand => "R",
            CurrencyName::Hryvnia => "₴",
            CurrencyName::UnitedStatesDollar => "$",
            CurrencyName::Bolivar => "Bs",
        }
    }
    
    pub fn acronym(&self) -> &str {
        match self {
            CurrencyName::AustralianDollar => "AUD",
            CurrencyName::Real => "BRL",
            CurrencyName::CanadianDollar => "CAD",
            CurrencyName::Euro => "EUR",
            CurrencyName::Yen => "JPY",
            CurrencyName::Yuan => "CNY",
            CurrencyName::Ruble => "RUB",
            CurrencyName::Riyal => "SAR",
            CurrencyName::Rand => "ZAR",
            CurrencyName::Hryvnia => "UAH",
            CurrencyName::UnitedStatesDollar => "USD",
            CurrencyName::Bolivar => "VES",
        }
    }
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Currency {
    /// The currency's name
    pub name: CurrencyName,

    /// Default value of the currency per euro
    pub base_value: f32,

    /// The values over time per euro
    pub values: Vec<f32>,
}

impl Instrument for Currency {
    fn name(&self) -> String {
        self.name.to_name()
    }

    fn lowername(&self) -> String {
        self.name.to_lowername()
    }

    fn description(&self) -> &str {
        ""
    }
    
    fn kind(&self) -> InstrumentKind {
        InstrumentKind::Forex(self.name)
    }
    
    fn all(&self) -> &Vec<f32> {
        &self.values
    }

    fn current(&self) -> f32 {
        *self.values.last().unwrap()
    }
}