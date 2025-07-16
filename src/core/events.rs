use crate::core::countries::CountryName;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::commodities::CommodityName;
use crate::core::instruments::crypto::CryptoName;
use crate::core::instruments::instrument::InstrumentKind;
use crate::core::player::Player;
use crate::core::research::TechName;
use crate::utils::NameFromEnum;
use chrono::NaiveDate;
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;
use rand::rng;
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventName {
    BrazilPolitics,
    Covid,
    Crimea,
    CryptoCrash(CryptoName),
    Drought(CountryName),
    Gas(CountryName),
    Harvest(CountryName),
    Oil(CountryName),
    RussiaWar,
    TradeWar,
}

impl EventName {
    pub fn create(economy: &GlobalEconomy, player: &Player) -> EconomicEvent {
        let dist = WeightedIndex::new(&Self::weights(economy, player)).unwrap();
        let mut name = Self::iter().collect::<Vec<_>>()[dist.sample(&mut rng())].clone();

        name = match name {
            EventName::CryptoCrash(_) => EventName::CryptoCrash(
                CryptoName::iter()
                    .filter(|c| {
                        let instrument = economy.get(&InstrumentKind::Crypto(*c));
                        player.can_see_crypto(instrument)
                    })
                    .choose(&mut rng())
                    .unwrap(),
            ),
            name @ (EventName::Drought(_) | EventName::Harvest(_)) => {
                let country = economy
                    .countries
                    .iter()
                    .filter_map(|c| {
                        c.production
                            .iter()
                            .any(|(n, _)| n.is_food())
                            .then_some(c.name)
                    })
                    .choose(&mut rng())
                    .unwrap();

                match name {
                    EventName::Drought(_) => EventName::Drought(country),
                    EventName::Harvest(_) => EventName::Harvest(country),
                    _ => unreachable!(),
                }
            },
            EventName::Gas(_) => EventName::Gas(
                economy
                    .countries
                    .iter()
                    .filter_map(|c| {
                        c.production
                            .iter()
                            .any(|(n, _)| *n == CommodityName::LNG)
                            .then_some(c.name)
                    })
                    .choose(&mut rng())
                    .unwrap(),
            ),
            EventName::Oil(_) => EventName::Oil(
                economy
                    .countries
                    .iter()
                    .filter_map(|c| {
                        c.production
                            .iter()
                            .any(|(n, _)| *n == CommodityName::Oil)
                            .then_some(c.name)
                    })
                    .choose(&mut rng())
                    .unwrap(),
            ),
            _ => name,
        };

        let duration = match name {
            EventName::BrazilPolitics => 365 + rand::random::<u32>() % 365,
            EventName::Covid => 120 + rand::random::<u32>() % 120,
            EventName::Drought(_) | EventName::Harvest(_) => 30 + rand::random::<u32>() % 30,
            EventName::RussiaWar => 365 + rand::random::<u32>() % 365,
            EventName::TradeWar => 180 + rand::random::<u32>() % 180,
            _ => 1,
        };

        EconomicEvent::new(name, economy.date, duration)
    }

