use crate::core::constants::DEFAULT_SPEED;
use crate::core::factors::Factor;
use crate::core::factors::economy::Economy;
use crate::core::factors::inflation::Inflation;
use crate::core::factors::interest::Interest;
use bevy::prelude::*;
use chrono::{Local, NaiveDate};
use std::time::Duration;

#[derive(Resource, Clone)]
pub struct GlobalEconomy {
    /// Current in-game date
    pub date: NaiveDate,

    /// Timer for the game clock
    pub clock: Timer,

    /// Global economic factor (0-100)
    pub economy: Economy,

    /// Inflation rate (1-10)
    pub inflation: Inflation,

    /// Global interest rate (2-10)
    pub interest: Interest,
}

impl GlobalEconomy {
    /// Daily changes in the global economy
    pub fn bump(&mut self) {
        let economy = self.economy.bump();
        let interest = self.interest.current();
        self.inflation.bump(economy, interest);
    }
}

impl Default for GlobalEconomy {
    fn default() -> Self {
        Self {
            date: Local::now().date_naive(),
            clock: Timer::new(Duration::from_secs_f32(DEFAULT_SPEED), TimerMode::Repeating),
            economy: Economy::default(),
            inflation: Inflation::default(),
            interest: Interest::default(),
        }
    }
}
