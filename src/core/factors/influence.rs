use serde::{Deserialize, Serialize};

use crate::core::factors::Factor;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Influence {
    score: f32,
}

impl Influence {
    pub fn bump(&mut self, aum: f32) {
        self.score += aum / 1e4;
    }
}

impl Factor for Influence {
    fn image(&self) -> &str {
        "influence"
    }

    fn description(&self) -> String {
        "The influence refers to the capacity or power of the company to affect the behavior, \
        and decisions of politicians and other law-makers in the world. Use this resource to \
        lobby politics and policies towards your desired preference."
            .to_string()
    }

    fn current(&self) -> f32 {
        self.score
    }
}
