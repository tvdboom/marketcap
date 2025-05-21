use crate::core::factors::cash::Cash;
use crate::core::factors::credit_score::CreditScore;
use bevy::prelude::*;

#[derive(Resource, Clone, Default)]
pub struct Player {
    pub cash: Cash,
    pub credit_score: CreditScore,
}

impl Player {
    pub fn enterprise_value(&self) -> f32 {
        self.cash.amount
    }

    pub fn inflow(&self) -> f32 {
        self.cash.accumulated_interest
    }

    pub fn outflow(&self) -> f32 {
        0.
    }

    pub fn netflow(&self) -> f32 {
        self.inflow() - self.outflow()
    }
}
