use std::collections::HashMap;

use bevy::prelude::*;
use chrono::{Datelike, Duration};
use rand::{Rng, rng};

use crate::core::audio::PlayAudioEv;
use crate::core::constants::{CURRENCY, DATE_FORMAT, DAYS_PER_EVENT, VICTORY_AMOUNT};
use crate::core::derivatives::{DerivativeAction, DerivativeKind, OptionKind};
use crate::core::events::EventName;
use crate::core::factors::Factor;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::bonds::{Bond, BondIssuer};
use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::core::instruments::stocks::Company;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::orders::OrderEv;
use crate::core::player::Player;
use crate::core::research::TechName;
use crate::core::states::GameState;
use crate::core::ui::state::UiState;
use crate::utils::{EnhFloat, NameFromEnum};

pub fn time_pass(
    mut economy: ResMut<GlobalEconomy>,
    mut player: ResMut<Player>,
    mut state: ResMut<UiState>,
    mut order_ev: EventWriter<OrderEv>,
    mut message: EventWriter<MessageEv>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut play_audio_ev: EventWriter<PlayAudioEv>,
    time: Res<Time>,
) {
    economy.clock.tick(time.delta());

    let aum = player.aum(&economy);
    if aum <= 0. || (aum >= VICTORY_AMOUNT && !player.has_continued) {
        next_game_state.set(GameState::GameEnd);
        return;
    }

    if economy.clock.just_finished() {
        // Advance 1 day
        let today = economy.date.succ_opt().unwrap();
        economy.date = today;

        // Daily operations =================================== >>

        player.research.advance(&mut message);
        player.cash.amount -= player.research.costs();

        let aum = player.aum(&economy);
        let (_, inflation, interest) =
            economy.bump(aum, &mut state, &mut player, &mut order_ev, &mut message);

        player.cash.bump(interest);

        let has_tech = player.has_tech(&TechName::BackdoorPolitics);
        player.influence.bump(aum, has_tech);

        // Increase storage costs and dividends with inflation
        for stock in &mut economy.stocks {
            stock.dividend *= 1. + inflation / 100. / 365.;
        }

        for commodity in &mut economy.commodities {
            commodity.storage_cost *= 1. + inflation / 100. / 365.;
        }

        // Update bounds for trailing orders
        for order in player.pending_orders_mut() {
            if order.lower_bound {
                order.bound = order.bound.min(economy.get_price(&order.instrument));
            } else {
                order.bound = order.bound.max(economy.get_price(&order.instrument));
            }
        }

        // Start new events
        if rng().random::<f32>() < 1. / DAYS_PER_EVENT
        // && economy.date > START_DATE + Duration::days(30)
        {
            let new_event = EventName::create(&economy, &player);
            new_event.immediate(&mut economy);

            state.active_event = Some(new_event.name.clone());
            economy.events.push(new_event);

            play_audio_ev.write(PlayAudioEv::new("message"));
            next_game_state.set(GameState::Paused);
        }

        // Advance all events
        let events = economy.active_events().into_iter().cloned().collect::<Vec<_>>();

        for event in events {
            event.advance(&mut economy); // todo!
        }

        let mut cash = player.cash.current();

        // Check the maturity of bonds
        player.instruments.retain(|owned| {
            if let InstrumentKind::Bond(issuer) = owned.kind {
                if owned.maturity_date == economy.date {
                    // Bond matured, pay out face value
                    match issuer {
                        BondIssuer::Government(country) => {
                            let currency =
                                economy.currencies.iter().find(|c| c.country == country).unwrap();

                            cash += Bond::FACE_VALUE_GOVERNMENT
                                * owned.amount as f32
                                * currency.current();
                        },
                        BondIssuer::Corporate(_) => {
                            cash += Bond::FACE_VALUE_CORPORATE * owned.amount as f32;
                        },
                    }

                    message.write(MessageEv {
                        message: format!(
                            "Maturity reached for {} {}. Face value returned.",
                            owned.amount,
                            owned.kind.lowername(),
                        ),
                        level: MessageLevel::Info,
                    });

                    return false;
                }
            }

            true
        });

        // Check margin call for margin loans
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
                }
            }

            true
        });

        player.cash.amount = cash;

        player.resolve_orders(&economy, &mut order_ev, &mut message);

        if economy.date.day() == 1 {
            // Monthly operations =================================== >>

            economy.economy.reset_traded_volume();

            // Central bank calculates/pushes next interest rate
            let inflation = economy.inflation.current();
            economy.interest.resolve(inflation);

            // Interest on cash is paid
            player.cash.resolve();

            // Quarterly operations =================================== >>

            // Stock dividends are paid quarterly
            if economy.date.month() % 3 == 1 {
                // Calculate the exact dividends
                let dividends: HashMap<Company, f32> = economy
                    .stocks
                    .iter()
                    .map(|stock| {
                        (stock.issuer, stock.dividend() * (1. + rng().random_range(-0.2..0.2)))
                    })
                    .collect();

                let total_dividends = player
                    .stocks()
                    .iter()
                    .map(|owned| {
                        if let InstrumentKind::Stock(company) = &owned.kind {
                            let dividend = dividends.get(&company).unwrap();

                            if *dividend == 0. {
                                message.write(MessageEv {
                                    message: format!(
                                        "Company {} is not paying out any dividend.",
                                        company.to_name(),
                                    ),
                                    level: MessageLevel::Warning,
                                });
                            }

                            dividend * owned.amount as f32
                        } else {
                            return 0.;
                        }
                    })
                    .sum::<f32>();

                if total_dividends > 0. {
                    player.cash.amount += total_dividends;

                    message.write(MessageEv {
                        message: format!(
                            "You received {}{CURRENCY} on dividend payments.",
                            total_dividends.clean(),
                        ),
                        level: MessageLevel::Info,
                    });
                }
            }

            // Bi-yearly operations =================================== >>

            // Bond's interests are paid twice a year
            if economy.date.month() % 6 == 1 {
                // Calculate which bonds default this round
                let defaults: HashMap<BondIssuer, bool> = economy
                    .bonds
                    .iter()
                    .map(|b| {
                        (b.issuer.clone(), rng().random::<f32>() < b.quality().default_chance())
                    })
                    .collect();

                let total_coupons = player
                    .bonds()
                    .iter()
                    .map(|owned| {
                        if let InstrumentKind::Bond(issuer) = &owned.kind {
                            // If the bond has CDS, it never defaults
                            if !owned.cds && *defaults.get(&issuer).unwrap() {
                                message.write(MessageEv {
                                    message: format!(
                                        "Bond issuer {} is defaulting on its coupon payment.",
                                        issuer.to_name(),
                                    ),
                                    level: MessageLevel::Warning,
                                });

                                0.
                            } else {
                                issuer.coupon_payment(owned.interest, owned.cds, &economy)
                                    * owned.amount as f32
                            }
                        } else {
                            0.
                        }
                    })
                    .sum::<f32>();

                if total_coupons > 0. {
                    player.cash.amount += total_coupons;

                    message.write(MessageEv {
                        message: format!(
                            "You received {}{CURRENCY} on coupon payments.",
                            total_coupons.clean(),
                        ),
                        level: MessageLevel::Info,
                    });
                }
            }

            // Resolve debts ======================================= >>

            let mut has_debt = false;
            let mut cash = player.cash.current();

            // Pay storage costs for commodities
            let storage_costs = player
                .instruments
                .iter()
                .map(|o| o.amount as f32 * economy.get(&o.kind).storage_cost(&player))
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
                    let has_tech = player.has_tech(&TechName::TrustworthyBorrower);
                    player.credit_score.increase(has_tech);
                } else {
                    let has_tech = player.has_tech(&TechName::SecureBased);
                    player.credit_score.decrease(has_tech);
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

        // Update derivatives ====================================== >>

        // Update execution for options
        let mut status = vec![];
        for option in player.pending_derivatives() {
            let mut execute = option.execute;

            if option.kind == DerivativeKind::Option {
                // Only adjust automatically when execute not changed by the player
                if !option.force_execute {
                    let market_price = economy.get_price(&option.instrument);

                    execute = if option.is_buy() {
                        market_price > option.price
                    } else {
                        market_price < option.price
                    };
                }

                // Always disable bought options when there is no cash or instruments to cover
                if option.action == DerivativeAction::Bought
                    && ((option.option_kind == OptionKind::Call
                        && player.cash.current() < option.price * option.amount as f32)
                        || (option.option_kind == OptionKind::Put
                            && player.get_owned(&option.instrument) < option.amount as i32))
                {
                    execute = false;
                }
            }

            status.push(execute);
        }

        // Assign execution status to derivatives
        for (option, status) in player.pending_derivatives_mut().into_iter().zip(status.into_iter())
        {
            option.execute = status;
        }

        player.resolve_derivatives(&mut economy, &mut message);

        // Warning messages ======================================== >>

        // Warn every month when cash is negative
        if economy.date.day() == 1 && player.cash.current() < 0. {
            message.write(MessageEv {
                message: "Your cash reserve is negative! Consider taking a loan or selling assets."
                    .to_string(),
                level: MessageLevel::Error,
            });
        }

        // Check if the player has enough cash to cover outflow next month
        if economy.date.day() == 20 && player.outflow(&economy) > player.cash.current() {
            message.write(MessageEv {
                message: "You're outflow for next month is larger than your cash reserve!"
                    .to_string(),
                level: MessageLevel::Warning,
            });
        }

        // Warn if the player hasn't enough cash or instruments to cover a derivative in 20 days
        for derivative in player.pending_derivatives() {
            if derivative.maturity_date() == economy.date + Duration::days(20) {
                let total_price = derivative.price * derivative.amount as f32;

                // Buy requires cash and sell requires instruments
                let no_buy = derivative.kind == DerivativeKind::Option
                    && derivative.is_buy()
                    && player.cash.current() < total_price;

                let no_sell = derivative.is_sell()
                    && player
                        .instruments
                        .iter()
                        .filter(|o| o.kind == derivative.instrument)
                        .count()
                        < derivative.amount as usize;

                if no_buy || no_sell {
                    message.write(MessageEv {
                        message: format!(
                            "Not enough {} to {} {}{} on {}.",
                            if no_buy {
                                "cash".to_string()
                            } else {
                                derivative.instrument.lowername()
                            },
                            if derivative.action == DerivativeAction::Bought {
                                "execute"
                            } else {
                                "cover"
                            },
                            if derivative.kind == DerivativeKind::Future {
                                "".to_string()
                            } else {
                                format!("{} ", derivative.option_kind.to_lowername())
                            },
                            derivative.kind.to_lowername(),
                            derivative.maturity_date().format(DATE_FORMAT),
                        ),
                        level: MessageLevel::Warning,
                    });
                }
            }
        }
    }
}
