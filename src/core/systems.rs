use crate::core::game_params::GameParams;
use bevy::prelude::*;
use chrono::Duration;

pub fn time_pass(mut game_params: ResMut<GameParams>, time: Res<Time>) {
    game_params.clock.tick(time.delta());

    if game_params.clock.just_finished() {
        game_params.date = game_params.date + Duration::days(1);

        game_params.economic_factor.bump();

        let economy = game_params.economic_factor.current();
        game_params.interest_rate.bump(economy);
    }
}
