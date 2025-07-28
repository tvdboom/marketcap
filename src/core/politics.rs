use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::global_economy::PoliticalLandscape;

#[derive(EnumIter, Clone, Debug, Serialize, Deserialize)]
pub enum Government {
    Democracy,
    SemiDemocracy,
    Autocracy,
}

#[derive(EnumIter, Clone, Debug, Serialize, Deserialize)]
pub enum Ideology {
    Left,
    Neutral,
    Right,
}

#[derive(EnumIter, Clone, Debug, Serialize, Deserialize)]
pub enum Culture {
    Conservative,
    Moderate,
    Progressive,
}

#[derive(EnumIter, Clone, Debug, Serialize, Deserialize)]
pub enum Orientation {
    Socialism,
    Mixed,
    Capitalism,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Politics {
    pub government: Government,
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

        score += match self.government {
            Government::Democracy => Self::score_alignment(landscape.government, -1),
            Government::SemiDemocracy => 0.,
            Government::Autocracy => Self::score_alignment(landscape.government, 1),
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
            government: Government::SemiDemocracy,
            ideology: Ideology::Neutral,
            culture: Culture::Moderate,
            orientation: Orientation::Mixed,
        }
    }
}
