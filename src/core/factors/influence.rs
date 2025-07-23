use serde::{Deserialize, Serialize};

use crate::core::factors::Factor;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Influence {
    score: f32,
}

impl Influence {
    pub fn bump(&mut self, aum: f32, has_tech: bool) {
        self.score += aum
            / if has_tech {
                5e4
            } else {
                1e5
            };
    }
}

impl Factor for Influence {
    fn image(&self) -> &str {
        "influence"
    }

    fn description(&self) -> String {
        "The influence refers to the capacity or power of the company to affect the behavior, \
        and decisions of politicians and other lawmakers in the world. Use this resource to \
        lobby politics and policies towards your desired preference. Influence increases over \
        time proportionally to your AUM."
            .to_string()
    }

    fn current(&self) -> f32 {
        self.score
    }
}
