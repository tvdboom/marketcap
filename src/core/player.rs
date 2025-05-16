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