    pub fn weights(economy: &GlobalEconomy, player: &Player) -> Vec<f32> {
        Self::iter()
            .map(|event| {
                match event {
                    n @ EventName::BrazilPolitics => {
                        (!economy.events.iter().any(|e| e.name == n)).then_some(0.1)
                    },
                    n @ EventName::Covid => {
                        (!economy.events.iter().any(|e| e.name == n)).then_some(0.1)
                    },
                    n @ EventName::Crimea => (player.has_tech(&TechName::ForeignExchange)
                        && !economy.events.iter().any(|e| e.name == n))
                    .then_some(0.1),
                    EventName::CryptoCrash(_) => {
                        player.has_tech(&TechName::Cryptocurrencies).then_some(1.)
                    },
                    EventName::Drought(_) | EventName::Harvest(_) => {
                        player.has_tech(&TechName::Commodities).then_some(1.)
                    },
                    EventName::Gas(_) | EventName::Oil(_) => {
                        player.has_tech(&TechName::Commodities).then_some(1.)
                    },
                    n @ EventName::RussiaWar => (!economy.events.iter().any(|e| e.name == n)
                        && economy
                            .active_events()
                            .iter()
                            .any(|e| e.name == EventName::Crimea))
                    .then_some(0.1),
                    n @ EventName::TradeWar => {
                        (!economy.events.iter().any(|e| e.name == n)).then_some(0.1)
                    },
                }
                .unwrap_or(0.)
            })
            .collect()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EconomicEvent {
    /// Name of the event
    pub name: EventName,

    /// Start date
    pub start_date: NaiveDate,

    /// Number of days the event has been active
    pub duration: u32,
}

impl EconomicEvent {
    pub fn new(name: EventName, start_date: NaiveDate, duration: u32) -> Self {
        Self {
            name,
            start_date,
            duration,
        }
    }

    pub fn title(&self) -> String {
        match self.name {
            EventName::BrazilPolitics => "Brazilian right-wing gains power".to_string(),
            EventName::Covid => "Covid pandemic".to_string(),
            EventName::Crimea => "Russia invades Crimea".to_string(),
            EventName::CryptoCrash(name) => format!("{} crash", name.to_name()),
            EventName::Drought(country) => format!("Prolonged drought in {}", country.to_name()),
            EventName::Gas(country) => format!("New gas field discovered in {}", country.to_name()),
            EventName::Harvest(country) => format!("Plentiful harvest in {}", country.to_name()),
            EventName::Oil(country) => format!("New oil field discovered in {}", country.to_name()),
            EventName::RussiaWar => "Russia invades Ukraine".to_string(),
            EventName::TradeWar => "USA - China trade war escalates".to_string(),
        }
    }

    pub fn image(&self) -> String {
        self.name.to_lowername().replace(" ", "-")
    }

    pub fn description(&self) -> String {
        match self.name {
            EventName::BrazilPolitics => {
                "A right-wing government in Brazil implements policies that favor deregulation \
                and privatization, leading to increased foreign investment. However, it also \
                results in social unrest and protests against inequality. The government's \
                focus on economic growth may lead to environmental concerns, particularly in \
                the Amazon rainforest."
                    .to_string()
            },
            EventName::Covid => {
                "The Covid pandemic causes a global economic downturn, affecting all financial \
                markets. It leads to reduced consumer spending, disrupted supply chains, and \
                increased unemployment. Governments implement lockdowns and stimulus packages to \
                mitigate the impact. The pandemic also accelerates digital transformation and \
                remote work trends, reshaping the global economy."
                    .to_string()
            },
            EventName::Crimea => {
                "Russia's invasion of Crimea leads to geopolitical tensions, sanctions from \
                Western countries, and a significant impact on global markets. The conflict \
                disrupts trade routes and energy supplies, causing volatility in oil and gas \
                prices. The situation may lead to increased military spending and shifts in \
                alliances among nations."
                    .to_string()
            },
            EventName::CryptoCrash(name) => format!(
                "A failure in the underlying technology of cryptocurrency {} causes investors \
                to panic, leading to a big sell-off in the market. This results in a sudden and \
                significant drop in the value of the coin.",
                name.to_name()
            ),
            EventName::Drought(country) => format!(
                "A prolonged drought in {} leads to reduced agricultural output, causing food \
                prices to rise and impacting the economy of the country. The government may \
                implement measures to conserve water and support farmers.",
                country.to_name()
            ),
            EventName::Gas(country) => format!(
                "The discovery of a new gas field in {} boosts the country's energy sector, \
                leading to increased exports and potential economic growth. This may also lead \
                to geopolitical tensions over energy resources.",
                country.to_name()
            ),
            EventName::Harvest(country) => format!(
                "A plentiful harvest in {} boosts the agricultural sector, leading to lower food \
                prices and increased exports. This positively impacts the country's economy, \
                providing a temporary relief from inflation and improving trade balances.",
                country.to_name()
            ),
            EventName::Oil(country) => format!(
                "The discovery of a new oil field in {} significantly increases the country's \
                oil reserves, leading to potential economic growth. This may attract foreign \
                investment and boost the energy sector, but could also lead to environmental \
                concerns and geopolitical tensions.",
                country.to_name()
            ),
            EventName::RussiaWar => {
                "Russia's invasion of Ukraine leads to a prolonged conflict, causing significant \
                economic disruption in the region. It results in sanctions, trade restrictions, \
                and increased military spending. The war also affects global energy prices and \
                supply chains, leading to inflationary pressures worldwide."
                    .to_string()
            },
            EventName::TradeWar => {
                "The escalating trade war between the USA and China leads to tariffs, trade \
                restrictions, and increased costs for consumers and businesses. It causes \
                uncertainty in global markets, affecting investments and economic growth. The \
                conflict may lead to shifts in supply chains and trade alliances."
                    .to_string()
            },
        }
    }

    pub fn initialize(&self, economy: &mut GlobalEconomy) {
        match self.name {
            EventName::Covid => {
                *economy.economy.values.back_mut().unwrap() += 0.05;
            },
            _ => (),
        }
    }

    pub fn advance(&self, economy: &mut GlobalEconomy) {
        match self.name {
            _ => (),
        }
    }
}
