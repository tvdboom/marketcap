use bevy::prelude::*;
use chrono::{Datelike, Duration, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::core::constants::{DEFAULT_SPEED, START_DATE};
use crate::core::countries::{Country, start_countries};
use crate::core::events::{EconomicEvent, EventName};
use crate::core::factors::economy::Economy;
use crate::core::factors::inflation::Inflation;
use crate::core::factors::interest::Interest;
use crate::core::instruments::bonds::{Bond, BondKind, start_bonds};
use crate::core::instruments::commodities::{Commodity, start_commodities};
use crate::core::instruments::crypto::{Crypto, start_cryptos};
use crate::core::instruments::forex::{Currency, start_currencies};
use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::core::instruments::stocks::{Stock, start_stocks};
use crate::core::loans::Term;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::orders::{Command, Order, OrderEv, OrderKind, OrderStatus};
use crate::core::player::Player;
use crate::core::research::TechName;
use crate::core::sectors::{Sector, start_sectors};
use crate::core::ui::state::UiState;
use crate::utils::create_guid;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PoliticalLandscape {
    pub government: i32,
    pub ideology: i32,
    pub culture: i32,
    pub orientation: i32,
}

impl PoliticalLandscape {
    pub const RANGE: i32 = 50;
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct GlobalEconomy {
    /// Current in-game date
    pub date: NaiveDate,

    /// Timer for the game clock
    pub clock: Timer,

    /// Global economy (0-100)
    pub economy: Economy,

    /// Inflation rate (1-10)
    pub inflation: Inflation,

    /// Global interest rate (1-10)
    pub interest: Interest,

    /// Information about all countries
    pub countries: Vec<Country>,

    /// Sectors of the economy
    pub sectors: Vec<Sector>,

    /// Information about all stocks
    pub stocks: Vec<Stock>,

    /// Information about all bonds
    pub bonds: Vec<Bond>,

    /// Currencies and their exchange rates
    pub currencies: Vec<Currency>,

    /// Information of all commodities
    pub commodities: Vec<Commodity>,

    /// Information about all cryptocurrencies
    pub cryptos: Vec<Crypto>,

    /// Political factors affecting the global economy
    pub politics: PoliticalLandscape,

    /// Currently active events
    pub events: Vec<EconomicEvent>,
}

impl GlobalEconomy {
    /// Daily changes in the global economy
    pub fn bump(
        &mut self,
        aum: f32,
        state: &mut UiState,
        player: &mut Player,
        order_ev: &mut EventWriter<OrderEv>,
        message: &mut EventWriter<MessageEv>,
    ) -> (f32, f32, f32) {
        let economy = self.economy.bump(aum);
        let interest = self.interest.bump();
        let inflation = self.inflation.bump(economy, interest);

        for commodity in &mut self.commodities {
            commodity.bump(economy, inflation);
        }

        for sector in &mut self.sectors {
            sector.bump(inflation, &self.commodities, &self.politics);
        }

        for crypto in &mut self.cryptos {
            let price = crypto.current();

            crypto.bump(inflation);

            if price != 0. && crypto.current() == 0. {
                let instrument = InstrumentKind::Crypto(crypto.name);
                if state.modal == Some(instrument) {
                    state.modal = None;
                }

                if player.has_tech(&TechName::Cryptocurrencies) {
                    // Close short selling position, returning maximum profit
                    let amount = player.get_owned(&instrument);
                    if amount < 0 {
                        let id = create_guid();
                        player.orders.push(Order {
                            id: id.clone(),
                            created: self.date,
                            instrument,
                            command: Command::Buy,
                            kind: OrderKind::MarketOrder,
                            amount,
                            price: 0.,
                            interest: 0.,
                            cds: false,
                            term: Term::default(),
                            threshold: 0.,
                            bound: 0.,
                            lower_bound: false,
                            loan: None,
                            processed: self.date,
                            status: OrderStatus::Executed,
                        });

                        order_ev.write(OrderEv {
                            id,
                            price: 0.,
                        });
                    }

                    message.write(MessageEv {
                        message: format!(
                            "The cryptocurrency {} has become worthless and cannot be traded anymore.",
                            crypto.name()
                        ),
                        level: MessageLevel::Warning,
                    });
                }
            }
        }

        for stock in &mut self.stocks {
            stock.bump(inflation, &self.sectors);
        }

        for currency in &mut self.currencies {
            currency.bump(&self.countries, &self.commodities, &self.politics);
        }

        for bond in &mut self.bonds {
            bond.bump(&self.currencies);
        }

        if self.date.day() == 1 && self.date.month() % 6 == 1 {
            for bond in self.bonds.iter_mut().filter(|b| b.kind() == BondKind::Government) {
                bond.issue(interest, &self.stocks, &self.currencies);
            }

            if self.date != START_DATE + Duration::days(1) {
                // Skip the first issue on the first day
                message.write(MessageEv {
                    message: "New government bonds have been issued.".to_string(),
                    level: MessageLevel::Info,
                });
            }
        }

        if self.date.day() == 1 && self.date.month() == 1 {
            for bond in self.bonds.iter_mut().filter(|b| b.kind() == BondKind::Corporate) {
                bond.issue(interest, &self.stocks, &self.currencies);
            }

            if self.date != START_DATE + Duration::days(1) {
                // Skip the first issue on the first day
                message.write(MessageEv {
                    message: "New corporate bonds have been issued.".to_string(),
                    level: MessageLevel::Info,
                });
            }
        }

        (economy, inflation, interest)
    }

    pub fn get(&self, instrument: &InstrumentKind) -> &dyn Instrument {
        match instrument {
            InstrumentKind::Stock(issuer) => {
                self.stocks.iter().find(|c| c.issuer == *issuer).unwrap()
            },
            InstrumentKind::Bond(issuer) => {
                self.bonds.iter().find(|b| b.issuer == *issuer).unwrap()
            },
            InstrumentKind::Forex(name) => {
                self.currencies.iter().find(|c| c.name == *name).unwrap()
            },
            InstrumentKind::Commodity(name) => {
                self.commodities.iter().find(|c| c.name == *name).unwrap()
            },
            InstrumentKind::Crypto(name) => self.cryptos.iter().find(|c| c.name == *name).unwrap(),
        }
    }

    pub fn get_price(&self, instrument: &InstrumentKind) -> f32 {
        self.get(instrument).current()
    }

    pub fn active_events(&self) -> Vec<&EconomicEvent> {
        self.events.iter().filter(|e| e.is_active(&self.date)).collect()
    }

    pub fn has_active_event(&self, name: &EventName) -> bool {
        self.active_events().iter().any(|e| e.name == *name)
    }

    pub fn historical_events(&self) -> Vec<&EconomicEvent> {
        self.events
            .iter()
            .filter(|e| self.date >= e.start_date + Duration::days(e.duration as i64))
            .collect()
    }
}

impl Default for GlobalEconomy {
    fn default() -> Self {
        Self {
            date: START_DATE,
            clock: Timer::new(
                std::time::Duration::from_secs_f32(DEFAULT_SPEED),
                TimerMode::Repeating,
            ),
            economy: Economy::default(),
            inflation: Inflation::default(),
            interest: Interest::default(),
            countries: start_countries(),
            sectors: start_sectors(),
            stocks: start_stocks(),
            bonds: start_bonds(),
            commodities: start_commodities(),
            currencies: start_currencies(),
            cryptos: start_cryptos(),
            politics: PoliticalLandscape::default(),
            events: Vec::new(),
        }
    }
}
