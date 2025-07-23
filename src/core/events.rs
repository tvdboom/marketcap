use chrono::{Duration, NaiveDate};
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;
use rand::seq::IteratorRandom;
use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::core::countries::CountryName;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::bonds::{BondIssuer, BondQuality};
use crate::core::instruments::commodities::CommodityName;
use crate::core::instruments::crypto::CryptoName;
use crate::core::instruments::instrument::InstrumentKind;
use crate::core::instruments::stocks::{Company, ESGRating};
use crate::core::player::Player;
use crate::core::research::TechName;
use crate::core::sectors::SectorName;
use crate::utils::NameFromEnum;

#[derive(EnumIter, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventName {
    BrazilPolitics,
    CeoResignation(Company),
    Covid,
    Crimea,
    CryptoCrash(CryptoName),
    CryptoFan(CryptoName),
    DDos(Company),
    Drought(CountryName),
    EsgScandal(Company),
    GasDiscovery(CountryName),
    GoldRush,
    Grounded,
    Harvest(CountryName),
    InterestBump(CountryName),
    NewProduct(Company),
    NewContract(Company),
    OilDiscovery(CountryName),
    OilDisruption,
    StorageCosts,
    Recession,
    RegulatoryCrackdown(SectorName),
    RussiaWar,
    TradeWar,
    Vaccine(Company),
}

impl EventName {
    pub fn create(economy: &GlobalEconomy, player: &Player) -> EconomicEvent {
        let dist = WeightedIndex::new(&Self::weights(economy, player)).unwrap();
        let mut name = Self::iter().collect::<Vec<_>>()[dist.sample(&mut rng())].clone();

        name = match name {
            EventName::CeoResignation(_) => {
                EventName::CeoResignation(Company::iter().choose(&mut rng()).unwrap())
            },
            name @ (EventName::CryptoCrash(_) | EventName::CryptoFan(_)) => {
                let crypto = CryptoName::iter()
                    .filter(|c| {
                        let instrument = economy.get(&InstrumentKind::Crypto(*c));
                        player.can_see_crypto(instrument)
                    })
                    .choose(&mut rng())
                    .unwrap();

                match name {
                    EventName::CryptoCrash(_) => EventName::CryptoCrash(crypto),
                    EventName::CryptoFan(_) => EventName::CryptoFan(crypto),
                    _ => unreachable!(),
                }
            },
            EventName::DDos(_) => EventName::DDos(Company::iter().choose(&mut rng()).unwrap()),
            name @ (EventName::Drought(_) | EventName::Harvest(_)) => {
                let country = economy
                    .countries
                    .iter()
                    .filter_map(|c| c.production.iter().any(|(n, _)| n.is_food()).then_some(c.name))
                    .choose(&mut rng())
                    .unwrap();

                match name {
                    EventName::Drought(_) => EventName::Drought(country),
                    EventName::Harvest(_) => EventName::Harvest(country),
                    _ => unreachable!(),
                }
            },
            EventName::EsgScandal(_) => EventName::EsgScandal(
                Company::iter()
                    .filter(|c| {
                        let instrument = economy.get(&InstrumentKind::Stock(*c));
                        instrument.esg() >= ESGRating::BB
                    })
                    .choose(&mut rng())
                    .unwrap(),
            ),
            EventName::NewContract(_) => EventName::NewContract(
                Company::iter()
                    .filter(|c| {
                        matches!(*c, Company::Boeing | Company::LockheedMartin | Company::Toyota)
                    })
                    .choose(&mut rng())
                    .unwrap(),
            ),
            EventName::NewProduct(_) => EventName::NewProduct(
                Company::iter()
                    .filter(|c| {
                        economy
                            .get(&InstrumentKind::Stock(*c))
                            .sectors()
                            .get(&SectorName::Retail)
                            .map(|v| *v > 0.2)
                            .unwrap_or(false)
                    })
                    .choose(&mut rng())
                    .unwrap(),
            ),
            EventName::GasDiscovery(_) => EventName::GasDiscovery(
                economy
                    .countries
                    .iter()
                    .filter_map(|c| {
                        c.production.iter().any(|(n, _)| *n == CommodityName::LNG).then_some(c.name)
                    })
                    .choose(&mut rng())
                    .unwrap(),
            ),
            EventName::InterestBump(_) => {
                EventName::InterestBump(CountryName::iter().choose(&mut rng()).unwrap())
            },
            EventName::OilDiscovery(_) => EventName::OilDiscovery(
                economy
                    .countries
                    .iter()
                    .filter_map(|c| {
                        c.production.iter().any(|(n, _)| *n == CommodityName::Oil).then_some(c.name)
                    })
                    .choose(&mut rng())
                    .unwrap(),
            ),
            EventName::RegulatoryCrackdown(_) => {
                EventName::RegulatoryCrackdown(SectorName::iter().choose(&mut rng()).unwrap())
            },
            EventName::Vaccine(_) => EventName::Vaccine(
                Company::iter()
                    .filter(|c| {
                        economy
                            .get(&InstrumentKind::Stock(*c))
                            .sectors()
                            .get(&SectorName::Healthcare)
                            .map(|v| *v > 0.7)
                            .unwrap_or(false)
                    })
                    .choose(&mut rng())
                    .unwrap(),
            ),
            _ => name,
        };

        EconomicEvent::new(name.clone(), economy.date, name.duration())
    }

