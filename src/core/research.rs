use bevy::prelude::EventWriter;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::messages::{MessageEv, MessageLevel};
use crate::utils::NameFromEnum;

#[derive(EnumIter, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResearchField {
    Trading,
    Equity,
    AlternativeInvestments,
    Credit,
    Strategy,
}

#[derive(EnumIter, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TechName {
    // Trading
    LimitOrder,
    TrailingOrder,
    ShortSelling,
    Futures,
    Options,

    // Equities
    ESG,
    CorporateBonds,
    HighYield,
    CreditDefaultSwap,

    // Alternative Investments
    ForeignExchange,
    Commodities,
    ReducedStorage,
    Cryptocurrencies,
    ObscureCoins,

    // Credit
    MarginLoan,
    TrustworthyBorrower,
    AlternativeLender,
    SecureBased,

    // Strategy
    ImprovedResearch,
    BackdoorPolitics,
}

impl TechName {
    pub fn description(&self) -> &str {
        match self {
            TechName::LimitOrder => {
                "Enables the use of limit orders, which allow you to set a maximum price at which \
                you are willing to buy or a minimum price at which you are willing to sell a \
                financial instrument."
            },
            TechName::TrailingOrder => {
                "Enables the use of trailing orders, which allow you to set a stop-loss order that \
                follows the market price by a certain percentage."
            },
            TechName::ShortSelling => {
                "Enables the use of short selling, which allows you to borrow and sell a financial \
                instrument with the expectation that its price will fall, allowing you to buy it \
                back at a lower price."
            },
            TechName::Futures => {
                "Enables the use of futures contracts, which are agreements to buy or sell a \
                financial instrument at a predetermined price at a specified time in the future. \
                This can be used for hedging or speculation."
            },
            TechName::Options => {
                "Enables the use of options contracts, which give you the right, but not the \
                obligation, to buy or sell a financial instrument at a predetermined price before \
                a specified date. This can be used for hedging or speculation."
            },
            TechName::ESG => {
                "Enables the ESG (Environmental, Social, and Governance) information on stocks \
                ESG scores have a direct impact on stock prices."
            },
            TechName::CorporateBonds => {
                "Enables the trading of corporate bonds, which are debt securities issued by \
                corporations to raise capital. They typically offer higher yields than government \
                bonds."
            },
            TechName::HighYield => {
                "Enables the trading of high-yield bonds, which are bonds with lower credit \
                ratings (thus higher chance of default), but also higher interest rates."
            },
            TechName::CreditDefaultSwap => {
                "Enables the use of credit default swaps, which are financial derivatives that \
                allow an investor to hedge against a bond issuer defaulting."
            },
            TechName::ForeignExchange => {
                "Enables the trading of foreign exchange (forex), which is the market for buying \
                and selling currencies. This allows you to profit from changes in exchange rates."
            },
            TechName::Commodities => {
                "Enables the trading of commodities, which are basic goods used in commerce that \
                are interchangeable with other goods of the same type. Examples include gold, \
                silver, oil, and agricultural products."
            },
            TechName::ReducedStorage => {
                "Reduces the cost of storing commodities by 20%, increasing profitability."
            },
            TechName::Cryptocurrencies => {
                "Enables the trading of cryptocurrencies, which are digital currencies based on \
                blockchain technology. Cryptos are very risky investments but can offer huge returns."
            },
            TechName::ObscureCoins => {
                "Enables the trading of obscure coins, which are lesser-known cryptocurrencies \
                that can be highly volatile and speculative. They can offer high returns on short \
                term but also come with incredible risks."
            },
            TechName::MarginLoan => {
                "Enables the use of margin loans, which allow you to borrow money to invest in \
                financial instruments, using the instruments self as collateral. This can greatly \
                leverage your positions."
            },
            TechName::TrustworthyBorrower => {
                "the credit score increases twice as fast when debt obligations are met in time."
            },
            TechName::AlternativeLender => {
                "Enables the use of alternative lenders for term loans. Alternative lenders offer \
                loans without taken into account the credit score, for usually higher interest \
                rates than banks."
            },
            TechName::SecureBased => {
                "The credit score decreases half as fast when debt obligations are not met in time."
            },
            TechName::ImprovedResearch => "Increases the maximum research capacity by 100.",
            TechName::BackdoorPolitics => "Increases the amount of influence gained.",
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Technology {
    pub name: TechName,
    pub field: ResearchField,
    pub progress: f32,
    pub researching: bool,
    pub dependencies: Option<Vec<TechName>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Research {
    pub capacity: f32,
    pub technologies: Vec<Technology>,
}

impl Research {
    const COST_PER_CAPACITY: f32 = 0.1;

    pub fn get_tech_mut(&mut self, field: &ResearchField) -> Vec<&mut Technology> {
        self.technologies.iter_mut().filter(|r| r.field == *field).collect()
    }

    pub fn has_technology(&self, name: &TechName) -> bool {
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
                    name: TechName::LimitOrder,
                    field: ResearchField::Trading,
                    progress: 0.,
                    researching: false,
                    dependencies: None,
                },
                Technology {
                    name: TechName::TrailingOrder,
                    field: ResearchField::Trading,
                    progress: 0.,
                    researching: false,
                    dependencies: Some(vec![TechName::LimitOrder]),
                },
                Technology {
                    name: TechName::ShortSelling,
                    field: ResearchField::Trading,
                    progress: 0.,
                    researching: false,
                    dependencies: Some(vec![TechName::TrailingOrder, TechName::MarginLoan]),
                },
                Technology {
                    name: TechName::Futures,
                    field: ResearchField::Trading,
                    progress: 0.,
                    researching: false,
                    dependencies: Some(vec![TechName::ShortSelling]),
                },
                Technology {
                    name: TechName::Options,
                    field: ResearchField::Trading,
                    progress: 0.,
                    researching: false,
                    dependencies: Some(vec![TechName::Futures]),
                },
                Technology {
                    name: TechName::ESG,
                    field: ResearchField::Equity,
                    progress: 0.,
                    researching: false,
                    dependencies: None,
                },
                Technology {
                    name: TechName::CorporateBonds,
                    field: ResearchField::Equity,
                    progress: 0.,
                    researching: false,
                    dependencies: Some(vec![TechName::ESG]),
                },
                Technology {
                    name: TechName::HighYield,
                    field: ResearchField::Equity,
                    progress: 0.,
                    researching: false,
                    dependencies: Some(vec![TechName::CorporateBonds]),
                },
                Technology {
                    name: TechName::CreditDefaultSwap,
                    field: ResearchField::Equity,
                    progress: 0.,
                    researching: false,
                    dependencies: Some(vec![TechName::HighYield]),
                },
                Technology {
                    name: TechName::ForeignExchange,
                    field: ResearchField::AlternativeInvestments,
                    progress: 0.,
                    researching: false,
                    dependencies: None,
                },
                Technology {
                    name: TechName::Commodities,
                    field: ResearchField::AlternativeInvestments,
                    progress: 0.,
                    researching: false,
                    dependencies: Some(vec![TechName::ForeignExchange]),
                },
                Technology {
                    name: TechName::ReducedStorage,
                    field: ResearchField::AlternativeInvestments,
                    progress: 0.,
                    researching: false,
                    dependencies: Some(vec![TechName::Commodities]),
                },
                Technology {
                    name: TechName::Cryptocurrencies,
                    field: ResearchField::AlternativeInvestments,
                    progress: 0.,
                    researching: false,
                    dependencies: Some(vec![TechName::ReducedStorage]),
                },
                Technology {
                    name: TechName::ObscureCoins,
                    field: ResearchField::AlternativeInvestments,
                    progress: 0.,
                    researching: false,
                    dependencies: Some(vec![TechName::Cryptocurrencies]),
                },
                Technology {
                    name: TechName::MarginLoan,
                    field: ResearchField::Credit,
                    progress: 0.,
                    researching: false,
                    dependencies: None,
                },
                Technology {
                    name: TechName::TrustworthyBorrower,
                    field: ResearchField::Credit,
                    progress: 0.,
                    researching: false,
                    dependencies: Some(vec![TechName::MarginLoan]),
                },
                Technology {
                    name: TechName::AlternativeLender,
                    field: ResearchField::Credit,
                    progress: 0.,
                    researching: false,
                    dependencies: Some(vec![TechName::TrustworthyBorrower]),
                },
                Technology {
                    name: TechName::SecureBased,
                    field: ResearchField::Credit,
                    progress: 0.,
                    researching: false,
                    dependencies: Some(vec![TechName::AlternativeLender]),
                },
                Technology {
                    name: TechName::ImprovedResearch,
                    field: ResearchField::Strategy,
                    progress: 0.,
                    researching: false,
                    dependencies: None,
                },
                Technology {
                    name: TechName::BackdoorPolitics,
                    field: ResearchField::Strategy,
                    progress: 0.,
                    researching: false,
                    dependencies: Some(vec![TechName::ImprovedResearch]),
                },
            ]),
        }
    }
}
