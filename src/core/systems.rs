use bevy::prelude::*;
use chrono::{Datelike, Duration};

use crate::core::factors::Factor;
use crate::core::global_economy::GlobalEconomy;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::player::Player;
use crate::utils::NameFromEnum;

pub fn time_pass(
    mut economy: ResMut<GlobalEconomy>,
    mut player: ResMut<Player>,
    mut message: EventWriter<MessageEv>,
    time: Res<Time>,
) {
    economy.clock.tick(time.delta());

    if economy.clock.just_finished() {
        // Advance 1 day
        economy.date = economy.date.succ_opt().unwrap();

        // Daily operations =================================== >>

        economy.bump();

        player.cash.bump(economy.interest.current());

        player.commodities.retain_mut(|commodity| {
            if let Some(maturity) = commodity.maturity_date(&economy) {
                if maturity <= economy.date {
                    // Commodity has degraded -> remove it
                    message.write(MessageEv {
                        message: format!(
                            "{} units of {} have degraded and have been removed!",
                            commodity.amount,
                            commodity.kind.to_lowername()
                        ),
                        level: MessageLevel::Error,
                    });

                    return false; // Remove commodity
                } else if !commodity.warning && maturity <= economy.date + Duration::days(30) {
                    // Commodity is about to degrade -> warn player
                    commodity.warning = true;
                    message.write(MessageEv {
                        message: format!(
                            "{} units of {} are degrading in 30 days!",
                            commodity.amount,
                            commodity.kind.to_lowername()
                        ),
                        level: MessageLevel::Warning,
                    });
                }
            }

            // Keep commodity if it hasn't degraded
            true
        });

        if economy.date.day() == 20 && player.outflow() > player.cash.current() {
            message.write(MessageEv {
                message: "You're outflow is larger than your cash reserve!".to_string(),
                level: MessageLevel::Warning,
            });
        }

        // Monthly operations =================================== >>

        if economy.date.day() == 1 {
            // Central bank calculates/pushes next interest rate
            let inflation = economy.inflation.current();
            economy.interest.resolve(inflation);

            // Interest on cash is paid
            player.cash.resolve();

            // Loans are paid
            if !player.resolve_loans() {
                message.write(MessageEv {
                    message: "You defaulted on a loan!".to_string(),
                    level: MessageLevel::Error,
                });
            }
        }
    }
}
