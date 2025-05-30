use std::time::Duration;

use bevy::prelude::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::core::constants::DEFAULT_SPEED;
use crate::core::factors::economy::Economy;
use crate::core::factors::inflation::Inflation;
use crate::core::factors::interest::Interest;
use crate::core::securities::{Security, SecurityName, start_securities};

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

    /// Information of all securities
    pub securities: Vec<Security>,
}

impl GlobalEconomy {
    /// Daily changes in the global economy
    pub fn bump(&mut self) {
        let economy = self.economy.bump();
        let interest = self.interest.bump();
        let inflation = self.inflation.bump(economy, interest);

        for security in &mut self.securities {
            security.bump(inflation);
        }
    }

    pub fn get(&self, name: &SecurityName) -> &Security {
        self.securities.iter().find(|c| c.name == *name).unwrap()
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
            securities: start_securities(),
        }
    }
}
