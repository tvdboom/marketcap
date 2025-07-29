use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::core::global_economy::PoliticalLandscape;
use crate::utils::NameFromEnum;

#[derive(EnumIter, Clone, Debug, Serialize, Deserialize)]
pub enum Government {
    Democracy,
    Neutral,
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
    Neutral,
    Progressive,
}

#[derive(EnumIter, Clone, Debug, Serialize, Deserialize)]
pub enum Orientation {
    Socialism,
    Neutral,
    Capitalism,
}

#[derive(EnumIter, Clone, Debug, Serialize, Deserialize)]
pub enum PoliticalField {
    Government,
    Ideology,
    Culture,
    Orientation,
}

impl PoliticalField {
    pub fn emoji(&self) -> &str {
        match self {
            PoliticalField::Government => "👑",
            PoliticalField::Ideology => "🍀",
            PoliticalField::Culture => "👨",
            PoliticalField::Orientation => "💲",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            PoliticalField::Government => {
                "Defines the structure of political authority within the state, from democratic \
                systems that emphasize representation and citizen participation, to autocratic \
                regimes centered around centralized control and limited individual freedoms."
            },
            PoliticalField::Ideology => {
                "Represents the dominant political worldview, influencing how power, equality, \
                and justice are interpreted—ranging from progressive left-wing ideals to \
                conservative right-wing beliefs, with neutral positions offering moderate approaches."
            },
            PoliticalField::Culture => {
                "Captures the societal attitude toward tradition, change, and social values—whether \
                rooted in conservative norms, aligned with progressive reforms, or maintaining a \
                neutral stance that balances both."
            },
            PoliticalField::Orientation => {
                "Reflects the state's economic philosophy, determining whether it leans toward \
                collective welfare and redistribution (socialism), free-market competition and \
                individual enterprise (capitalism), or a centrist mix of both approaches."
            },
        }
    }

    pub fn fields(&self) -> Vec<String> {
        match self {
            PoliticalField::Government => Government::iter().map(|g| g.to_name()).collect(),
            PoliticalField::Ideology => Ideology::iter().map(|i| i.to_name()).collect(),
            PoliticalField::Culture => Culture::iter().map(|c| c.to_name()).collect(),
            PoliticalField::Orientation => Orientation::iter().map(|o| o.to_name()).collect(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Politics {
    pub government: Government,
    pub ideology: Ideology,
    pub culture: Culture,
    pub orientation: Orientation,
}

impl Politics {
    pub fn get(&self, field: &PoliticalField) -> String {
        match field {
            PoliticalField::Government => self.government.to_name(),
            PoliticalField::Ideology => self.ideology.to_name(),
            PoliticalField::Culture => self.culture.to_name(),
            PoliticalField::Orientation => self.orientation.to_name(),
        }
    }

    pub fn matches(&self, field: &PoliticalField, value: i8) -> i8 {
        match field {
            PoliticalField::Government => match self.government {
                Government::Democracy if value < 0 => 1,
                Government::Neutral => 0,
                Government::Autocracy if value > 0 => 1,
                _ => -1,
            },
            PoliticalField::Ideology => match self.ideology {
                Ideology::Left if value < 0 => 1,
                Ideology::Neutral => 0,
                Ideology::Right if value > 0 => 1,
                _ => -1,
            },
            PoliticalField::Culture => match self.culture {
                Culture::Conservative if value < 0 => 1,
                Culture::Neutral => 0,
                Culture::Progressive if value > 0 => 1,
                _ => -1,
            },
            PoliticalField::Orientation => match self.orientation {
                Orientation::Socialism if value < 0 => 1,
                Orientation::Neutral => 0,
                Orientation::Capitalism if value > 0 => 1,
                _ => -1,
            },
        }
    }

    fn score_alignment(value: i8, direction: i8) -> f32 {
        let normalized = value as f32 / PoliticalLandscape::RANGE as f32;
        (normalized * direction as f32) * 0.125
    }

    pub fn get_score(&self, landscape: &PoliticalLandscape) -> f32 {
        let mut score = 0.0;

        score += match self.government {
            Government::Democracy => Self::score_alignment(landscape.government, -1),
            Government::Neutral => 0.,
            Government::Autocracy => Self::score_alignment(landscape.government, 1),
        };

        score += match self.ideology {
            Ideology::Left => Self::score_alignment(landscape.ideology, -1),
            Ideology::Neutral => 0.,
            Ideology::Right => Self::score_alignment(landscape.ideology, 1),
        };

        score += match self.culture {
            Culture::Conservative => Self::score_alignment(landscape.culture, -1),
            Culture::Neutral => 0.,
            Culture::Progressive => Self::score_alignment(landscape.culture, 1),
        };

        score += match self.orientation {
            Orientation::Socialism => Self::score_alignment(landscape.orientation, -1),
            Orientation::Neutral => 0.,
            Orientation::Capitalism => Self::score_alignment(landscape.orientation, 1),
        };

        score.clamp(-0.5, 0.5)
    }
}

impl Default for Politics {
    fn default() -> Self {
        Self {
            government: Government::Neutral,
            ideology: Ideology::Neutral,
            culture: Culture::Neutral,
            orientation: Orientation::Neutral,
        }
    }
}
