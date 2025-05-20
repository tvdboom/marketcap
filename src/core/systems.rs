use crate::core::attributes::attribute::Attribute;
use crate::core::game_params::GameParams;
use crate::core::player::Player;
use bevy::prelude::*;
use chrono::Datelike;

pub fn time_pass(mut game_params: ResMut<GameParams>, mut player: ResMut<Player>, time: Res<Time>) {
    game_params.clock.tick(time.delta());

    if game_params.clock.just_finished() {
        let month = game_params.date.month();

        // Advance 1 day
        game_params.date = game_params.date.succ_opt().unwrap();

        // Daily operations =================================== >>

        game_params.economic_factor.bump();

        let economy = game_params.economic_factor.current();
        game_params.interest_rate.bump(economy);

        player.cash.bump(game_params.interest_rate.current());

        // Monthly operations =================================== >>

        if month != game_params.date.month() {
            player.cash.resolve();
        }
    }
}
