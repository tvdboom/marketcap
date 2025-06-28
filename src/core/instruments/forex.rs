use crate::core::countries::Country;
use crate::core::instruments::commodities::Commodity;
use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::utils::NameFromEnum;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

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

impl Currency {
    pub fn bump(&mut self, countries: &Vec<Country>, commodities: &Vec<Commodity>) -> f32 {
        let country = countries.iter().find(|c| c.currency == self.name).unwrap();
        let mut new_value = self.current()
            + (1.
                * country
                    .production
                    .iter()
                    .map(|(n, w)| {
                        commodities
                            .iter()
                            .find(|c| c.name == *n)
                            .map_or(0., |c| w * (c.current() - c.base_price) / c.base_price)
                    })
                    .sum::<f32>());

        // Adjust value to tend towards the base value
        // At 100% deviation, there's a 20% adjustment towards the base price
        // At 50% deviation, there's a 5% adjustment towards the base price
        let deviation = (new_value - self.base_value) / self.base_value;
        new_value *= 1. + -deviation * deviation.abs() / 5.;

        new_value = new_value.max(0.);

        self.values.push(new_value);
        new_value
    }
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

pub fn start_currencies() -> Vec<Currency> {
    vec![
        Currency {
            name: CurrencyName::AustralianDollar,
            base_value: 0.56,
            values: vec![0.56],
        },
        Currency {
            name: CurrencyName::Real,
            base_value: 0.16,
            values: vec![0.16],
        },
        Currency {
            name: CurrencyName::CanadianDollar,
            base_value: 0.6,
            values: vec![0.6],
        },
        Currency {
            name: CurrencyName::Euro,
            base_value: 1.0,
            values: vec![1.0],
        },
        Currency {
            name: CurrencyName::Yen,
            base_value: 0.006,
            values: vec![0.006],
        },
        Currency {
            name: CurrencyName::Yuan,
            base_value: 0.12,
            values: vec![0.12],
        },
        Currency {
            name: CurrencyName::Ruble,
            base_value: 0.01,
            values: vec![0.01],
        },
        Currency {
            name: CurrencyName::Riyal,
            base_value: 0.23,
            values: vec![0.23],
        },
        Currency {
            name: CurrencyName::Rand,
            base_value: 0.05,
            values: vec![0.05],
        },
        Currency {
            name: CurrencyName::Hryvnia,
            base_value: 0.02,
            values: vec![0.02],
        },
        Currency {
            name: CurrencyName::UnitedStatesDollar,
            base_value: 0.85,
            values: vec![0.85],
        },
        Currency {
            name: CurrencyName::Bolivar,
            base_value: 0.008,
            values: vec![0.008],
        },
    ]
}
