use crate::core::attributes::global_economic_factor::GlobalEconomicFactor;
use crate::core::attributes::global_interest_rate::GlobalInterestRate;
use crate::core::constants::DEFAULT_SPEED;
use crate::core::ui::systems::Tab;
use bevy::prelude::*;
use chrono::{Local, NaiveDate};
use std::time::Duration;

#[derive(Resource, Clone)]
pub struct GameParams {
    /// Current in-game date
    pub date: NaiveDate,

    /// Timer for the game clock
    pub clock: Timer,

    /// Active tab in the game's UI
    pub tab: Tab,

    /// Economic factor (0-100)
    pub economic_factor: GlobalEconomicFactor,

    /// Interest rate (2-10)
    pub interest_rate: GlobalInterestRate,
}

impl Default for GameParams {
    fn default() -> Self {
        Self {
            date: Local::now().date_naive(),
            clock: Timer::new(Duration::from_secs_f32(DEFAULT_SPEED), TimerMode::Repeating),
            tab: Tab::default(),
            economic_factor: GlobalEconomicFactor::default(),
            interest_rate: GlobalInterestRate::default(),
        }
    }
}
