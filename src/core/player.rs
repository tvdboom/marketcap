use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct Player {
    pub cash: u32,
}

impl Default for Player {
    fn default() -> Self {
        Player { cash: 1000 }
    }
}

impl Player {
    pub fn market_cap(&self) -> u32 {
        self.cash
    }

    pub fn inflow(&self) -> u32 {
        self.cash
    }

    pub fn outflow(&self) -> u32 {
        0
    }

    pub fn netflow(&self) -> i32 {
        self.inflow() as i32 - self.outflow() as i32
    }
}
