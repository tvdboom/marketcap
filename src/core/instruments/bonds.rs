use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::instruments::Instrument;
use crate::core::loans::Term;
use crate::core::player::InstrumentKind;
use crate::utils::NameFromEnum;

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
    HighYield,
    InvestmentGrade,
}

impl BondQuality {
    pub fn description(&self) -> &str {
        match self {
            BondQuality::HighYield => {
                "Bonds rated BB or lower. Considered junk bonds, offering high yields due \
                to the increased risk of default."
            },
            BondQuality::InvestmentGrade => "Bonds rated AAA to BBB. Low risk of default.",
        }
    }
}

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum BondName {
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Bond {
    /// The name of the bond
    pub name: BondName,

    /// The kind of bond (government or corporate)
    pub kind: BondKind,

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
    /// Issue a new bond, recalculating interest and face value
    pub fn issue(&mut self) {
        self.prices
            .push(self.prices.last().unwrap() * (1.0 + self.interest / 100.));
    }
}

impl Instrument for Bond {
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
        InstrumentKind::Bond(self.name)
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
            name: BondName::Australia,
            kind: BondKind::Government,
            quality: BondQuality::InvestmentGrade,
            prices: vec![10000.],
            interest: 4.8,
            term: Term::FiveYears,
        },
        Bond {
            name: BondName::Brazil,
            kind: BondKind::Government,
            quality: BondQuality::HighYield,
            prices: vec![10000.],
            interest: 6.5,
            term: Term::FiveYears,
        },
        Bond {
            name: BondName::Canada,
            kind: BondKind::Government,
            quality: BondQuality::InvestmentGrade,
            prices: vec![10000.],
            interest: 4.2,
            term: Term::FiveYears,
        },
        Bond {
            name: BondName::China,
            kind: BondKind::Government,
            quality: BondQuality::InvestmentGrade,
            prices: vec![10000.],
            interest: 3.5,
            term: Term::FiveYears,
        },
        Bond {
            name: BondName::Japan,
            kind: BondKind::Government,
            quality: BondQuality::InvestmentGrade,
            prices: vec![10000.],
            interest: 0.5,
            term: Term::FiveYears,
        },
        Bond {
            name: BondName::Russia,
            kind: BondKind::Government,
            quality: BondQuality::HighYield,
            prices: vec![10000.],
            interest: 7.5,
            term: Term::FiveYears,
        },
        Bond {
            name: BondName::SaudiArabia,
            kind: BondKind::Government,
            quality: BondQuality::InvestmentGrade,
            prices: vec![10000.],
            interest: 3.8,
            term: Term::FiveYears,
        },
        Bond {
            name: BondName::SouthAfrica,
            kind: BondKind::Government,
            quality: BondQuality::HighYield,
            prices: vec![10000.],
            interest: 8.0,
            term: Term::FiveYears,
        },
        Bond {
            name: BondName::Ukraine,
            kind: BondKind::Government,
            quality: BondQuality::HighYield,
            prices: vec![10000.],
            interest: 10.0,
            term: Term::FiveYears,
        },
        Bond {
            name: BondName::USA,
            kind: BondKind::Government,
            quality: BondQuality::InvestmentGrade,
            prices: vec![10000.],
            interest: 4.,
            term: Term::ThreeYears,
        },
        Bond {
            name: BondName::Venezuela,
            kind: BondKind::Government,
            quality: BondQuality::HighYield,
            prices: vec![10000.],
            interest: 12.0,
            term: Term::FiveYears,
        },
    ]
}
