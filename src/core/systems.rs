use crate::core::factors::Factor;
use crate::core::global_economy::GlobalEconomy;
use crate::core::messages::Messages;
use crate::core::player::Player;
use bevy::prelude::*;
use chrono::Datelike;

pub fn time_pass(
    mut global_economy: ResMut<GlobalEconomy>,
    mut player: ResMut<Player>,
    mut messages: ResMut<Messages>,
    time: Res<Time>,
) {
    global_economy.clock.tick(time.delta());

    if global_economy.clock.just_finished() {
        // Advance 1 day
        global_economy.date = global_economy.date.succ_opt().unwrap();

        // Daily operations =================================== >>

        global_economy.bump();

        player.cash.bump(global_economy.interest.current());

        // Monthly operations =================================== >>

        if global_economy.date.day() == 1 {
            // Central bank calculates/pushes next interest rate
            let inflation = global_economy.inflation.current();
            global_economy.interest.resolve(inflation);

            // Interest on cash is paid
            player.cash.resolve();

            // Loans are paid
            if !player.resolve_loans() {
                messages.error("You defaulted on a loan!");
            }
        }
    }
}
