use std::fmt::Display;

use crate::core::constants::CURRENCY;
use crate::core::countries::{Country, CountryName};
use crate::core::instruments::commodities::Commodity;
use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::utils::{DQueue, NameFromEnum};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CurrencyName {
    AUD,
    BRL,
    CAD,
    CNY,
    EUR,
    JPY,
    RUB,
    SAR,
    UAH,
    USD,
    VES,
    ZAR,
}

impl CurrencyName {
    pub fn symbol(&self) -> &str {
        match self {
            CurrencyName::AUD => "A$",
            CurrencyName::BRL => "R$",
            CurrencyName::CAD => "C$",
            CurrencyName::CNY => "¥",
            CurrencyName::EUR => "€",
            CurrencyName::JPY => "¥",
            CurrencyName::RUB => "₽",
            CurrencyName::SAR => "﷼",
            CurrencyName::UAH => "₴",
            CurrencyName::USD => "$",
            CurrencyName::VES => "Bs",
            CurrencyName::ZAR => "R",
        }
    }

    pub fn fullname(&self) -> &str {
        match self {
            CurrencyName::AUD => "Australian Dollar",
            CurrencyName::BRL => "Brazilian Real",
            CurrencyName::CAD => "Canadian Dollar",
            CurrencyName::CNY => "Chinese Yuan",
            CurrencyName::EUR => "European Euro",
            CurrencyName::JPY => "Japanese Yen",
            CurrencyName::RUB => "Russian Ruble",
            CurrencyName::SAR => "Saudi Arabian Riyal",
            CurrencyName::UAH => "Ukrainian Hryvnia",
            CurrencyName::USD => "United States Dollar",
            CurrencyName::VES => "Venezuelan Bolivar",
            CurrencyName::ZAR => "South African Rand",
        }
    }
}

impl Display for CurrencyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Currency {
    /// The currency's name
    pub name: CurrencyName,

    /// The country that uses this currency
    pub country: CountryName,

    /// Default value of the currency per euro
    pub base_value: f32,

    /// The values over time per euro
    pub values: DQueue<f32>,
}

impl Currency {
    pub fn bump(&mut self, countries: &Vec<Country>, commodities: &Vec<Commodity>) -> f32 {
        if self.name == CURRENCY {
            return self.current(); // The base currency doesn't change
        }

        let country = countries.iter().find(|c| c.name == self.country).unwrap();
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
                    .sum::<f32>()
                / 100.);

        // Adjust value to tend towards the base value
        let deviation = (new_value - self.base_value) / self.base_value;
        new_value *= 1. + -deviation * deviation.abs() / 2.5;

        new_value = new_value.max(0.001);

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

    fn fullname(&self) -> String {
        self.name.fullname().to_string()
    }

    fn description(&self) -> &str {
        self.country.description()
    }

    fn kind(&self) -> InstrumentKind {
        InstrumentKind::Forex(self.name)
    }

    fn all(&self) -> &DQueue<f32> {
        &self.values
    }

    fn symbol(&self) -> &str {
        self.name.symbol()
    }
}

pub fn start_currencies() -> Vec<Currency> {
    vec![
        Currency {
            name: CurrencyName::AUD,
            country: CountryName::Australia,
            base_value: 0.56,
            values: DQueue::from([0.56]),
        },
        Currency {
            name: CurrencyName::BRL,
            country: CountryName::Brazil,
            base_value: 0.16,
            values: DQueue::from([0.16]),
        },
        Currency {
            name: CurrencyName::CAD,
            country: CountryName::Canada,
            base_value: 0.6,
            values: DQueue::from([0.6]),
        },
        Currency {
            name: CurrencyName::CNY,
            country: CountryName::China,
            base_value: 0.12,
            values: DQueue::from([0.12]),
        },
        Currency {
            name: CurrencyName::EUR,
            country: CountryName::EU,
            base_value: 1.0,
            values: DQueue::from([1.0]),
        },
        Currency {
            name: CurrencyName::JPY,
            country: CountryName::Japan,
            base_value: 0.006,
            values: DQueue::from([0.006]),
        },
        Currency {
            name: CurrencyName::RUB,
            country: CountryName::Russia,
            base_value: 0.01,
            values: DQueue::from([0.01]),
        },
        Currency {
            name: CurrencyName::SAR,
            country: CountryName::SaudiArabia,
            base_value: 0.23,
            values: DQueue::from([0.23]),
        },
        Currency {
            name: CurrencyName::UAH,
            country: CountryName::Ukraine,
            base_value: 0.02,
            values: DQueue::from([0.02]),
        },
        Currency {
            name: CurrencyName::USD,
            country: CountryName::USA,
            base_value: 0.85,
            values: DQueue::from([0.85]),
        },
        Currency {
            name: CurrencyName::VES,
            country: CountryName::Venezuela,
            base_value: 0.008,
            values: DQueue::from([0.008]),
        },
        Currency {
            name: CurrencyName::ZAR,
            country: CountryName::SouthAfrica,
            base_value: 0.05,
            values: DQueue::from([0.05]),
        },
    ]
}
