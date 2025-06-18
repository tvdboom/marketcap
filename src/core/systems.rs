use bevy::prelude::*;
use chrono::Datelike;

use crate::core::factors::Factor;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::bonds::BondKind;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::orders::OrderEv;
use crate::core::player::{OwnedInstrument, Player};

pub fn time_pass(
    mut economy: ResMut<GlobalEconomy>,
    mut player: ResMut<Player>,
    mut order_ev: EventWriter<OrderEv>,
    mut close_short_ev: EventWriter<CloseShortEv>,
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

        // Update bounds for trailing orders
        for order in player.pending_orders_mut() {
            if order.lower_bound {
                order.bound = order.bound.min(economy.get_current(&order.instrument));
            } else {
                order.bound = order.bound.max(economy.get_current(&order.instrument));
            }
        }

        let mut has_debt = false;
        let mut has_paid = true;
        
        // Check margin call for short positions
        player.instruments.iter_mut().filter(|o| o.amount < 0).for_each(|o| {
            has_debt = true;

            let instrument = economy.get(&o.kind);
            let price = instrument.current() * o.amount.abs() as f32;

            let margin = 1.5 * o.start_price * o.amount as f32 / (1. + o.margin_frac - 0.1);
            if price > margin {
                has_paid = false;
                close_short_ev.write(CloseShortEv {
                    owned: o.clone(),
                    reason: "Margin reached".to_string(),
                });
            } else if price > margin * 0.9 && !o.warning {
                o.warning = true;
                message.write(MessageEv {
                    message: format!(
                        "Margin call for short position on {}! Increase the collateral to avoid liquidation.",
                        o.kind.lowername(),
                    ),
                    level: MessageLevel::Warning,
                });
            } else if price < margin * 0.8 && o.warning {
                o.warning = false; // Reset warning if price is below 80% of margin
            }
        });

        player.resolve_orders(&economy, &mut order_ev, &mut message);

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

            // Pay storage costs for commodities
            let storage_costs = player
                .instruments
                .iter()
                .map(|instrument| {
                    let cost =
                        instrument.amount as f32 * economy.get(&instrument.kind).storage_cost();
                    (instrument.kind.clone(), cost)
                })
                .collect::<Vec<_>>();

            for (instrument, cost) in storage_costs {
                if cost > 0. {
                    has_debt = true;
                }

                if player.cash.current() > cost {
                    player.cash.amount -= cost;
                } else {
                    has_paid = false;
                    message.write(MessageEv {
                        message: format!(
                            "Not enough cash to pay the storage costs for {}!",
                            instrument.lowername()
                        ),
                        level: MessageLevel::Error,
                    });
                    break;
                }
            }

            // Pay loan installments
            let mut cash = player.cash.amount;
            player.loans.retain_mut(|loan| {
                has_debt = true;

                let installment = loan.next_installment_amount();

                if installment > cash {
                    loan.defaults += 1;
                    has_paid = false;
                    message.write(MessageEv {
                        message: format!(
                            "Not enough cash to pay the installment on loan {}!",
                            loan.id
                        ),
                        level: MessageLevel::Error,
                    });
                } else {
                    cash -= loan.next_installment_amount();
                    loan.outstanding -= loan.next_principal_component();
                    loan.n_installments += 1;
                }

                loan.outstanding >= 1. // Keep loans that are not fully repaid
            });

            player.cash.amount = cash;

            // Pay interest on short positions
            let short_positions = player
                .instruments
                .iter()
                .filter(|o| o.amount < 0)
                .cloned()
                .collect::<Vec<_>>();

            for o in short_positions {
                let interest = o.interest / 100. / 12. * o.start_price * o.amount.abs() as f32;

                if player.cash.current() >= interest {
                    player.cash.amount -= interest;
                } else {
                    has_paid = false;
                    close_short_ev.write(CloseShortEv {
                        owned: o.clone(),
                        reason: "Not enough cash to pay the interest".to_string(),
                    });
                }
            }

            if has_debt {
                if has_paid {
                    player.credit_score.increase();
                } else {
                    player.credit_score.decrease();
                }
            }
        }

        // Warning messages =================================== >>

        // Check if player has enough cash to cover outflow next month
        if economy.date.day() == 20 && player.outflow(&economy) > player.cash.current() {
            message.write(MessageEv {
                message: "You're outflow for next month is larger than your cash reserve!"
                    .to_string(),
                level: MessageLevel::Warning,
            });
        }
    }
}

#[derive(Event)]
pub struct CloseShortEv {
    pub owned: OwnedInstrument,
    pub reason: String,
}

pub fn liquidate_short_positions(
    mut close_short_ev: EventReader<CloseShortEv>,
    economy: Res<GlobalEconomy>,
    mut player: ResMut<Player>,
    mut message: EventWriter<MessageEv>,
) {
    for CloseShortEv { owned, reason } in close_short_ev.read() {
        player.cash.amount +=
            owned.collateral - economy.get_current(&owned.kind) * -owned.amount as f32;
        player.instruments.retain(|o| o.kind != owned.kind);

        message.write(MessageEv {
            message: format!(
                "Forced liquidation on short position for {}. Reason: {reason}.",
                owned.kind.lowername()
            ),
            level: MessageLevel::Error,
        });
    }
}
