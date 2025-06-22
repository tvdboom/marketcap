use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::utils::NameFromEnum;

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Sector {
    Finance,
    Food,
    Healthcare,
    Materials,
    Military,
    Retail,
    Technology,
    Transport,
}

#[derive(EnumIter, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Company {
    Apple,
    Boeing,
    GoldmanSachs,
    Inditex,
    LockheedMartin,
    LVMH,
    Maersk,
    Moderna,
    Nestle,
    Nvidia,
    Pfizer,
    RioTinto,
    Shell,
    Toyota,
    Unilever,
}

impl Company {
    pub fn description(&self) -> &str {
        match self {
            Company::Apple => {
                "Apple Inc. is an American multinational technology company that designs, \
                manufactures, and markets consumer electronics, software, and services."
            },
            Company::Boeing => {
                "Boeing is an American multinational corporation that designs, manufactures, \
                and sells airplanes, rotorcraft, rockets, satellites, telecommunications \
                equipment, and missiles worldwide."
            },
            Company::GoldmanSachs => {
                "Goldman Sachs is a leading global investment banking, securities and investment \
                management firm that provides a wide range of financial services to a substantial \
                and diversified client base."
            },
            Company::Inditex => {
                "Industria de Diseño Textil S.A. (Inditex) is a Spanish multinational clothing \
                company known for its fashion retail chain Zara."
            },
            Company::LockheedMartin => {
                "Lockheed Martin is an American aerospace, defense, arms, security, and advanced \
                technologies company with worldwide interests."
            },
            Company::LVMH => {
                "LVMH Moët Hennessy Louis Vuitton is a French multinational luxury goods conglomerate."
            },
            Company::Maersk => {
                "A.P. Moller - Maersk is a Danish integrated shipping company and the largest \
                container ship operator in the world."
            },
            Company::Moderna => {
                "Moderna Inc. is an American biotechnology company pioneering messenger RNA \
                (mRNA) therapeutics and vaccines."
            },
            Company::Nestle => {
                "Nestlé S.A. is a Swiss multinational food and drink processing conglomerate \
                corporation headquartered in Vevey, Switzerland."
            },
            Company::Nvidia => {
                "Nvidia Corporation is an American multinational technology company incorporated \
                in Delaware and based in Santa Clara, California."
            },
            Company::Pfizer => {
                "Pfizer Inc. is an American multinational pharmaceutical corporation headquartered \
                in Manhattan, New York City."
            },
            Company::RioTinto => {
                "Rio Tinto Group is an Anglo-Australian multinational and one of the world's \
                largest metals and mining corporations."
            },
            Company::Shell => {
                "Royal Dutch Shell plc is a British-Dutch multinational oil and gas company \
                headquartered in The Hague, Netherlands."
            },
            Company::Toyota => {
                "Toyota Motor Corporation is a Japanese multinational automotive manufacturer \
                headquartered in Toyota City, Aichi, Japan."
            },
            Company::Unilever => {
                "Unilever PLC is a British-Dutch multinational consumer goods company \
                co-headquartered in London, England, and Rotterdam, Netherlands."
            },
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Stock {
    /// The issuer of th stock
    pub issuer: Company,

    /// Default price of the stock
    pub base_price: f32,

    /// The prices over time
    pub prices: Vec<f32>,

    /// Percentage of the base price that can change daily
    pub volatility: f32,

    /// Influence per sector
    pub sector: HashMap<Sector, f32>,
}

impl Instrument for Stock {
    fn name(&self) -> String {
        self.issuer.to_name()
    }

    fn lowername(&self) -> String {
        self.issuer.to_lowername()
    }

    fn description(&self) -> &str {
        self.issuer.description()
    }

    fn kind(&self) -> InstrumentKind {
        InstrumentKind::Stock(self.issuer)
    }

    fn all(&self) -> &Vec<f32> {
        &self.prices
    }

    fn current(&self) -> f32 {
        *self.prices.last().unwrap()
    }

    fn volatility(&self) -> f32 {
        self.volatility
    }
}

pub fn start_stocks() -> Vec<Stock> {
    vec![Stock {
        issuer: Company::Boeing,
        base_price: 100.,
        prices: vec![100.],
        volatility: 3.0,
        sector: HashMap::from([(Sector::Transport, 0.5), (Sector::Military, 0.5)]),
    }]
}
