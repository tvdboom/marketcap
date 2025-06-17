use crate::core::factors::Factor;
use crate::core::factors::cash::Cash;
use crate::core::factors::credit_score::CreditScore;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::bonds::BondIssuer;
use crate::core::instruments::commodities::CommodityName;
use crate::core::instruments::crypto::CryptoName;
use crate::core::loans::Loan;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::orders::{Command, Order, OrderEv, OrderKind, OrderStatus};
use crate::utils::NameFromEnum;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum InstrumentKind {
    Bond(BondIssuer),
    Commodity(CommodityName),
    Crypto(CryptoName),
}

impl InstrumentKind {
    pub fn name(&self) -> String {
        match self {
            InstrumentKind::Bond(name) => name.to_name(),
            InstrumentKind::Commodity(name) => name.to_name(),
            InstrumentKind::Crypto(name) => name.to_name(),
        }
    }

    pub fn lowername(&self) -> String {
        match self {
            InstrumentKind::Bond(name) => name.to_lowername(),
            InstrumentKind::Commodity(name) => name.to_lowername(),
            InstrumentKind::Crypto(name) => name.to_lowername(),
        }
    }

    pub fn order_options(&self) -> Vec<OrderKind> {
        match self {
            InstrumentKind::Commodity(_) => vec![
                OrderKind::MarketOrder,
                OrderKind::LimitOrder,
                OrderKind::TrailingOrder,
                OrderKind::ShortSelling,
            ],
            InstrumentKind::Crypto(_) => vec![
                OrderKind::MarketOrder,
                OrderKind::LimitOrder,
                OrderKind::TrailingOrder,
            ],
            _ => OrderKind::iter().collect(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OwnedInstrument {
    pub kind: InstrumentKind,
    pub amount: i32,
    pub interest: f32,
}

#[derive(Resource, Clone, Default, Serialize, Deserialize)]
pub struct Player {
    pub cash: Cash,
    pub collateral: f32,
    pub credit_score: CreditScore,
    pub loans: Vec<Loan>,
    pub orders: Vec<Order>,
    pub instruments: Vec<OwnedInstrument>,
}

impl Player {
    // Factors ===================================================== >>

    pub fn enterprise_value(&self, economy: &GlobalEconomy) -> f32 {
        self.cash.amount
            + self.collateral
            + self
                .instruments
                .iter()
                .map(|o| o.amount as f32 * economy.get(&o.kind).current())
                .sum::<f32>()
            - self.loans.iter().map(|l| l.outstanding).sum::<f32>()
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

    pub fn short_positions(&self) -> f32 {
        self.instruments
            .iter()
            .filter_map(|o| (o.amount < 0). then_some(o.interest))
            .sum()
    }
    
    pub fn outflow(&self, economy: &GlobalEconomy) -> f32 {
        self.storage_costs(economy) + self.loan_installments()
    }

    pub fn netflow(&self, economy: &GlobalEconomy) -> f32 {
        self.inflow() - self.outflow(economy)
    }

    pub fn resolve_debts(&mut self, economy: &GlobalEconomy) -> bool {
        let mut has_debt = false;
        let mut has_paid = true;

        for instrument in self.instruments.iter() {
            let storage_cost =
                instrument.amount as f32 * economy.get(&instrument.kind).storage_cost();

            if storage_cost > 0. {
                has_debt = true;
            }

            if self.cash.current() > storage_cost {
                self.cash.amount -= storage_cost;
            } else {
                has_paid = false;
                break;
            }
        }

        self.loans.retain_mut(|loan| {
            has_debt = true;

            let installment = loan.next_installment_amount();

            if installment > self.cash.current() {
                loan.defaults += 1;
                has_paid = false;
            } else {
                self.cash.amount -= loan.next_installment_amount();
                loan.outstanding -= loan.next_principal_component();
                loan.n_installments += 1;
            }

            loan.outstanding >= 1. // Keep loans that are not fully repaid
        });

        if has_debt {
            if has_paid {
                self.credit_score.score = (self.credit_score.score + 1).min(CreditScore::MAX);
            } else {
                self.credit_score.score = self
                    .credit_score
                    .score
                    .saturating_sub(12)
                    .max(CreditScore::MIN);
            }
        }

        has_paid
    }

    // Instruments ================================================= >>

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
        self.get_owned(instrument) as f32 * economy.get_current(instrument)
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
                        instrument.current() >= order.threshold as f32
                    } else {
                        instrument.current() <= order.threshold as f32
                    }
                },
                OrderKind::TrailingOrder => {
                    if order.lower_bound {
                        instrument.current() >= (1. + order.threshold as f32 / 100.) * order.bound
                    } else {
                        instrument.current() <= (1. - order.threshold as f32 / 100.) * order.bound
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