    pub fn duration(&self) -> u32 {
        let duration = match self {
            EventName::BrazilPolitics => 365,
            EventName::Covid => 120,
            EventName::CryptoFan(_) => 30,
            EventName::Drought(_) | EventName::Harvest(_) => 30,
            EventName::GoldRush => 40,
            EventName::Grounded => 7,
            EventName::InterestBump(_) => 200,
            EventName::OilDisruption => 10,
            EventName::StorageCosts => 90,
            EventName::Recession => 60,
            EventName::RussiaWar => 365,
            EventName::TradeWar => 180,
            _ => 1,
        };

        if duration > 1 {
            duration * rng().random_range(1..3)
        } else {
            duration
        }
    }

    pub fn base_weight(&self) -> f32 {
        match self {
            EventName::BrazilPolitics => 0.1,
            EventName::Covid => 0.1,
            EventName::Crimea => 0.1,
            EventName::Recession => 0.4,
            EventName::RussiaWar => 0.1,
            EventName::TradeWar => 0.1,
            EventName::Vaccine(_) => 0.1,
            _ => 1.0,
        }
    }

    pub fn weights(economy: &GlobalEconomy, player: &Player) -> Vec<f32> {
        Self::iter()
            .map(|event| {
                match event {
                    n @ EventName::Crimea => (player.has_tech(&TechName::ForeignExchange)
                        && !economy.events.iter().any(|e| e.name == n))
                    .then_some(n.base_weight()),
                    EventName::CryptoCrash(_) | EventName::CryptoFan(_) => {
                        player.has_tech(&TechName::Cryptocurrencies).then_some(1.)
                    },
                    EventName::Drought(_) | EventName::Harvest(_) => {
                        player.has_tech(&TechName::Commodities).then_some(1.)
                    },
                    EventName::EsgScandal(_) => player.has_tech(&TechName::ESG).then_some(1.),
                    EventName::GasDiscovery(_) | EventName::OilDiscovery(_) => {
                        player.has_tech(&TechName::Commodities).then_some(1.)
                    },
                    n @ EventName::GoldRush => (player.has_tech(&TechName::Commodities)
                        && !economy.events.iter().any(|e| e.name == n))
                    .then_some(n.base_weight()),
                    n @ EventName::RussiaWar => (!economy.events.iter().any(|e| e.name == n)
                        && economy.active_events().iter().any(|e| e.name == EventName::Crimea))
                    .then_some(n.base_weight()),
                    n @ EventName::StorageCosts => (player.has_tech(&TechName::Commodities)
                        && !economy.active_events().iter().any(|e| e.name == n))
                    .then_some(n.base_weight()),
                    n @ EventName::Vaccine(_) => {
                        Some(if economy.has_active_event(&EventName::Covid) {
                            3. * n.base_weight()
                        } else {
                            n.base_weight()
                        })
                    },
                    n => (!economy.events.iter().any(|e| e.name == n)).then_some(n.base_weight()),
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
            EventName::CeoResignation(company) => {
                format!("The CEO of {} resigns", company.to_name())
            },
            EventName::Covid => "Covid pandemic".to_string(),
            EventName::Crimea => "Russia invades Crimea".to_string(),
            EventName::CryptoCrash(name) => format!("{} crash", name.to_name()),
            EventName::CryptoFan(name) => format!("Influencers support {}", name.to_name()),
            EventName::DDos(company) => format!("DDoS attack on {}", company.to_name()),
            EventName::Drought(country) => format!("Prolonged drought in {}", country.to_name()),
            EventName::EsgScandal(company) => {
                format!("{} ESG scandal", company.to_name())
            },
            EventName::GasDiscovery(country) => {
                format!("New gas field discovered in {}", country.to_name())
            },
            EventName::GoldRush => "Gold rush".to_string(),
            EventName::Grounded => "Air travel in EU grounded".to_string(),
            EventName::Harvest(country) => format!("Plentiful harvest in {}", country.to_name()),
            EventName::InterestBump(country) => {
                format!("Interest rates raised in {}", country.to_name())
            },
            EventName::NewContract(company) => {
                format!("{} signs a big new contract", company.to_name())
            },
            EventName::NewProduct(company) => {
                format!("{} launches a new product", company.to_name())
            },
            EventName::OilDiscovery(country) => {
                format!("New oil field discovered in {}", country.to_name())
            },
            EventName::OilDisruption => "Oil supply disruption in Saudi Arabia".to_string(),
            EventName::Recession => "Global recession".to_string(),
            EventName::RegulatoryCrackdown(sector) => {
                format!("Regulatory crackdown on sector {}", sector.to_name())
            },
            EventName::RussiaWar => "Russia invades Ukraine".to_string(),
            EventName::StorageCosts => "Storage costs rise".to_string(),
            EventName::TradeWar => "USA - China trade war escalates".to_string(),
            EventName::Vaccine(company) => format!("{} vaccine breakthrough", company.to_name()),
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
            EventName::CeoResignation(company) => {
                format!(
                    "The CEO of {} resigns unexpectedly, causing uncertainty in the company's \
                    leadership. This leads to a temporary drop in the stock price as investors \
                    react to the news. The company may face challenges in finding a suitable \
                    replacement and maintaining stability during the transition.",
                    company.to_name()
                )
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
            EventName::CryptoFan(name) => format!(
                "Influencers and celebrities endorse the cryptocurrency {} on social media, \
                leading to a surge in interest and investment. This causes the price of the coin \
                to rise sharply as new investors enter the market.",
                name.to_name()
            ),
            EventName::DDos(company) => format!(
                "A DDoS attack on {} disrupts its online services, leading to a temporary \
                decline in stock price. The company may face reputational damage and increased \
                security costs as it works to restore its systems and prevent future attacks.",
                company.to_name()
            ),
            EventName::Drought(country) => format!(
                "A prolonged drought in {} leads to reduced agricultural output, causing food \
                prices to rise and impacting the economy of the country. The government may \
                implement measures to conserve water and support farmers.",
                country.to_name()
            ),
            EventName::EsgScandal(company) => format!(
                "A major ESG scandal involving {} leads to a loss of investor confidence and \
                a significant drop in the company's stock price. The scandal may involve issues \
                such as environmental violations, labor rights abuses, or governance failures.",
                company.to_name()
            ),
            EventName::GasDiscovery(country) => format!(
                "The discovery of a new gas field in {} boosts the country's energy sector, \
                leading to increased exports and potential economic growth. This may also lead \
                to geopolitical tensions over energy resources.",
                country.to_name()
            ),
            EventName::GoldRush => {
                "Tensions un geopolitics causes gold and silver prices to skyrocket. Investors \
                flock to these safe-haven assets, leading to a surge in mining activities."
                    .to_string()
            },
            EventName::Grounded => {
                "Volcanic activity in Iceland has grounded all air traffic in the European Union. \
                This disrupts travel plans for millions of passengers and causes significant \
                economic losses for airlines and related industries."
                    .to_string()
            },
            EventName::Harvest(country) => format!(
                "A plentiful harvest in {} boosts the agricultural sector, leading to lower food \
                prices and increased exports. This positively impacts the country's economy, \
                providing a temporary relief from inflation and improving trade balances.",
                country.to_name()
            ),
            EventName::InterestBump(country) => format!(
                "The central bank of {} raises interest rates to combat inflation, leading to \
                higher borrowing costs for consumers and businesses. This may slow down economic \
                growth in the short term but is aimed at stabilizing the economy in the long run.",
                country.to_name()
            ),
            EventName::NewContract(company) => format!(
                "A major contract signed by {} boosts the company's stock price and increases its \
                revenue. The contract involves the production of new equipment, leading to \
                increased spending in the sector.",
                company.to_name()
            ),
            EventName::NewProduct(company) => format!(
                "The launch of a new product by {} creates a buzz in the market, leading to \
                increased sales and stock price appreciation. The product's success may depend \
                on consumer demand, marketing strategies, and competition.",
                company.to_name()
            ),
            EventName::OilDiscovery(country) => format!(
                "The discovery of a new oil field in {} significantly increases the country's \
                oil reserves, leading to potential economic growth. This may attract foreign \
                investment and boost the energy sector, but could also lead to environmental \
                concerns and geopolitical tensions.",
                country.to_name()
            ),
            EventName::OilDisruption => {
                "A major disruption in oil supply from Saudi Arabia leads to a spike in global \
                oil prices. This causes inflation and economic instability in the area, affecting \
                industries reliant on oil and energy."
                    .to_string()
            },
            EventName::Recession => {
                "A global recession leads to a significant downturn in economic activity, \
                resulting in high unemployment rates, reduced consumer spending, and falling \
                stock prices. Governments implement stimulus measures to boost the economy, \
                but recovery may take time as businesses struggle to adapt to the new economic \
                environment."
                    .to_string()
            },
            EventName::RegulatoryCrackdown(sector) => {
                format!(
                    "A regulatory crackdown on the {} sector leads to increased compliance costs \
                    and reduced profitability for companies. This may result in a steep decline of \
                    stock prices and a shift in investor sentiment towards more compliant sectors.",
                    sector.to_name()
                )
            },
            EventName::RussiaWar => {
                "Russia's invasion of Ukraine leads to a prolonged conflict, causing significant \
                economic disruption in the region. It results in sanctions, trade restrictions, \
                and increased military spending. The war also affects global energy prices and \
                supply chains, leading to inflationary pressures worldwide."
                    .to_string()
            },
            EventName::StorageCosts => {
                "Rising storage costs for commodities lead to increased prices and reduced \
                profitability for producers. This may result in a shift in supply chains, as \
                companies seek to minimize storage expenses. The situation could also lead to \
                increased investment in logistics and infrastructure."
                    .to_string()
            },
            EventName::TradeWar => {
                "The escalating trade war between the USA and China leads to tariffs, trade \
                restrictions, and increased costs for consumers and businesses. It causes \
                uncertainty in global markets, affecting investments and economic growth. The \
                conflict may lead to shifts in supply chains and trade alliances."
                    .to_string()
            },
            EventName::Vaccine(company) => format!(
                "A breakthrough in vaccine development by {} leads to a significant reduction \
                in the spread of infectious diseases. This boosts public health, increases \
                consumer confidence, and positively impacts the company's stock price.",
                company.to_name()
            ),
        }
    }

    pub fn is_active(&self, date: &NaiveDate) -> bool {
        *date < self.start_date + Duration::days(self.duration as i64)
    }

    pub fn start(&self, economy: &mut GlobalEconomy) {
        let mut rng = rng();
        match self.name {
            EventName::BrazilPolitics => {
                economy
                    .bonds
                    .iter_mut()
                    .find(|b| b.issuer == BondIssuer::Government(CountryName::Brazil))
                    .map(|b| {
                        b.quality = BondQuality::B;
                    });
            },
            EventName::CeoResignation(company) => {
                economy.stocks.iter_mut().find(|s| s.issuer == company).map(|s| {
                    *s.prices.back_mut().unwrap() *= rng.random_range(0.8..0.9);
                });
            },
            EventName::Covid => {
                economy.sectors.iter_mut().find(|s| s.name == SectorName::Transport).map(|s| {
                    s.update(rng.random_range(-30..-10));
                });
            },
            EventName::Crimea => {
                economy
                    .currencies
                    .iter_mut()
                    .find(|c| matches!(c.country, CountryName::Russia | CountryName::Ukraine))
                    .map(|c| {
                        *c.values.back_mut().unwrap() *= rng.random_range(0.5..0.8);
                    });

                economy
                    .commodities
                    .iter_mut()
                    .find(|c| {
                        matches!(
                            c.name,
                            CommodityName::LNG | CommodityName::Oil | CommodityName::Wheat
                        )
                    })
                    .map(|c| {
                        *c.prices.back_mut().unwrap() *= rng.random_range(1.2..1.3);
                    });

                economy.sectors.iter_mut().find(|s| s.name == SectorName::Military).map(|s| {
                    s.update(rng.random_range(15..25));
                });
            },
            EventName::CryptoCrash(name) => {
                economy.cryptos.iter_mut().find(|c| c.name == name).map(|c| {
                    *c.prices.back_mut().unwrap() *= rng.random_range(0.5..0.75);
                });
            },
            EventName::DDos(company) => {
                economy.stocks.iter_mut().find(|s| s.issuer == company).map(|s| {
                    *s.prices.back_mut().unwrap() *= rng.random_range(0.85..0.95);
                });
            },
            EventName::EsgScandal(company) => {
                economy.stocks.iter_mut().find(|s| s.issuer == company).map(|s| {
                    s.esg.decrease();
                });
            },
            EventName::GasDiscovery(country) => {
                economy.currencies.iter_mut().find(|c| c.country == country).map(|c| {
                    c.base_value *= rng.random_range(1.1..1.2);
                });

                economy.commodities.iter_mut().find(|c| c.name == CommodityName::LNG).map(|c| {
                    c.base_price *= rng.random_range(0.85..0.95);
                });
            },
            EventName::Grounded => {
                economy.sectors.iter_mut().find(|s| s.name == SectorName::Transport).map(|s| {
                    s.update(-10);
                });
            },
            EventName::InterestBump(country) => {
                economy
                    .bonds
                    .iter_mut()
                    .filter(|b| b.issuer == BondIssuer::Government(country))
                    .for_each(|b| {
                        b.interest *= rng.random_range(1.15..1.25);
                    });
            },
            EventName::NewProduct(company) | EventName::NewContract(company) => {
                economy.stocks.iter_mut().find(|s| s.issuer == company).map(|s| {
                    *s.prices.back_mut().unwrap() *= rng.random_range(1.2..1.4);
                });
            },
            EventName::OilDiscovery(country) => {
                economy.currencies.iter_mut().find(|c| c.country == country).map(|c| {
                    c.base_value *= rng.random_range(1.1..1.2);
                });

                economy.commodities.iter_mut().find(|c| c.name == CommodityName::Oil).map(|c| {
                    c.base_price *= rng.random_range(0.85..0.95);
                });
            },
            EventName::RegulatoryCrackdown(sector) => {
                economy.sectors.iter_mut().find(|s| s.name == sector).map(|s| {
                    s.update(-rng.random_range(20..40));
                });
            },
            EventName::RussiaWar => {
                economy
                    .currencies
                    .iter_mut()
                    .find(|c| matches!(c.country, CountryName::Russia | CountryName::Ukraine))
                    .map(|c| {
                        c.base_value *= rng.random_range(0.5..0.8);
                        *c.values.back_mut().unwrap() *= rng.random_range(0.5..0.8);
                    });

                economy
                    .bonds
                    .iter_mut()
                    .find(|b| b.issuer == BondIssuer::Government(CountryName::Russia))
                    .map(|b| {
                        b.quality = BondQuality::C;
                    });

                economy.sectors.iter_mut().find(|s| s.name == SectorName::Military).map(|s| {
                    s.update(rng.random_range(15..25));
                });
            },
            EventName::TradeWar => {
                economy
                    .currencies
                    .iter_mut()
                    .find(|c| matches!(c.country, CountryName::China | CountryName::USA))
                    .map(|c| {
                        c.base_value *= rng.random_range(0.8..0.9);
                        *c.values.back_mut().unwrap() *= rng.random_range(0.8..0.9);
                    });

                economy.sectors.iter_mut().find(|s| s.name == SectorName::Finance).map(|s| {
                    s.update(-10);
                });
            },
            EventName::Vaccine(company) => {
                economy.stocks.iter_mut().find(|s| s.issuer == company).map(|s| {
                    *s.prices.back_mut().unwrap() *= rng.random_range(1.2..1.4);
                });

                economy.sectors.iter_mut().find(|s| s.name == SectorName::Healthcare).map(|s| {
                    s.update(rng.random_range(10..25));
                });
            },
            _ => (),
        }
    }

    pub fn advance(&self, economy: &mut GlobalEconomy) {
        match self.name {
            EventName::BrazilPolitics => {
                economy.currencies.iter_mut().find(|c| c.country == CountryName::Brazil).map(|c| {
                    *c.values.back_mut().unwrap() *= 1.01;
                });
            },
            EventName::Covid => {
                *economy.economy.values.back_mut().unwrap() *= 0.95;
            },
            EventName::CryptoFan(name) => {
                economy.cryptos.iter_mut().find(|c| c.name == name).map(|c| {
                    *c.prices.back_mut().unwrap() *= 1.1;
                });
            },
            EventName::Drought(country) => {
                economy.currencies.iter_mut().find(|c| c.country == country).map(|c| {
                    *c.values.back_mut().unwrap() *= 0.97;
                });

                let country = economy.countries.iter().find(|c| c.name == country).unwrap();
                economy
                    .commodities
                    .iter_mut()
                    .filter(|com| com.name.is_food() && country.production.contains_key(&com.name))
                    .for_each(|c| {
                        *c.prices.back_mut().unwrap() *= 1.04;
                    });
            },
            EventName::GoldRush => {
                economy
                    .commodities
                    .iter_mut()
                    .filter(|c| matches!(c.name, CommodityName::Gold | CommodityName::Silver))
                    .for_each(|c| {
                        *c.prices.back_mut().unwrap() *= 1.01;
                    });
            },
            EventName::Grounded => {
                economy.stocks.iter_mut().find(|s| s.issuer == Company::Boeing).map(|s| {
                    *s.prices.back_mut().unwrap() *= 0.97;
                });
            },
            EventName::Harvest(country) => {
                economy.currencies.iter_mut().find(|c| c.country == country).map(|c| {
                    *c.values.back_mut().unwrap() *= 1.03;
                });

                let country = economy.countries.iter().find(|c| c.name == country).unwrap();
                economy
                    .commodities
                    .iter_mut()
                    .filter(|com| com.name.is_food() && country.production.contains_key(&com.name))
                    .for_each(|c| {
                        *c.prices.back_mut().unwrap() *= 0.96;
                    });
            },
            EventName::OilDisruption => {
                economy.commodities.iter_mut().filter(|c| c.name == CommodityName::Oil).for_each(
                    |c| {
                        *c.prices.back_mut().unwrap() *= 1.2;
                    },
                );

                economy.currencies.iter_mut().find(|c| c.country == CountryName::SaudiArabia).map(
                    |c| {
                        *c.values.back_mut().unwrap() *= 0.95;
                    },
                );

                economy.sectors.iter_mut().find(|s| s.name == SectorName::Energy).map(|s| {
                    s.update(-5);
                });
            },
            EventName::Recession => {
                *economy.economy.values.back_mut().unwrap() -= 2.;
            },
            _ => (),
        }
    }

    pub fn end(&self, economy: &mut GlobalEconomy) {
        let mut rng = rng();
        match self.name {
            EventName::BrazilPolitics => {
                economy
                    .bonds
                    .iter_mut()
                    .find(|b| b.issuer == BondIssuer::Government(CountryName::Brazil))
                    .map(|b| {
                        b.quality = BondQuality::BBB;
                    });
            },
            EventName::Covid => {
                economy.sectors.iter_mut().find(|s| s.name == SectorName::Transport).map(|s| {
                    s.update(20);
                });
            },
            EventName::EsgScandal(company) => {
                economy.stocks.iter_mut().find(|s| s.issuer == company).map(|s| {
                    s.esg.increase();
                });
            },
            EventName::Grounded => {
                economy.sectors.iter_mut().find(|s| s.name == SectorName::Transport).map(|s| {
                    s.update(10);
                });
            },
            EventName::InterestBump(country) => {
                economy
                    .bonds
                    .iter_mut()
                    .filter(|b| b.issuer == BondIssuer::Government(country))
                    .for_each(|b| {
                        b.interest *= rng.random_range(0.75..0.85);
                    });
            },
            EventName::RussiaWar => {
                economy
                    .currencies
                    .iter_mut()
                    .find(|c| matches!(c.country, CountryName::Russia | CountryName::Ukraine))
                    .map(|c| {
                        c.base_value *= rng.random_range(1.2..1.5);
                        *c.values.back_mut().unwrap() *= rng.random_range(1.2..1.5);
                    });

                economy
                    .bonds
                    .iter_mut()
                    .find(|b| b.issuer == BondIssuer::Government(CountryName::Russia))
                    .map(|b| {
                        b.quality = BondQuality::CCC;
                    });

                economy.sectors.iter_mut().find(|s| s.name == SectorName::Military).map(|s| {
                    s.update(-rng.random_range(15..25));
                });
            },
            EventName::TradeWar => {
                economy
                    .currencies
                    .iter_mut()
                    .find(|c| matches!(c.country, CountryName::China | CountryName::USA))
                    .map(|c| {
                        c.base_value *= rng.random_range(1.1..1.2);
                        *c.values.back_mut().unwrap() *= rng.random_range(1.1..1.2);
                    });

                economy.sectors.iter_mut().find(|s| s.name == SectorName::Finance).map(|s| {
                    s.update(10);
                });
            },
            _ => (),
        }
    }
}
