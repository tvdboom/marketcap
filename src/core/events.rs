use crate::core::instruments::crypto::CryptoName;
use crate::utils::NameFromEnum;
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;
use rand::rng;
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventName {
    CryptoCrash(CryptoName),
    Covid,
}

impl EventName {
    pub fn create_event() -> EconomicEvent {
        let weights = Self::iter().map(|e| e.weight()).collect::<Vec<_>>();
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut choice = Self::iter().collect::<Vec<_>>()[dist.sample(&mut rng())].clone();

        choice = match choice {
            EventName::CryptoCrash(_) => {
                EventName::CryptoCrash(CryptoName::iter().choose(&mut rng()).unwrap())
            },
            _ => choice,
        };

        EconomicEvent::new(choice, 1)
    }

    pub fn weight(&self) -> f32 {
        match self {
            EventName::CryptoCrash(_) => 1.,
            EventName::Covid => 0.1,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EconomicEvent {
    /// Name of the event
    pub name: EventName,

    /// Number of days the event is active
    pub duration: u32,
}

impl EconomicEvent {
    pub fn new(name: EventName, duration: u32) -> Self {
        Self { name, duration }
    }

    pub fn title(&self) -> String {
        match self.name {
            EventName::CryptoCrash(name) => format!("{} crash", name.to_name()),
            EventName::Covid => "Covid-19 pandemic".to_string(),
        }
    }

    pub fn image(&self) -> String {
        self.name.to_lowername().replace(" ", "-")
    }

    pub fn description(&self) -> String {
        match self.name {
            EventName::CryptoCrash(name) => format!(
                "A sudden and significant drop in the value of cryptocurrency {} due to mistrust in the underlying technology.",
                name.to_name()
            ),
            EventName::Covid => {
                "The Covid-19 pandemic causes a global economic downturn, affecting all sectors."
                    .to_string()
            },
        }
    }

    pub fn advance(&self) {}
}
