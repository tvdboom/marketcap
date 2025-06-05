use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::ui::themes::{Aesthetics, NordDark, NordLight};

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl Theme {
    pub fn emoji(&self) -> &str {
        match self {
            Theme::Dark => "🌙",
            Theme::Light => "☀",
        }
    }

    pub fn get(&self) -> Box<dyn Aesthetics> {
        match self {
            Theme::Dark => Box::new(NordDark),
            Theme::Light => Box::new(NordLight),
        }
    }
}

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum AudioSetting {
    Mute,
    #[default]
    NoMusic,
    Sound,
}

impl AudioSetting {
    pub fn emoji(&self) -> &str {
        match self {
            AudioSetting::Mute => "🔕",
            AudioSetting::NoMusic => "🔇",
            AudioSetting::Sound => "🔊",
        }
    }
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    /// Ui theme
    pub theme: Theme,

    /// Game speed (1-5)
    pub speed: f32,

    /// Audio setting
    pub audio: AudioSetting,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            speed: 1.,
            audio: AudioSetting::default(),
        }
    }
}
