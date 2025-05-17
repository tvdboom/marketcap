use crate::core::constants::{GAME_SPEED_STEP, MAX_GAME_SPEED};
use crate::core::game_settings::GameSettings;
use crate::core::states::{AppState, GameState};
use bevy::prelude::*;
use std::time::Duration;

pub fn toggle_pause_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut game_settings: ResMut<GameSettings>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match app_state.get() {
            AppState::Game => match game_state.get() {
                GameState::Running | GameState::Paused => {
                    next_game_state.set(GameState::InGameMenu)
                }
                GameState::InGameMenu => next_game_state.set(GameState::Running),
            },
            _ => (),
        }
    }

    if keyboard.just_pressed(KeyCode::Space) {
        match game_state.get() {
            GameState::Running => next_game_state.set(GameState::Paused),
            GameState::Paused => next_game_state.set(GameState::Running),
            _ => (),
        }
    }

    if keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
        if keyboard.just_pressed(KeyCode::ArrowLeft) && game_settings.speed > 0. {
            game_settings.speed -= GAME_SPEED_STEP;

            if game_settings.speed == 0. {
                next_game_state.set(GameState::Paused);
            } else {
                let duration = Duration::from_secs_f32(1. / game_settings.speed);
                game_settings.clock.set_duration(duration);
            }
        }

        if keyboard.just_pressed(KeyCode::ArrowRight) && game_settings.speed < MAX_GAME_SPEED {
            game_settings.speed += GAME_SPEED_STEP;

            let duration = Duration::from_secs_f32(1. / game_settings.speed);
            game_settings.clock.set_duration(duration);

            next_game_state.set(GameState::Running);
        }
    }
}
