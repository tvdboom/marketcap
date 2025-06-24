use std::collections::HashMap;

use bevy::prelude::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::core::factors::Factor;
use crate::core::factors::cash::Cash;
use crate::core::factors::credit_score::CreditScore;
use crate::core::factors::influence::Influence;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::instrument::InstrumentKind;
use crate::core::loans::{MarginLoan, TermLoan};
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::orders::{Command, Order, OrderEv, OrderKind, OrderStatus};

#[derive(Clone, Serialize, Deserialize)]
pub struct OwnedInstrument {
    pub kind: InstrumentKind,
    pub amount: i32,
    pub loan: Option<MarginLoan>,
    pub warning: bool,
}

#[derive(Resource, Clone, Default, Serialize, Deserialize)]
pub struct Player {
    pub cash: Cash,
    pub credit_score: CreditScore,
    pub influence: Influence,
    pub loans: Vec<TermLoan>,
    pub orders: Vec<Order>,
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

    pub fn enterprise_value(&self, economy: &GlobalEconomy) -> f32 {
        self.cash.amount
            + self
                .instruments
                .iter()
                .map(|o| o.amount as f32 * economy.get(&o.kind).current())
                .sum::<f32>()
            - self.term_loan_debt()
            - self.margin_loan_debt()
    }

    pub fn inflow(&self) -> f32 {
        self.cash.accumulated_interest
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
        self.inflow() - self.outflow(economy)
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
                        instrument.current() >= order.threshold
                    } else {
                        instrument.current() <= order.threshold
                    }
                },
                OrderKind::TrailingOrder => {
                    if order.lower_bound {
                        instrument.current() >= (1. + order.threshold / 100.) * order.bound
                    } else {
                        instrument.current() <= (1. - order.threshold / 100.) * order.bound
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
}
