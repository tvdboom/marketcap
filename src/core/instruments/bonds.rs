use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::countries::CountryName;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::forex::Currency;
use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::core::instruments::stocks::{Company, Stock};
use crate::utils::{DQueue, NameFromEnum};

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum BondKind {
    #[default]
    Government,
    Corporate,
}

impl BondKind {
    pub fn emoji(&self) -> &str {
        match self {
            BondKind::Government => "💼",
            BondKind::Corporate => "🏢",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum BondQuality {
    AAA,
    AA,
    A,
    BBB,
    BB,
    B,
    CCC,
    CC,
    C,
}

impl BondQuality {
    pub fn is_high_yield(&self) -> bool {
        matches!(
            self,
            BondQuality::B | BondQuality::CCC | BondQuality::CC | BondQuality::C
        )
    }

    pub fn description(&self) -> &str {
        if self.is_high_yield() {
            "High yield (junk) bond. Offers higher interest due to the increased risk of default."
        } else {
            "Investment grade bond. Low risk of default."
        }
    }

    pub fn value(&self) -> f32 {
        match self {
            BondQuality::AAA => 0.2,
            BondQuality::AA => 0.3,
            BondQuality::A => 0.4,
            BondQuality::BBB => 0.5,
            BondQuality::BB => 0.6,
            BondQuality::B => 0.7,
            BondQuality::CCC => 0.8,
            BondQuality::CC => 0.9,
            BondQuality::C => 1.0,
        }
    }

    pub fn default_chance(&self) -> f32 {
        1. - (-0.3 * self.value()).exp()
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum BondIssuer {
    Government(CountryName),
    Corporate(Company),
}

impl BondIssuer {
    pub fn to_name(&self) -> String {
        match self {
            BondIssuer::Government(country) => country.to_name(),
            BondIssuer::Corporate(company) => company.to_name(),
        }
    }

    pub fn to_lowername(&self) -> String {
        format!(
            "{} bond",
            match self {
                BondIssuer::Government(country) => country.to_lowername(),
                BondIssuer::Corporate(company) => company.to_lowername(),
            }
        )
    }

    pub fn description(&self) -> &str {
        match self {
            BondIssuer::Government(country) => country.description(),
            BondIssuer::Corporate(company) => company.description(),
        }
    }

    pub fn coupon_payment(&self, interest: f32, cds: bool, economy: &GlobalEconomy) -> f32 {
        // Multiply by 0.5 since coupon is paid out twice a year
        match self {
            BondIssuer::Government(country) => {
                let currency = economy
                    .currencies
                    .iter()
                    .find(|c| c.country == *country)
                    .unwrap();

                Bond::FACE_VALUE_GOVERNMENT * 0.5 * interest / 100. * currency.current()
            },
            BondIssuer::Corporate(name) => {
                let stock = economy.stocks.iter().find(|s| s.issuer == *name).unwrap();

                // Very bad-performing stocks default on coupon payment
                if !cds && stock.current() < stock.base_price * 0.75 {
                    0.
                } else {
                    Bond::FACE_VALUE_CORPORATE * 0.5 * interest / 100.
                }
            },
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Bond {
    /// The issuer of the bond
    pub issuer: BondIssuer,

    /// The face value of the bond
    pub prices: DQueue<f32>,

    /// The quality of the bond (investment grade or high yield)
    pub quality: BondQuality,

    /// Interest rate on the bond
    pub interest: f32,
}

impl Bond {
    pub const FACE_VALUE_GOVERNMENT: f32 = 10000.;
    pub const FACE_VALUE_CORPORATE: f32 = 1000.;

    pub fn kind(&self) -> BondKind {
        match self.issuer {
            BondIssuer::Government(_) => BondKind::Government,
            BondIssuer::Corporate(_) => BondKind::Corporate,
        }
    }

    pub fn bump(&mut self, currencies: &Vec<Currency>) {
        self.prices.push(match self.issuer {
            BondIssuer::Government(country) => {
                let currency = currencies.iter().find(|c| c.country == country).unwrap();
                Self::FACE_VALUE_GOVERNMENT * currency.current()
            },
            BondIssuer::Corporate(_) => Self::FACE_VALUE_CORPORATE,
        });
    }

    /// Issue a new bond = recalculating interest
    pub fn issue(&mut self, interest: f32, stocks: &Vec<Stock>, currencies: &Vec<Currency>) {
        self.interest = match self.issuer {
            BondIssuer::Government(country) => {
                let currency = currencies.iter().find(|c| c.country == country).unwrap();
                interest
                    * (1.
                        + self.quality.value()
                        + (currency.base_value - currency.current()) / currency.base_value)
            },
            BondIssuer::Corporate(name) => {
                let stock = stocks.iter().find(|s| s.issuer == name).unwrap();
                interest
                    * (1.
                        + self.quality.value()
                        + (stock.base_price - stock.current()) / stock.base_price)
            },
        };
    }
}

impl Instrument for Bond {
    fn name(&self) -> String {
        self.issuer.to_name()
    }

    fn lowername(&self) -> String {
        self.issuer.to_lowername()
    }

    fn image(&self) -> String {
        match self.issuer {
            BondIssuer::Government(country) => country.to_lowername(),
            BondIssuer::Corporate(company) => format!("{}-bond", company.to_lowername()),
        }
    }

    fn description(&self) -> &str {
        self.issuer.description()
    }

    fn kind(&self) -> InstrumentKind {
        InstrumentKind::Bond(self.issuer)
    }

    fn all(&self) -> &DQueue<f32> {
        &self.prices
    }

    fn interest(&self) -> f32 {
        self.interest
    }

    fn quality(&self) -> BondQuality {
        self.quality.clone()
    }
}

pub fn start_bonds() -> Vec<Bond> {
    vec![
        Bond {
            issuer: BondIssuer::Government(CountryName::Australia),
            quality: BondQuality::AA,
            prices: DQueue::from([Bond::FACE_VALUE_GOVERNMENT]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::Brazil),
            quality: BondQuality::BBB,
            prices: DQueue::from([Bond::FACE_VALUE_GOVERNMENT]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::Canada),
            quality: BondQuality::AAA,
            prices: DQueue::from([Bond::FACE_VALUE_GOVERNMENT]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::China),
            quality: BondQuality::A,
            prices: DQueue::from([Bond::FACE_VALUE_GOVERNMENT]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::EU),
            quality: BondQuality::AAA,
            prices: DQueue::from([Bond::FACE_VALUE_GOVERNMENT]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::Japan),
            quality: BondQuality::AAA,
            prices: DQueue::from([Bond::FACE_VALUE_GOVERNMENT]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::Russia),
            quality: BondQuality::CCC,
            prices: DQueue::from([Bond::FACE_VALUE_GOVERNMENT]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::SaudiArabia),
            quality: BondQuality::BBB,
            prices: DQueue::from([Bond::FACE_VALUE_GOVERNMENT]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::SouthAfrica),
            quality: BondQuality::CC,
            prices: DQueue::from([Bond::FACE_VALUE_GOVERNMENT]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::Ukraine),
            quality: BondQuality::CC,
            prices: DQueue::from([Bond::FACE_VALUE_GOVERNMENT]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::USA),
            quality: BondQuality::AAA,
            prices: DQueue::from([Bond::FACE_VALUE_GOVERNMENT]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::Venezuela),
            quality: BondQuality::C,
            prices: DQueue::from([Bond::FACE_VALUE_GOVERNMENT]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Corporate(Company::Apple),
            quality: BondQuality::AAA,
            prices: DQueue::from([Bond::FACE_VALUE_CORPORATE]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Corporate(Company::Boeing),
            quality: BondQuality::A,
            prices: DQueue::from([Bond::FACE_VALUE_CORPORATE]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Corporate(Company::GoldmanSachs),
            quality: BondQuality::AA,
            prices: DQueue::from([Bond::FACE_VALUE_CORPORATE]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Corporate(Company::Inditex),
            quality: BondQuality::A,
            prices: DQueue::from([Bond::FACE_VALUE_CORPORATE]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Corporate(Company::LockheedMartin),
            quality: BondQuality::B,
            prices: DQueue::from([Bond::FACE_VALUE_CORPORATE]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Corporate(Company::LVMH),
            quality: BondQuality::A,
            prices: DQueue::from([Bond::FACE_VALUE_CORPORATE]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Corporate(Company::Maersk),
            quality: BondQuality::BBB,
            prices: DQueue::from([Bond::FACE_VALUE_CORPORATE]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Corporate(Company::Moderna),
            quality: BondQuality::BB,
            prices: DQueue::from([Bond::FACE_VALUE_CORPORATE]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Corporate(Company::Nestle),
            quality: BondQuality::AAA,
            prices: DQueue::from([Bond::FACE_VALUE_CORPORATE]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Corporate(Company::Nvidia),
            quality: BondQuality::B,
            prices: DQueue::from([Bond::FACE_VALUE_CORPORATE]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Corporate(Company::Pfizer),
            quality: BondQuality::A,
            prices: DQueue::from([Bond::FACE_VALUE_CORPORATE]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Corporate(Company::RioTinto),
            quality: BondQuality::BB,
            prices: DQueue::from([Bond::FACE_VALUE_CORPORATE]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Corporate(Company::Shell),
            quality: BondQuality::AAA,
            prices: DQueue::from([Bond::FACE_VALUE_CORPORATE]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Corporate(Company::Toyota),
            quality: BondQuality::AAA,
            prices: DQueue::from([Bond::FACE_VALUE_CORPORATE]),
            interest: 0.,
        },
        Bond {
            issuer: BondIssuer::Corporate(Company::Unilever),
            quality: BondQuality::A,
            prices: DQueue::from([Bond::FACE_VALUE_CORPORATE]),
            interest: 0.,
        },
    ]
}
