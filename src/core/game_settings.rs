use crate::core::constants::DEFAULT_SPEED;
use crate::core::states::AudioState;
use crate::core::ui::systems::Tab;
use bevy::prelude::*;
use chrono::{Local, NaiveDate};
use std::time::Duration;

#[derive(Resource, Clone)]
pub struct GameSettings {
    pub audio: AudioState,
    pub date: NaiveDate,
    pub speed: f32,
    pub clock: Timer,
    pub tab: Tab,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            audio: AudioState::default(),
            date: Local::now().date_naive(),
            speed: 1.,
            clock: Timer::new(Duration::from_secs_f32(DEFAULT_SPEED), TimerMode::Repeating),
            tab: Tab::Home,
        }
    }
}
