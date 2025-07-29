use std::collections::HashMap;

use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::core::sectors::{Sector, SectorName};
use crate::utils::{DQueue, NameFromEnum};

#[derive(EnumIter, Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Company {
    #[default]
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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum ESGRating {
    AAA,
    AA,
    A,
    BBB,
    BB,
    B,
    CCC,
}

impl ESGRating {
    pub fn description(&self) -> &str {
        match self {
            ESGRating::AAA => "Industry leader in managing ESG risks and opportunities.",
            ESGRating::AA => "Strong ESG practices and risk management above peers.",
            ESGRating::A => "Adequate ESG performance. Manages key risks reasonably well.",
            ESGRating::BBB => "	Moderate ESG risk exposure and average risk management practices.",
            ESGRating::BB => "Below-average ESG performance. Some unmanaged or unaddressed risks.",
            ESGRating::B => "Poor ESG practices, with significant risk exposure.",
            ESGRating::CCC => {
                "Worst performers. Very high ESG risks with little or no mitigation strategies."
            },
        }
    }

    pub fn increase(&self) -> ESGRating {
        match self {
            ESGRating::AAA => ESGRating::AAA,
            ESGRating::AA => ESGRating::AAA,
            ESGRating::A => ESGRating::AA,
            ESGRating::BBB => ESGRating::A,
            ESGRating::BB => ESGRating::BBB,
            ESGRating::B => ESGRating::BB,
            ESGRating::CCC => ESGRating::B,
        }
    }

    pub fn decrease(&self) -> ESGRating {
        match self {
            ESGRating::AAA => ESGRating::A,
            ESGRating::AA => ESGRating::BBB,
            ESGRating::A => ESGRating::BB,
            ESGRating::BBB => ESGRating::B,
            ESGRating::BB => ESGRating::CCC,
            ESGRating::B => ESGRating::CCC,
            ESGRating::CCC => ESGRating::CCC,
        }
    }

