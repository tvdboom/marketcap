use bevy::prelude::*;
use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::core::factors::Factor;
use crate::core::factors::cash::Cash;
use crate::core::factors::credit_score::CreditScore;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::commodities::CommodityName;
use crate::core::loans::Loan;

#[derive(Clone, Serialize, Deserialize)]
pub struct OwnedCommodity {
    pub id: String,
    pub name: CommodityName,
    pub amount: u32,
    pub buy_date: NaiveDate,
    pub buy_price: f32,
    pub warning: bool,
}

impl OwnedCommodity {
    pub fn maturity_date(&self, economy: &GlobalEconomy) -> Option<NaiveDate> {
        economy
            .get_commodity(&self.name)
            .maturity
            .and_then(|m| Some(self.buy_date + Duration::days(m as i64)))
    }
}

#[derive(Resource, Clone, Default, Serialize, Deserialize)]
pub struct Player {
    pub cash: Cash,
    pub credit_score: CreditScore,
    pub loans: Vec<Loan>,
    pub commodities: Vec<OwnedCommodity>,
}

impl Player {
    pub fn enterprise_value(&self, economy: &GlobalEconomy) -> f32 {
        self.cash.amount
            + self
                .commodities
                .iter()
                .map(|owned| owned.amount as f32 * economy.get_commodity(&owned.name).current())
                .sum::<f32>()
            - self.loans.iter().map(|l| l.outstanding).sum::<f32>()
    }

    pub fn inflow(&self) -> f32 {
        self.cash.accumulated_interest
    }

    pub fn outflow(&self) -> f32 {
        self.loans.iter().map(|l| l.next_installment_amount()).sum()
    }

    pub fn netflow(&self) -> f32 {
        self.inflow() - self.outflow()
    }

    pub fn resolve_loans(&mut self) -> bool {
        if self.loans.is_empty() {
            return true;
        }

        let mut success = true;
        for loan in self.loans.iter_mut() {
            let installment = loan.next_installment_amount();

            if installment > self.cash.current() {
                loan.defaults += 1;
                success = false;
            } else {
                self.cash.amount -= loan.next_installment_amount();
                loan.outstanding -= loan.next_principal_component();
                loan.n_installments += 1;
            }
        }

        if success {
            self.credit_score.score = (self.credit_score.score + 1).min(CreditScore::MAX);
        } else {
            self.credit_score.score = (self.credit_score.score - 12).max(CreditScore::MIN);
        }

        // Remove loans that are fully repaid
        // Remove at <1 to not show outstanding 0 for an active loan
        self.loans.retain(|l| l.outstanding >= 1.);

        success
    }
}
