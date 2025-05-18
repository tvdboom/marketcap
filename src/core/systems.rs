use crate::core::constants::BASE_INTEREST_RATE;
use crate::core::game_settings::GameSettings;
use bevy::prelude::*;
use chrono::Duration;

fn calculate_global_economy(economy: f32) -> f32 {
    economy + rng().random_range(-2.5, 2.5)
}

fn calculate_interest_rate(economy: f32) -> f32 {
    BASE_INTEREST_RATE + ((1. - BASE_INTEREST_RATE) * (1. - economy))
}

pub fn time_pass(mut game_settings: ResMut<GameSettings>, time: Res<Time>) {
    game_settings.clock.tick(time.delta());

    if game_settings.clock.just_finished() {
        game_settings.date = game_settings.date + Duration::days(1);

        let interest = calculate_interest_rate(game_settings.economy());
        game_settings.interest_rate.push(interest);
    }
}