    pub fn value(&self) -> f32 {
        match self {
            ESGRating::AAA => 1.0,
            ESGRating::AA => 0.5,
            ESGRating::A => 0.,
            ESGRating::BBB => -0.2,
            ESGRating::BB => -0.4,
            ESGRating::B => -0.6,
            ESGRating::CCC => -1.0,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Stock {
    /// The issuer of the stock
    pub issuer: Company,

    /// Default price of the stock
    pub base_price: f32,

    /// The prices over time
    pub prices: DQueue<f32>,

    /// Percentage of the base price that can change daily
    pub volatility: f32,

    /// Average dividend given out per share
    pub dividend: f32,

    /// People's sentiment towards the stock (-1 to +1)
    pub sentiment: f32,

    /// ESG score of the company
    pub esg: ESGRating,

    /// Influence per sector
    pub sector: HashMap<SectorName, f32>,
}

impl Stock {
    pub fn bump(&mut self, inflation: f32, sectors: &Vec<Sector>) -> f32 {
        self.base_price *= 1. + inflation / 100. / 365.;

        let sector_effect = self
            .sector
            .iter()
            .map(|(s, w)| {
                sectors
                    .iter()
                    .find(|sec| sec.name == *s)
                    .map_or(0., |sec| w * sec.value as f32 / Sector::RANGE as f32)
            })
            .sum::<f32>();
        let sentiment_effect = self.sentiment() * 0.005 * self.base_price; // 0.5% max effect
        let esg_effect = self.esg.value() * 0.002 * self.base_price; // 0.2% max effect
        let volatility = self.base_price * self.volatility / 100.;

        let mut new_price = self.current() * (1. + inflation / 100. / 365.)
            + sector_effect
            + sentiment_effect
            + esg_effect
            + rng().random_range(-volatility..volatility);

        // Adjust price to tend towards the base price
        let deviation = (new_price - self.base_price) / self.base_price;
        new_price *= 1. + -deviation * deviation.abs() / 25.;

        new_price = new_price.max(1.);

        self.prices.push(new_price);
        new_price
    }
}

impl Instrument for Stock {
    fn name(&self) -> String {
        self.issuer.to_name()
    }

    fn lowername(&self) -> String {
        format!("{} stocks", self.issuer.to_lowername())
    }

    fn description(&self) -> &str {
        self.issuer.description()
    }

    fn kind(&self) -> InstrumentKind {
        InstrumentKind::Stock(self.issuer)
    }

    fn all(&self) -> &DQueue<f32> {
        &self.prices
    }

    fn dividend(&self) -> f32 {
        if self.current() < self.base_price * 0.5 {
            0. // Bad-performing stocks pay no dividend
        } else {
            // Good performing stocks pay out a higher dividend
            self.dividend * (1. + (self.current() - self.base_price) / self.base_price)
        }
    }

    fn esg(&self) -> ESGRating {
        self.esg.clone()
    }

    fn sectors(&self) -> HashMap<SectorName, f32> {
        self.sector.clone()
    }

    fn sentiment(&self) -> f32 {
        self.sentiment
    }

    fn volatility(&self) -> f32 {
        self.volatility
    }
}

pub fn start_stocks() -> Vec<Stock> {
    vec![
        Stock {
            issuer: Company::Apple,
            base_price: 175.,
            prices: DQueue::from([175.]),
            volatility: 2.5,
            dividend: 0.22,
            sector: HashMap::from([(SectorName::Technology, 0.8), (SectorName::Retail, 0.2)]),
            sentiment: 0.,
            esg: ESGRating::AA,
        },
        Stock {
            issuer: Company::Boeing,
            base_price: 210.,
            prices: DQueue::from([210.]),
            volatility: 3.2,
            dividend: 1.5,
            sector: HashMap::from([(SectorName::Transport, 0.7), (SectorName::Military, 0.3)]),
            sentiment: 0.,
            esg: ESGRating::CCC,
        },
        Stock {
            issuer: Company::GoldmanSachs,
            base_price: 350.,
            prices: DQueue::from([350.]),
            volatility: 2.0,
            dividend: 3.0,
            sector: HashMap::from([(SectorName::Finance, 0.9), (SectorName::Technology, 0.1)]),
            sentiment: 0.,
            esg: ESGRating::BB,
        },
        Stock {
            issuer: Company::Inditex,
            base_price: 32.,
            prices: DQueue::from([32.]),
            volatility: 1.8,
            dividend: 0.25,
            sector: HashMap::from([(SectorName::Fashion, 0.6), (SectorName::Retail, 0.4)]),
            sentiment: 0.,
            esg: ESGRating::A,
        },
        Stock {
            issuer: Company::LockheedMartin,
            base_price: 470.,
            prices: DQueue::from([470.]),
            volatility: 2.0,
            dividend: 3.25,
            sector: HashMap::from([(SectorName::Military, 0.8), (SectorName::Transport, 0.2)]),
            sentiment: 0.,
            esg: ESGRating::CCC,
        },
        Stock {
            issuer: Company::LVMH,
            base_price: 830.,
            prices: DQueue::from([830.]),
            volatility: 1.5,
            dividend: 4.25,
            sector: HashMap::from([(SectorName::Fashion, 0.8), (SectorName::Retail, 0.2)]),
            sentiment: 0.,
            esg: ESGRating::AA,
        },
        Stock {
            issuer: Company::Maersk,
            base_price: 1450.,
            prices: DQueue::from([1450.]),
            volatility: 2.8,
            dividend: 9.5,
            sector: HashMap::from([(SectorName::Transport, 0.8), (SectorName::Energy, 0.2)]),
            sentiment: 0.,
            esg: ESGRating::A,
        },
        Stock {
            issuer: Company::Moderna,
            base_price: 110.,
            prices: DQueue::from([110.]),
            volatility: 4.5,
            dividend: 0.0,
            sector: HashMap::from([(SectorName::Healthcare, 0.9), (SectorName::Technology, 0.1)]),
            sentiment: 0.,
            esg: ESGRating::AA,
        },
        Stock {
            issuer: Company::Nestle,
            base_price: 120.,
            prices: DQueue::from([120.]),
            volatility: 1.2,
            dividend: 0.875,
            sector: HashMap::from([
                (SectorName::Food, 0.7),
                (SectorName::Healthcare, 0.2),
                (SectorName::Retail, 0.1),
            ]),
            sentiment: 0.,
            esg: ESGRating::AAA,
        },
        Stock {
            issuer: Company::Nvidia,
            base_price: 1250.,
            prices: DQueue::from([1250.]),
            volatility: 4.2,
            dividend: 0.3,
            sector: HashMap::from([(SectorName::Technology, 0.85), (SectorName::Military, 0.15)]),
            sentiment: 0.,
            esg: ESGRating::A,
        },
        Stock {
            issuer: Company::Pfizer,
            base_price: 30.,
            prices: DQueue::from([30.]),
            volatility: 2.5,
            dividend: 0.275,
            sector: HashMap::from([(SectorName::Healthcare, 0.9), (SectorName::Retail, 0.1)]),
            sentiment: 0.,
            esg: ESGRating::AA,
        },
        Stock {
            issuer: Company::RioTinto,
            base_price: 65.,
            prices: DQueue::from([65.]),
            volatility: 1.9,
            dividend: 0.65,
            sector: HashMap::from([(SectorName::Materials, 0.9), (SectorName::Energy, 0.1)]),
            sentiment: 0.,
            esg: ESGRating::BBB,
        },
        Stock {
            issuer: Company::Shell,
            base_price: 65.,
            prices: DQueue::from([65.]),
            volatility: 2.0,
            dividend: 0.625,
            sector: HashMap::from([(SectorName::Energy, 0.8), (SectorName::Materials, 0.2)]),
            sentiment: 0.,
            esg: ESGRating::BB,
        },
        Stock {
            issuer: Company::Toyota,
            base_price: 190.,
            prices: DQueue::from([190.]),
            volatility: 2.1,
            dividend: 0.45,
            sector: HashMap::from([
                (SectorName::Transport, 0.6),
                (SectorName::Energy, 0.1),
                (SectorName::Technology, 0.1),
                (SectorName::Retail, 0.1),
            ]),
            sentiment: 0.,
            esg: ESGRating::A,
        },
        Stock {
            issuer: Company::Unilever,
            base_price: 50.,
            prices: DQueue::from([50.]),
            volatility: 1.3,
            dividend: 0.475,
            sector: HashMap::from([(SectorName::Retail, 0.5), (SectorName::Food, 0.5)]),
            sentiment: 0.,
            esg: ESGRating::AAA,
        },
    ]
}
