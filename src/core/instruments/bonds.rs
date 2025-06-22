use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::countries::CountryName;
use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::core::instruments::stocks::Company;
use crate::core::loans::Term;
use crate::utils::NameFromEnum;

#[derive(EnumIter, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum BondKind {
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
    pub fn description(&self) -> &str {
        match self {
            BondQuality::B | BondQuality::CCC | BondQuality::CC | BondQuality::C => {
                "High yield (junk) bond. Offers higher interest due to the increased risk of default."
            },
            _ => "Investment grade bond. Low risk of default.",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
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
        match self {
            BondIssuer::Government(country) => country.to_lowername(),
            BondIssuer::Corporate(company) => company.to_lowername(),
        }
    }

    pub fn description(&self) -> &str {
        match self {
            BondIssuer::Government(country) => country.description(),
            BondIssuer::Corporate(company) => company.description(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Bond {
    /// The issuer of the bond
    pub issuer: BondIssuer,

    /// The face value of the bond
    pub prices: Vec<f32>,

    /// The quality of the bond (investment grade or high yield)
    pub quality: BondQuality,

    /// Interest rate on the bond
    pub interest: f32,

    /// Number of years until the bond matures
    pub term: Term,
}

impl Bond {
    pub fn kind(&self) -> BondKind {
        match self.issuer {
            BondIssuer::Government(_) => BondKind::Government,
            BondIssuer::Corporate(_) => BondKind::Corporate,
        }
    }

    /// Issue a new bond, recalculating interest and face value
    pub fn issue(&mut self) {
        self.prices
            .push(self.prices.last().unwrap() * (1.0 + self.interest / 100.));
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

    fn all(&self) -> &Vec<f32> {
        &self.prices
    }

    fn current(&self) -> f32 {
        *self.prices.last().unwrap()
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
            prices: vec![10000.],
            interest: 4.8,
            term: Term::FiveYears,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::Brazil),
            quality: BondQuality::BBB,
            prices: vec![10000.],
            interest: 6.5,
            term: Term::FiveYears,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::Canada),
            quality: BondQuality::AAA,
            prices: vec![10000.],
            interest: 4.2,
            term: Term::FiveYears,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::China),
            quality: BondQuality::A,
            prices: vec![10000.],
            interest: 3.5,
            term: Term::FiveYears,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::EU),
            quality: BondQuality::AAA,
            prices: vec![10000.],
            interest: 3.0,
            term: Term::FiveYears,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::Japan),
            quality: BondQuality::AAA,
            prices: vec![10000.],
            interest: 0.5,
            term: Term::FiveYears,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::Russia),
            quality: BondQuality::CCC,
            prices: vec![10000.],
            interest: 7.5,
            term: Term::FiveYears,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::SaudiArabia),
            quality: BondQuality::BBB,
            prices: vec![10000.],
            interest: 3.8,
            term: Term::FiveYears,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::SouthAfrica),
            quality: BondQuality::CC,
            prices: vec![10000.],
            interest: 8.0,
            term: Term::FiveYears,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::Ukraine),
            quality: BondQuality::CC,
            prices: vec![10000.],
            interest: 10.0,
            term: Term::FiveYears,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::USA),
            quality: BondQuality::AAA,
            prices: vec![10000.],
            interest: 4.,
            term: Term::ThreeYears,
        },
        Bond {
            issuer: BondIssuer::Government(CountryName::Venezuela),
            quality: BondQuality::C,
            prices: vec![10000.],
            interest: 12.0,
            term: Term::FiveYears,
        },
    ]
}
