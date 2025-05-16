use crate::core::states::AudioState;
use bevy::prelude::*;
use chrono::{Local, NaiveDate};

#[derive(Resource, Clone)]
pub struct GameSettings {
    pub audio: AudioState,
    pub speed: u32,
    pub date: NaiveDate,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            audio: AudioState::default(),
            speed: 5,
            date: Local::now().date_naive(),
        }
    }
}
