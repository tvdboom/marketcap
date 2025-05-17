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
}
