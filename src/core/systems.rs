use bevy::prelude::*;
use chrono::Datelike;

use crate::core::factors::Factor;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::bonds::BondKind;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::player::Player;

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

        let (_, inflation, _) = economy.bump();

        player.cash.bump(economy.interest.current());

        // Increase storage costs for commodities, following inflation
        for commodity in economy.commodities.iter_mut() {
            commodity.storage_cost *= 1. + inflation / 100. / 365.;
        }

        player.resolve_orders(&economy, &mut message);

        if economy.date.day() == 1 {
            // Monthly operations =================================== >>

            // Central bank calculates/pushes next interest rate
            let inflation = economy.inflation.current();
            economy.interest.resolve(inflation);

            // Interest on cash is paid
            player.cash.resolve();

            // Bi-yearly operations =================================== >>

            if economy.date.month() % 6 == 1 {
                // Bond's interest is paid
                for owned in player.bonds() {
                    // let bond = economy.get(&owned.instrument);
                    // player.cash.amount += owned.interest * bond.face_value().iter().sum::<f32>();
                }

                // Government bonds are issued
                for bond in &mut economy
                    .bonds
                    .iter_mut()
                    .filter(|b| b.kind == BondKind::Government)
                {
                    bond.issue();
                }
            }

            // Yearly operations =================================== >>

            if economy.date.month() == 1 {
                // Corporate bonds are issued
                for bond in &mut economy
                    .bonds
                    .iter_mut()
                    .filter(|b| b.kind == BondKind::Corporate)
                {
                    bond.issue();
                }
            }

            // Resolve debts ======================================= >>

            if !player.resolve_debts(&economy) {
                message.write(MessageEv {
                    message: "You don't have enough cash to pay your debts!".to_string(),
                    level: MessageLevel::Error,
                });
            }
        }

        // Warning messages =================================== >>

        // Check if player has enough cash to cover outflow
        if economy.date.day() == 20 && player.outflow(&economy) > player.cash.current() {
            message.write(MessageEv {
                message: "You're outflow is larger than your cash reserve!".to_string(),
                level: MessageLevel::Warning,
            });
        }
    }
}
