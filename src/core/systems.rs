use bevy::prelude::*;
use chrono::Datelike;

use crate::core::factors::Factor;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::bonds::BondKind;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::orders::OrderEv;
use crate::core::player::Player;

pub fn time_pass(
    mut economy: ResMut<GlobalEconomy>,
    mut player: ResMut<Player>,
    mut order_ev: EventWriter<OrderEv>,
    mut message: EventWriter<MessageEv>,
    time: Res<Time>,
) {
    economy.clock.tick(time.delta());

    if economy.clock.just_finished() {
        // Advance 1 day
        economy.date = economy.date.succ_opt().unwrap();

        // Daily operations =================================== >>

        let ev = player.enterprise_value(&economy);
        let (_, _, interest) = economy.bump(ev, &mut message);

        player.cash.bump(interest);

        let ev = player.enterprise_value(&economy);
        player.influence.bump(ev);

        // Update bounds for trailing orders
        for order in player.pending_orders_mut() {
            if order.lower_bound {
                order.bound = order.bound.min(economy.get_price(&order.instrument));
            } else {
                order.bound = order.bound.max(economy.get_price(&order.instrument));
            }
        }

        // Check margin call for margin loans
        let mut cash = player.cash.current();
        player.instruments.retain_mut(|owned| {
            if let Some(loan) = &mut owned.loan {
                let price = economy.get_price(&owned.kind);
                let margin = loan.margin(owned.amount);

                if owned.amount > 0 && price < margin {
                    // Liquidate long position
                    cash += price * owned.amount.abs() as f32 + loan.collateral - loan.debt;

                    message.write(MessageEv {
                        message: format!(
                            "Margin reached for long position on {}. Forced liquidation.",
                            owned.kind.lowername()
                        ),
                        level: MessageLevel::Error,
                    });

                    return false
                } else if owned.amount < 0 && price > margin {
                    // Liquidate short position
                    cash += loan.collateral + loan.debt - price * owned.amount.abs() as f32;

                    message.write(MessageEv {
                        message: format!(
                            "Margin reached for short position on {}. Forced liquidation.",
                            owned.kind.lowername()
                        ),
                        level: MessageLevel::Error,
                    });

                    return false
                } else if !owned.warning && (owned.amount > 0 && price < margin * 0.9) || (owned.amount < 0 && price > margin * 0.9) {
                    owned.warning = true;
                    message.write(MessageEv {
                        message: format!(
                            "Margin call for position on {}! Increase the collateral to avoid liquidation.",
                            owned.kind.lowername(),
                        ),
                        level: MessageLevel::Warning,
                    });
                } else if owned.warning && (owned.amount > 0 && price < margin * 0.8) || (owned.amount < 0 && price > margin * 0.8) {
                    owned.warning = false; // Reset warning if price is below 80% of the margin
                }
            }

            true
        });

        player.cash.amount = cash;

        player.resolve_orders(&economy, &mut order_ev, &mut message);

        if economy.date.day() == 1 {
            // Monthly operations =================================== >>

            // Trading volume is reset
            economy.economy.last_traded_volume = economy.economy.current_traded_volume;
            economy.economy.current_traded_volume = 0.;

            // Central bank calculates/pushes next interest rate
            let inflation = economy.inflation.current();
            economy.interest.resolve(inflation);

            // Interest on cash is paid
            player.cash.resolve();

            // Quarterly operations =================================== >>

            if economy.date.month() % 3 == 1 {
                // Dividends are paid out
                for owned in player.instruments.iter_mut() {
                    let instrument = economy.get(&owned.kind);

                    if instrument.dividend() > 0. {
                        todo!();
                    }
                }

                // Corporate bonds are issued
                for bond in &mut economy
                    .bonds
                    .iter_mut()
                    .filter(|b| b.kind() == BondKind::Corporate)
                {
                    bond.issue();
                }
            }

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
                    .filter(|b| b.kind() == BondKind::Government)
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
                    .filter(|b| b.kind() == BondKind::Corporate)
                {
                    bond.issue();
                }
            }

            // Resolve debts ======================================= >>

            let mut has_debt = false;
            let mut cash = player.cash.amount;

            // Pay storage costs for commodities
            let storage_costs = player
                .instruments
                .iter()
                .map(|o| o.amount as f32 * economy.get(&o.kind).storage_cost())
                .sum::<f32>();

            if storage_costs > 0. {
                has_debt = true;
                cash -= storage_costs;
            }

            // Pay term loan installments
            player.loans.retain_mut(|loan| {
                has_debt = true;
                cash -= loan.next_installment_amount();
                loan.outstanding -= loan.next_principal_component();
                loan.n_installments += 1;
                loan.outstanding >= 1. // Keep loans that are not fully repaid
            });

            if has_debt {
                if cash >= 0. {
                    player.credit_score.increase();
                } else {
                    player.credit_score.decrease();
                }
            }

            // Pay interest on margin loans
            for owned in &mut player.instruments {
                if let Some(loan) = &mut owned.loan {
                    let interest = loan.interest();

                    // Pay from cash if possible, else subtract from collateral
                    if cash >= interest {
                        cash -= interest;
                    } else {
                        loan.collateral -= interest;
                    }
                }
            }

            player.cash.amount = cash;
        }

        // Warning messages =================================== >>

        // Warn every month when cash is negative
        if economy.date.day() == 1 && player.cash.current() < 0. {
            message.write(MessageEv {
                message: "Your cash reserve is negative! Consider taking a loan or selling assets."
                    .to_string(),
                level: MessageLevel::Error,
            });
        }

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
