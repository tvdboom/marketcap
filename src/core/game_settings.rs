use crate::core::states::AudioState;
use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct GameSettings {
    /// Audio setting
    pub audio: AudioState,

    /// Game speed (1-5)
    pub speed: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            audio: AudioState::default(),
            speed: 1.,
        }
    }
}
