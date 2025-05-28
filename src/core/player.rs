use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::core::factors::Factor;
use crate::core::factors::cash::Cash;
use crate::core::factors::credit_score::CreditScore;
use crate::core::loans::Loan;

#[derive(Resource, Clone, Default, Serialize, Deserialize)]
pub struct Player {
    pub cash: Cash,
    pub credit_score: CreditScore,
    pub loans: Vec<Loan>,
}

impl Player {
    pub fn enterprise_value(&self) -> f32 {
        self.cash.amount - self.loans.iter().map(|l| l.outstanding).sum::<f32>()
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
        self.loans.retain(|l| l.outstanding >= 1.);

        success
    }
}
