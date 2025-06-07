use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::core::factors::Factor;
use crate::core::factors::cash::Cash;
use crate::core::factors::credit_score::CreditScore;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::bonds::BondName;
use crate::core::instruments::commodities::CommodityName;
use crate::core::loans::Loan;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::utils::NameFromEnum;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum InstrumentKind {
    Bond(BondName),
    Commodity(CommodityName),
}

impl InstrumentKind {
    pub fn name(&self) -> String {
        match self {
            InstrumentKind::Bond(name) => name.to_name(),
            InstrumentKind::Commodity(name) => name.to_name(),
        }
    }

    pub fn lowername(&self) -> String {
        match self {
            InstrumentKind::Bond(name) => name.to_lowername(),
            InstrumentKind::Commodity(name) => name.to_lowername(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OwnedInstrument {
    pub kind: InstrumentKind,
    pub amount: u32,
    pub interest: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Order {
    Buy,
    Sell,
    Close,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TradeOrder {
    pub id: String,
    pub instrument: InstrumentKind,
    pub order: Order,
    pub amount: u32,
    pub price: u32,
}

#[derive(Resource, Clone, Default, Serialize, Deserialize)]
pub struct Player {
    pub cash: Cash,
    pub credit_score: CreditScore,
    pub loans: Vec<Loan>,
    pub orders: Vec<TradeOrder>,
    pub instruments: Vec<OwnedInstrument>,
}

impl Player {
    pub fn enterprise_value(&self, economy: &GlobalEconomy) -> f32 {
        self.cash.amount
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
            .map(|o| o.amount as f32 * economy.get(&o.kind).storage_cost())
            .sum()
    }

    pub fn loan_installments(&self) -> f32 {
        self.loans.iter().map(|l| l.next_installment_amount()).sum()
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
            has_debt = true;

            let storage_cost =
                instrument.amount as f32 * economy.get(&instrument.kind).storage_cost();

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

    pub fn get_owned(&self, instrument: &InstrumentKind) -> u32 {
        self.instruments
            .iter()
            .find(|c| c.kind == *instrument)
            .map(|c| c.amount)
            .unwrap_or_default()
    }

    pub fn get_value(&self, instrument: &InstrumentKind, economy: &GlobalEconomy) -> f32 {
        self.get_owned(instrument) as f32 * economy.get_current(instrument)
    }

    pub fn buy(&mut self, instrument: &InstrumentKind, amount: u32, price: f32) {
        if let Some(owned) = self.instruments.iter_mut().find(|o| o.kind == *instrument) {
            owned.amount += amount;
        } else {
            self.instruments.push(OwnedInstrument {
                kind: instrument.clone(),
                amount,
                interest: 0.,
            });
        }

        self.cash.amount -= price;
    }

    pub fn sell(&mut self, instrument: &InstrumentKind, amount: u32, price: f32) {
        self.instruments.retain_mut(|o| {
            if o.kind == *instrument {
                o.amount = o.amount.saturating_sub(amount);
            }
            o.amount > 0
        });

        self.cash.amount += price;
    }

    pub fn close(&mut self, instrument: &InstrumentKind, price: f32) {
        self.cash.amount += price;
        self.instruments.retain(|s| s.kind != *instrument);
    }

    pub fn execute_orders(
        &mut self,
        economy: &GlobalEconomy,
        message: &mut EventWriter<MessageEv>,
    ) {
        let mut to_drop = vec![];

        for order in self.orders.clone() {
            let instrument = economy.get(&order.instrument);
            let owned = self.get_owned(&order.instrument);
            let price = instrument.current() * order.amount as f32;

            match order.order {
                Order::Buy => {
                    if instrument.current() <= order.price as f32 {
                        if self.cash.current() >= price {
                            self.buy(&order.instrument, order.amount, price);

                            message.write(MessageEv {
                                message: format!(
                                    "Executed order {}: bought {} {}.",
                                    order.id,
                                    order.amount,
                                    order.instrument.name()
                                ),
                                level: MessageLevel::Info,
                            });
                        } else {
                            message.write(MessageEv {
                                message: format!(
                                    "Failed to execute order {}: lack of cash.",
                                    order.id
                                ),
                                level: MessageLevel::Warning,
                            });
                        }

                        to_drop.push(order.id);
                    }
                },
                Order::Sell => {
                    if instrument.current() >= order.price as f32 {
                        if owned >= order.amount {
                            self.sell(&order.instrument, order.amount, price);

                            message.write(MessageEv {
                                message: format!(
                                    "Executed order {}: sold {} {}.",
                                    order.id,
                                    order.amount,
                                    order.instrument.name()
                                ),
                                level: MessageLevel::Info,
                            });
                        } else {
                            message.write(MessageEv {
                                message: format!(
                                    "Failed to execute order {}: insufficient amount owned.",
                                    order.id
                                ),
                                level: MessageLevel::Warning,
                            });
                        }

                        to_drop.push(order.id);
                    }
                },
                Order::Close => {
                    if instrument.current() >= order.price as f32 {
                        if owned > 0 {
                            self.close(&order.instrument, instrument.current() * owned as f32);

                            message.write(MessageEv {
                                message: format!(
                                    "Executed order {}: closed {} position.",
                                    order.id,
                                    order.instrument.name(),
                                ),
                                level: MessageLevel::Info,
                            });
                        } else {
                            message.write(MessageEv {
                                message: format!(
                                    "Failed to execute order {}: no {} owned.",
                                    order.id,
                                    order.instrument.name()
                                ),
                                level: MessageLevel::Warning,
                            });
                        }

                        to_drop.push(order.id);
                    }
                },
            }
        }

        self.orders.retain(|o| !to_drop.contains(&o.id));
    }
}
