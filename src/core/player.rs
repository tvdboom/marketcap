use std::collections::HashMap;

use bevy::prelude::*;
use chrono::Datelike;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::core::derivatives::{Derivative, DerivativeKind};
use crate::core::factors::Factor;
use crate::core::factors::cash::Cash;
use crate::core::factors::credit_score::CreditScore;
use crate::core::factors::influence::Influence;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::bonds::{Bond, BondIssuer};
use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::core::instruments::stocks::Company;
use crate::core::loans::{MarginLoan, TermLoan};
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::orders::{Command, Order, OrderEv, OrderKind, OrderStatus};
use crate::utils::NameFromEnum;

#[derive(Clone, Serialize, Deserialize)]
pub struct OwnedInstrument {
    pub kind: InstrumentKind,
    pub amount: i32,
    pub loan: Option<MarginLoan>,
    pub interest: f32,
    pub warning: bool,
}

impl Default for OwnedInstrument {
    fn default() -> Self {
        OwnedInstrument {
            kind: InstrumentKind::Stock(Company::Apple),
            amount: 0,
            loan: None,
            interest: 0.,
            warning: false,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OwnedBond {
    pub issuer: BondIssuer,
    pub amount: u32,
    pub loan: Option<MarginLoan>,
    pub interest: f32,
    pub warning: bool,
}

#[derive(Resource, Clone, Default, Serialize, Deserialize)]
pub struct Player {
    pub cash: Cash,
    pub credit_score: CreditScore,
    pub influence: Influence,
    pub loans: Vec<TermLoan>,
    pub orders: Vec<Order>,
    pub derivatives: Vec<Derivative>,
    pub instruments: Vec<OwnedInstrument>,
    pub favourites: HashMap<u8, InstrumentKind>,
}

impl Player {
    // Factors ===================================================== >>

    pub fn term_loan_debt(&self) -> f32 {
        self.loans.iter().map(|l| l.outstanding).sum()
    }

    pub fn margin_loan_debt(&self) -> f32 {
        self.instruments
            .iter()
            .filter_map(|o| o.loan.as_ref().map(|l| l.debt - l.collateral))
            .sum()
    }

    pub fn aum(&self, economy: &GlobalEconomy) -> f32 {
        self.cash.amount
            + self
                .instruments
                .iter()
                .map(|o| o.amount as f32 * economy.get(&o.kind).current())
                .sum::<f32>()
            - self.term_loan_debt()
            - self.margin_loan_debt()
    }

    pub fn dividend_payment(&self, economy: &GlobalEconomy) -> f32 {
        self.stocks()
            .iter()
            .map(|owned| {
                let instrument = economy.get(&owned.kind);
                instrument.dividend() * owned.amount as f32
            })
            .sum()
    }

    pub fn coupon_payment(&self, economy: &GlobalEconomy) -> f32 {
        // Multiply by 0.5 since coupon is paid out twice a year
        self.bonds()
            .iter()
            .map(|owned| {
                if let InstrumentKind::Bond(issuer) = &owned.kind {
                    match issuer {
                        BondIssuer::Government(country) => {
                            let currency = economy
                                .currencies
                                .iter()
                                .find(|c| c.country == *country)
                                .unwrap();

                            Bond::DEFAULT_GOVERNMENT * 0.5 * owned.interest / 100.
                                * owned.amount as f32
                                * currency.current()
                        },
                        BondIssuer::Corporate(_) => {
                            Bond::DEFAULT_CORPORATE * 0.5 * owned.interest / 100.
                                * owned.amount as f32
                        },
                    }
                } else {
                    0.
                }
            })
            .sum::<f32>()
    }

    pub fn inflow(&self, economy: &GlobalEconomy) -> f32 {
        let mut inflow = self.cash.accumulated_interest;

        // Dividend is paid out quarterly, so inflow must be shown one month prior
        if economy.date.month() % 3 == 1 {
            inflow += self.dividend_payment(economy);
        }

        // Bonds are paid out semi-annually, so inflow must be shown one month prior
        if economy.date.month() % 6 == 1 {
            inflow += self.coupon_payment(economy);
        }

        inflow
    }

    pub fn storage_costs(&self, economy: &GlobalEconomy) -> f32 {
        self.instruments
            .iter()
            .map(|o| (o.amount as f32 * economy.get(&o.kind).storage_cost()).max(0.))
            .sum()
    }

    pub fn loan_installments(&self) -> f32 {
        self.loans.iter().map(|l| l.next_installment_amount()).sum()
    }

    pub fn short_sell_interest(&self) -> f32 {
        self.instruments
            .iter()
            .filter_map(|o| o.loan.as_ref().map(|l| l.interest()))
            .sum()
    }

    pub fn outflow(&self, economy: &GlobalEconomy) -> f32 {
        self.storage_costs(economy) + self.loan_installments() + self.short_sell_interest()
    }

    pub fn netflow(&self, economy: &GlobalEconomy) -> f32 {
        self.inflow(economy) - self.outflow(economy)
    }

    // Instruments ================================================= >>

    pub fn has_favourite(&self, instrument: &InstrumentKind) -> bool {
        self.favourites.values().contains(&instrument)
    }

    pub fn stocks(&self) -> Vec<&OwnedInstrument> {
        self.instruments
            .iter()
            .filter(|o| matches!(o.kind, InstrumentKind::Stock(_)))
            .collect::<Vec<_>>()
    }

    pub fn bonds(&self) -> Vec<&OwnedInstrument> {
        self.instruments
            .iter()
            .filter(|o| matches!(o.kind, InstrumentKind::Bond(_)))
            .collect::<Vec<_>>()
    }

    pub fn commodities(&self) -> Vec<&OwnedInstrument> {
        self.instruments
            .iter()
            .filter(|o| matches!(o.kind, InstrumentKind::Commodity(_)))
            .collect::<Vec<_>>()
    }

    pub fn crypto(&self) -> Vec<&OwnedInstrument> {
        self.instruments
            .iter()
            .filter(|o| matches!(o.kind, InstrumentKind::Crypto(_)))
            .collect::<Vec<_>>()
    }

    pub fn get(&mut self, kind: &InstrumentKind) -> Option<&OwnedInstrument> {
        self.instruments.iter().find(|c| c.kind == *kind)
    }

    pub fn get_mut(&mut self, kind: &InstrumentKind) -> Option<&mut OwnedInstrument> {
        self.instruments.iter_mut().find(|c| c.kind == *kind)
    }

    pub fn get_owned(&self, instrument: &InstrumentKind) -> i32 {
        self.instruments
            .iter()
            .find(|c| c.kind == *instrument)
            .map(|c| c.amount)
            .unwrap_or_default()
    }

    pub fn get_value(&self, instrument: &InstrumentKind, economy: &GlobalEconomy) -> f32 {
        self.get_owned(instrument) as f32 * economy.get_price(instrument)
    }

    // Orders ====================================================== >>

    pub fn pending_orders(&self) -> Vec<&Order> {
        self.orders
            .iter()
            .filter(|o| o.status == OrderStatus::Pending)
            .collect::<Vec<_>>()
    }

    pub fn pending_orders_mut(&mut self) -> Vec<&mut Order> {
        self.orders
            .iter_mut()
            .filter(|o| o.status == OrderStatus::Pending)
            .collect::<Vec<_>>()
    }

    pub fn processed_orders(&self) -> Vec<&Order> {
        self.orders
            .iter()
            .filter(|o| o.status != OrderStatus::Pending)
            .collect::<Vec<_>>()
    }

    pub fn pending_derivatives(&self) -> Vec<&Derivative> {
        self.derivatives
            .iter()
            .filter(|d| d.status == OrderStatus::Pending)
            .collect::<Vec<_>>()
    }

    pub fn pending_derivatives_mut(&mut self) -> Vec<&mut Derivative> {
        self.derivatives
            .iter_mut()
            .filter(|d| d.status == OrderStatus::Pending)
            .collect::<Vec<_>>()
    }

    pub fn processed_derivatives(&self) -> Vec<&Derivative> {
        self.derivatives
            .iter()
            .filter(|d| d.status != OrderStatus::Pending)
            .collect::<Vec<_>>()
    }

    pub fn resolve_orders(
        &mut self,
        economy: &GlobalEconomy,
        order_ev: &mut EventWriter<OrderEv>,
        message: &mut EventWriter<MessageEv>,
    ) {
        let mut processed = vec![];
        for order in self.pending_orders() {
            let instrument = economy.get(&order.instrument);
            let owned = self.get_owned(&order.instrument);
            let price = (instrument.current()
                - if order.command == Command::Buy {
                    0.
                } else {
                    instrument.storage_cost() * 30.
                })
                * order.amount as f32;

            let condition = match order.kind {
                OrderKind::LimitOrder => {
                    if order.lower_bound {
                        instrument.current() <= order.limit_price()
                    } else {
                        instrument.current() >= order.limit_price()
                    }
                },
                OrderKind::TrailingOrder => {
                    if order.lower_bound {
                        instrument.current() >= order.limit_price()
                    } else {
                        instrument.current() <= order.limit_price()
                    }
                },
                _ => unreachable!(),
            };

            if condition {
                let status = match order.command {
                    Command::Buy => {
                        if self.cash.current() >= price {
                            OrderStatus::Executed
                        } else {
                            OrderStatus::Failed("not enough cash".to_string())
                        }
                    },
                    Command::Sell => {
                        if owned >= order.amount {
                            OrderStatus::Executed
                        } else {
                            OrderStatus::Failed("insufficient owned".to_string())
                        }
                    },
                    Command::Close => {
                        if owned > 0 {
                            OrderStatus::Executed
                        } else {
                            OrderStatus::Failed("none owned".to_string())
                        }
                    },
                };

                processed.push((order.id.clone(), price, status));
            }
        }

        for (id, price, status) in processed {
            let order = self.orders.iter_mut().find(|o| o.id == id).unwrap();

            if let OrderStatus::Failed(msg) = &status {
                message.write(MessageEv {
                    message: format!("Failed to execute order {id}: {msg}."),
                    level: MessageLevel::Warning,
                });
            } else {
                order_ev.write(OrderEv {
                    id: order.id.clone(),
                    price,
                });
            }

            order.processed = economy.date;
            order.price = price;
            order.status = status;
        }
    }

    pub fn resolve_derivatives(
        &mut self,
        economy: &mut GlobalEconomy,
        message: &mut EventWriter<MessageEv>,
    ) {
        let pending = self
            .pending_derivatives()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();

        for derivative in pending {
            if economy.date == derivative.maturity_date() {
                if derivative.execute {
                    let total_price = derivative.price * derivative.amount as f32;

                    economy.economy.current_traded_volume += total_price;

                    if derivative.is_buy() {
                        // Futures are already paid for
                        if derivative.kind == DerivativeKind::Option {
                            self.cash.amount -= total_price;
                            if self.cash.amount < 0. {
                                self.credit_score.decrease();
                                message.write(MessageEv {
                                    message: "Forced buy to cover option. Credit score reduced."
                                        .to_string(),
                                    level: MessageLevel::Error,
                                });
                            }
                        }

                        if let Some(owned) = self.get_mut(&derivative.instrument) {
                            owned.amount += derivative.amount as i32;
                        } else {
                            self.instruments.push(OwnedInstrument {
                                kind: derivative.instrument.clone(),
                                amount: derivative.amount as i32,
                                ..default()
                            });
                        }

                        message.write(MessageEv {
                            message: format!(
                                "Executed {}{}. Bought {} {}.",
                                if derivative.kind == DerivativeKind::Option {
                                    format!("{} ", derivative.option_kind.to_lowername())
                                } else {
                                    "".to_string()
                                },
                                derivative.kind.to_lowername(),
                                derivative.amount,
                                derivative.instrument.lowername(),
                            ),
                            level: MessageLevel::Info,
                        });
                    } else {
                        if derivative.kind == DerivativeKind::Option {
                            self.cash.amount += total_price;
                        }

                        let remaining = if let Some(owned) = self.get_mut(&derivative.instrument) {
                            if owned.amount >= derivative.amount as i32 {
                                owned.amount -= derivative.amount as i32;
                                0
                            } else if owned.amount > 0 {
                                let remaining = derivative.amount - owned.amount as u32;
                                owned.amount = 0;
                                remaining
                            } else {
                                derivative.amount
                            }
                        } else {
                            derivative.amount
                        };

                        if remaining > 0 {
                            // Not sufficient instruments owned to cover the derivative
                            // Buy the remaining amount at market price
                            self.cash.amount -=
                                economy.get_price(&derivative.instrument) * remaining as f32;
                            self.credit_score.decrease();

                            message.write(MessageEv {
                                message: format!(
                                    "Executed {} sell for {} {}. Insufficient amount owned.",
                                    derivative.kind.to_lowername(),
                                    derivative.amount,
                                    derivative.instrument.lowername(),
                                ),
                                level: MessageLevel::Error,
                            });
                        } else {
                            message.write(MessageEv {
                                message: format!(
                                    "Executed {} sell for {} {}.",
                                    derivative.kind.to_lowername(),
                                    derivative.amount,
                                    derivative.instrument.lowername(),
                                ),
                                level: MessageLevel::Info,
                            });
                        }
                    }
                } else {
                    message.write(MessageEv {
                        message: format!(
                            "Option for {} {} matured without execution.",
                            derivative.amount,
                            derivative.instrument.lowername()
                        ),
                        level: MessageLevel::Info,
                    });
                }
            }
        }

        // Second pass to update the attributes
        for derivative in self.pending_derivatives_mut() {
            if economy.date == derivative.maturity_date() {
                derivative.transaction_price = economy.get_price(&derivative.instrument);
                derivative.status = if derivative.execute {
                    OrderStatus::Executed
                } else {
                    OrderStatus::Canceled
                }
            }
        }
    }
}
