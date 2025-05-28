use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

use crate::core::assets::WorldAssets;
use crate::core::game_settings::{AudioSetting, GameSettings};

#[derive(Event)]
pub struct PlayAudioEv {
    pub name: &'static str,
    pub volume: f64,
}

impl PlayAudioEv {
    pub fn new(name: &'static str) -> Self {
        Self { name, volume: 1. }
    }
}

pub fn toggle_music_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_settings: ResMut<GameSettings>,
) {
    if keyboard.just_pressed(KeyCode::KeyM) {
        game_settings.audio = match game_settings.audio {
            AudioSetting::Mute => AudioSetting::NoMusic,
            AudioSetting::NoMusic => AudioSetting::Sound,
            AudioSetting::Sound => AudioSetting::Mute,
        }
    }
}

pub fn play_audio_event(
    mut ev: EventReader<PlayAudioEv>,
    game_settings: Res<GameSettings>,
    audio: Res<Audio>,
    assets: Local<WorldAssets>,
) {
    if game_settings.audio != AudioSetting::Mute {
        for PlayAudioEv { name, volume } in ev.read() {
            audio.play(assets.audio(name)).with_volume(*volume);
        }
    }
}
