use serde::{Deserialize, Serialize};
use crate::core::global_economy::PoliticalLandscape;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Governance {
    Democracy,
    SemiDemocracy,
    Autocracy,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Ideology {
    Left,
    Neutral,
    Right,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Culture {
    Conservative,
    Moderate,
    Progressive,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Orientation {
    Socialism,
    Mixed,
    Capitalism,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Politics {
    pub governance: Governance,
    pub ideology: Ideology,
    pub culture: Culture,
    pub orientation: Orientation,
}

impl Politics {
    fn score_alignment(value: i32, direction: i32) -> f32 {
        let normalized = value as f32 / PoliticalLandscape::RANGE as f32;
        (normalized * direction as f32) * 0.125
    }

    pub fn get_score(&self, landscape: &PoliticalLandscape) -> f32 {
        let mut score = 0.0;

        score += match self.governance {
            Governance::Democracy => Self::score_alignment(landscape.government, -1),
            Governance::SemiDemocracy => 0.,
            Governance::Autocracy => Self::score_alignment(landscape.government, 1),
        };

        score += match self.ideology {
            Ideology::Left => Self::score_alignment(landscape.ideology, -1),
            Ideology::Neutral => 0.,
            Ideology::Right => Self::score_alignment(landscape.ideology, 1),
        };

        score += match self.culture {
            Culture::Conservative => Self::score_alignment(landscape.culture, -1),
            Culture::Moderate => 0.,
            Culture::Progressive => Self::score_alignment(landscape.culture, 1),
        };

        score += match self.orientation {
            Orientation::Socialism => Self::score_alignment(landscape.orientation, -1),
            Orientation::Mixed => 0.,
            Orientation::Capitalism => Self::score_alignment(landscape.orientation, 1),
        };

        score.clamp(-0.5, 0.5)
    }
}

impl Default for Politics {
    fn default() -> Self {
        Self {
            governance: Governance::SemiDemocracy,
            ideology: Ideology::Neutral,
            culture: Culture::Moderate,
            orientation: Orientation::Mixed,
        }
    }
}
