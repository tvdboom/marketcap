use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use crate::core::instruments::Instrument;
use crate::core::loans::Term;
use crate::utils::NameFromEnum;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BondType {
    Government,
    Corporate,
}

impl BondType {
    pub fn description(&self) -> &str {
        match self {
            BondType::Government => "Bonds issued by governments. Generally considered low risk.",
            BondType::Corporate => "\
                Bonds issued by corporations. Higher risk than government bonds, \
                but potentially higher returns.",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BondQuality {
    InvestmentGrade,
    HighYield,
}

impl BondQuality {
    pub fn description(&self) -> &str {
        match self {
            BondQuality::InvestmentGrade => "Bonds rated AAA to BBB. Low risk of default.",
            BondQuality::HighYield => "\
                Bonds rated BB or lower. Considered junk bonds, offering high yields due \
                to the increased risk.",
        }
    }
}

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum BondName {
    #[default]
    Australia,
    USA,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Bond {
    /// The name of the bond
    pub name: BondName,

    /// The type of the bond (government or corporate)
    pub bond_type: BondType,

    /// The quality of the bond (investment grade or high yield)
    pub quality: BondQuality,

    /// The original value of the bond
    pub face_value: Vec<f32>,
    
    /// Interest rate on the bond
    pub interest: f32,

    /// Number of years until the bond matures
    pub term: Term,
}

impl Instrument for Bond {
    fn name(&self) -> String {
        self.name.to_name()
    }

    fn lowername(&self) -> String {
        self.name.to_lowername()
    }

    fn all(&self) -> &Vec<f32> {
        &self.face_value
    }

    fn current(&self) -> f32 {
        *self.face_value.last().unwrap()
    }
}

pub fn start_bonds() -> Vec<Bond> {
    vec![
        Bond {
            name: BondName::Australia,
            bond_type: BondType::Government,
            quality: BondQuality::InvestmentGrade,
            face_value: vec![100.0, 102.0, 101.5, 103.0],
            interest: 0.03,
            term: Term::FiveYears,
        },
        Bond {
            name: BondName::USA,
            bond_type: BondType::Government,
            quality: BondQuality::InvestmentGrade,
            face_value: vec![100.0, 98.0, 97.5, 99.0],
            interest: 0.05,
            term: Term::ThreeYears,
        },
    ]
}
