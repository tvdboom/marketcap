use std::time::Duration;

use bevy::prelude::*;

use crate::core::constants::{GAME_SPEED_STEP, MAX_GAME_SPEED};
use crate::core::game_settings::GameSettings;
use crate::core::global_economy::GlobalEconomy;
use crate::core::player::Player;
use crate::core::states::GameState;

pub fn toggle_pause_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut game_settings: ResMut<GameSettings>,
    mut economy: ResMut<GlobalEconomy>,
    mut player: ResMut<Player>,
) {
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
                economy.clock.set_duration(Duration::from_secs_f32(1. / game_settings.speed));
            }
        }

        if keyboard.just_pressed(KeyCode::ArrowRight) && game_settings.speed < MAX_GAME_SPEED {
            game_settings.speed += GAME_SPEED_STEP;

            economy.clock.set_duration(Duration::from_secs_f32(1. / game_settings.speed));

            next_game_state.set(GameState::Running);
        }

        if keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            // Hack to control global economy
            if keyboard.just_pressed(KeyCode::ArrowUp) {
                *economy.economy.values.back_mut().unwrap() += 10.0;
            } else if keyboard.just_pressed(KeyCode::ArrowDown) {
                *economy.economy.values.back_mut().unwrap() -= 10.0;
            }

            // Hack to control cash
            if keyboard.just_pressed(KeyCode::KeyV) {
                player.cash.amount *= 1000.;
            } else if keyboard.just_pressed(KeyCode::KeyC) {
                player.cash.amount -= 1000.;
            }

            // Hack to unlock all technologies
            if keyboard.just_pressed(KeyCode::KeyT) {
                for tech in player.research.technologies.iter_mut() {
                    tech.progress = 100.;
                    tech.researching = false;
                }
            }
        }
    }
}
