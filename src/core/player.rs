use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::core::factors::Factor;
use crate::core::factors::cash::Cash;
use crate::core::factors::credit_score::CreditScore;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::commodities::CommodityName;
use crate::core::loans::Loan;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum InstrumentKind {
    Commodity(CommodityName),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OwnedInstrument {
    pub kind: InstrumentKind,
    pub amount: u32,
    pub interest: f32,
}

#[derive(Resource, Clone, Default, Serialize, Deserialize)]
pub struct Player {
    pub cash: Cash,
    pub credit_score: CreditScore,
    pub loans: Vec<Loan>,
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

    pub fn outflow(&self, economy: &GlobalEconomy) -> f32 {
        let storage_costs: f32 = self
            .instruments
            .iter()
            .map(|o| o.amount as f32 * economy.get(&o.kind).storage_cost())
            .sum();

        let loan_payments: f32 = self.loans.iter().map(|l| l.next_installment_amount()).sum();

        storage_costs + loan_payments
    }

    pub fn netflow(&self, economy: &GlobalEconomy) -> f32 {
        self.inflow() - self.outflow(economy)
    }

    pub fn resolve_debts(&mut self, economy: &GlobalEconomy) -> bool {
        let mut success = true;

        for instrument in self.instruments.iter() {
            let storage_cost =
                instrument.amount as f32 * economy.get(&instrument.kind).storage_cost();

            if self.cash.current() > storage_cost {
                self.cash.amount -= storage_cost;
            } else {
                success = false;
                break;
            }
        }

        self.loans.retain_mut(|loan| {
            let installment = loan.next_installment_amount();

            if installment > self.cash.current() {
                loan.defaults += 1;
                success = false;
            } else {
                self.cash.amount -= loan.next_installment_amount();
                loan.outstanding -= loan.next_principal_component();
                loan.n_installments += 1;
            }

            loan.outstanding >= 1. // Keep loans that are not fully repaid
        });

        if success {
            self.credit_score.score = (self.credit_score.score + 1).min(CreditScore::MAX);
        } else {
            self.credit_score.score = (self.credit_score.score - 12).max(CreditScore::MIN);
        }

        success
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
}
