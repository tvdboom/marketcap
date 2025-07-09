use crate::core::messages::{MessageEv, MessageLevel};
use crate::utils::NameFromEnum;
use bevy::prelude::EventWriter;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResearchField {
    Stocks,
    Bonds,
    Forex,
    Commodities,
    Crypto,
    Credit,
    Policies,
}

#[derive(EnumIter, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TechnologyName {
    AlternativeLender,
    MarginLoan,
}

impl TechnologyName {
    pub fn description(&self) -> &str {
        match self {
            TechnologyName::AlternativeLender => {
                "Enables the use of alternative lenders for term loans. Alternative lenders offer \
                loans without taken into account the credit score, for usually higher interest \
                rates than banks."
            },
            TechnologyName::MarginLoan => {
                "Enables the use of margin loans, which allow you to borrow money to invest in \"
                financial instruments, using the instruments self as collateral. This can greatly \
                leverage your positions."
            },
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Technology {
    pub name: TechnologyName,
    pub field: ResearchField,
    pub progress: f32,
    pub researching: bool,
    pub dependency: Option<TechnologyName>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Research {
    pub capacity: f32,
    pub technologies: Vec<Technology>,
}

impl Research {
    const COST_PER_CAPACITY: f32 = 0.1;

    pub fn get_tech_mut(&mut self, field: &ResearchField) -> Vec<&mut Technology> {
        self.technologies
            .iter_mut()
            .filter(|r| r.field == *field)
            .collect()
    }

    pub fn has_technology(&self, name: &TechnologyName) -> bool {
        self.technologies
            .iter()
            .find(|t| t.name == *name)
            .map(|r| r.progress == 100.)
            .unwrap_or(false)
    }

    pub fn n_active(&self) -> usize {
        self.technologies.iter().filter(|r| r.researching).count()
    }

    pub fn costs(&self) -> f32 {
        if self.n_active() > 0 {
            self.capacity * Self::COST_PER_CAPACITY
        } else {
            0.
        }
    }

    pub fn advance(&mut self, message: &mut EventWriter<MessageEv>) {
        let n_active = self.n_active();
        for research in self.technologies.iter_mut() {
            if research.researching {
                research.progress =
                    (research.progress + self.capacity / 100. / n_active as f32).min(100.);

                if research.progress == 100. {
                    research.researching = false;

                    message.write(MessageEv {
                        message: format!("Research '{}' completed.", research.name.to_name()),
                        level: MessageLevel::Info,
                    });
                }
            }
        }
    }
}

impl Default for Research {
    fn default() -> Self {
        Self {
            capacity: 100.,
            technologies: Vec::from([
                Technology {
                    name: TechnologyName::AlternativeLender,
                    field: ResearchField::Credit,
                    progress: 0.,
                    researching: false,
                    dependency: None,
                },
                Technology {
                    name: TechnologyName::MarginLoan,
                    field: ResearchField::Credit,
                    progress: 0.,
                    researching: false,
                    dependency: Some(TechnologyName::AlternativeLender),
                },
            ]),
        }
    }
}
