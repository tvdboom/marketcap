use std::time::Duration;

use bevy::prelude::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::core::constants::DEFAULT_SPEED;
use crate::core::countries::{Country, start_countries};
use crate::core::factors::economy::Economy;
use crate::core::factors::inflation::Inflation;
use crate::core::factors::interest::Interest;
use crate::core::instruments::bonds::{Bond, start_bonds};
use crate::core::instruments::commodities::{Commodity, start_commodities};
use crate::core::instruments::crypto::{Crypto, start_cryptos};
use crate::core::instruments::instrument::Instrument;
use crate::core::instruments::instrument::InstrumentKind;

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct GlobalEconomy {
    /// Current in-game date
    pub date: NaiveDate,

    /// Timer for the game clock
    pub clock: Timer,

    /// Global economic factor (0-100)
    pub economy: Economy,

    /// Inflation rate (1-10)
    pub inflation: Inflation,

    /// Global interest rate (1-10)
    pub interest: Interest,

    /// Information about all countries
    pub countries: Vec<Country>,

    /// Information about all bonds
    pub bonds: Vec<Bond>,

    /// Information of all commodities
    pub commodities: Vec<Commodity>,

    /// Information about all cryptocurrencies
    pub cryptos: Vec<Crypto>,
}

impl GlobalEconomy {
    /// Daily changes in the global economy
    pub fn bump(&mut self) -> (f32, f32, f32) {
        let economy = self.economy.bump();
        let interest = self.interest.bump();
        let inflation = self.inflation.bump(economy, interest);

        for commodity in &mut self.commodities {
            commodity.bump(economy, inflation);
        }

        for crypto in &mut self.cryptos {
            crypto.bump(inflation);
        }

        (economy, inflation, interest)
    }

    pub fn get(&self, instrument: &InstrumentKind) -> &dyn Instrument {
        match instrument {
            InstrumentKind::Bond(issuer) => {
                self.bonds.iter().find(|b| b.issuer == *issuer).unwrap()
            },
            InstrumentKind::Commodity(name) => {
                self.commodities.iter().find(|c| c.name == *name).unwrap()
            },
            InstrumentKind::Crypto(name) => self.cryptos.iter().find(|c| c.name == *name).unwrap(),
        }
    }

    pub fn get_price(&self, instrument: &InstrumentKind) -> f32 {
        self.get(instrument).current()
    }
}

impl Default for GlobalEconomy {
    fn default() -> Self {
        Self {
            date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            clock: Timer::new(Duration::from_secs_f32(DEFAULT_SPEED), TimerMode::Repeating),
            economy: Economy::default(),
            inflation: Inflation::default(),
            interest: Interest::default(),
            countries: start_countries(),
            bonds: start_bonds(),
            commodities: start_commodities(),
            cryptos: start_cryptos(),
        }
    }
}
