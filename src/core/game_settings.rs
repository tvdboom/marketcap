use crate::core::states::AudioState;
use crate::core::ui::themes::{Aesthetics, NordDark, NordLight};
use bevy::prelude::*;

#[derive(Clone, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl Theme {
    pub fn get(&self) -> Box<dyn Aesthetics> {
        match self {
            Theme::Dark => Box::new(NordDark),
            Theme::Light => Box::new(NordLight),
        }
    }
}

#[derive(Resource, Clone)]
pub struct GameSettings {
    /// Audio setting
    pub audio: AudioState,

    /// Game speed (1-5)
    pub speed: f32,

    /// Ui theme
    pub theme: Theme,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            audio: AudioState::default(),
            speed: 1.,
            theme: Theme::default(),
        }
    }
}
