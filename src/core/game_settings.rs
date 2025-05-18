use crate::core::constants::DEFAULT_SPEED;
use crate::core::states::AudioState;
use crate::core::ui::systems::Tab;
use bevy::prelude::*;
use chrono::{Local, NaiveDate};
use std::time::Duration;

#[derive(Resource, Clone)]
pub struct GameSettings {
    /// Audio setting
    pub audio: AudioState,

    /// Game speed (1-5)
    pub speed: f32,

    /// Current in-game date
    pub date: NaiveDate,

    /// Timer for the game clock
    pub clock: Timer,

    /// Active tab in the game's UI
    pub tab: Tab,

    /// Global economy value (0-100)
    pub economy: Vec<f32>,

    /// Global interest rate (0-1)
    pub interest_rate: Vec<f32>,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            audio: AudioState::default(),
            speed: 1.,
            date: Local::now().date_naive(),
            clock: Timer::new(Duration::from_secs_f32(DEFAULT_SPEED), TimerMode::Repeating),
            tab: Tab::Home,
            economy: Vec::from([0.5]),
            interest_rate: Vec::from([0.05]),
        }
    }
}

impl GameSettings {
    pub fn economy(&self) -> f32 {
        *self.economy.last().unwrap()
    }

    pub fn interest_rate(&self) -> f32 {
        *self.interest_rate.last().unwrap()
    }
}
