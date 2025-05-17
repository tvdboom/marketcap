use crate::core::game_settings::GameSettings;
use bevy::prelude::*;
use chrono::Duration;

pub fn time_pass(mut game_settings: ResMut<GameSettings>, time: Res<Time>) {
    game_settings.clock.tick(time.delta());

    if game_settings.clock.just_finished() {
        game_settings.date = game_settings.date + Duration::days(1);
    }
}
