use std::time::Duration;

use bevy::prelude::*;
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::core::constants::DEFAULT_SPEED;
use crate::core::countries::{Country, start_countries};
use crate::core::factors::Factor;
use crate::core::factors::economy::Economy;
use crate::core::factors::inflation::Inflation;
use crate::core::factors::interest::Interest;
use crate::core::instruments::bonds::{Bond, BondKind, start_bonds};
use crate::core::instruments::commodities::{Commodity, start_commodities};
use crate::core::instruments::crypto::{Crypto, start_cryptos};
use crate::core::instruments::forex::{Currency, start_currencies};
use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::core::instruments::stocks::{Stock, start_stocks};
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::sectors::{Sector, start_sectors};
use crate::core::ui::state::UiState;

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct GlobalEconomy {
    /// Current in-game date
    pub date: NaiveDate,

    /// Timer for the game clock
    pub clock: Timer,

    /// Global economic factor (0-100)
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
}

impl GlobalEconomy {
    /// Daily changes in the global economy
    pub fn bump(
        &mut self,
        aum: f32,
        state: &mut UiState,
        message: &mut EventWriter<MessageEv>,
    ) -> (f32, f32, f32) {
        let economy = self.economy.bump(aum);
        let interest = self.interest.bump();
        let inflation = self.inflation.bump(economy, interest);

        for commodity in &mut self.commodities {
            commodity.bump(economy, inflation);
        }

        for sector in &mut self.sectors {
            sector.bump(inflation, &self.commodities);
        }

        for crypto in &mut self.cryptos {
            let price = crypto.current();
            crypto.bump(inflation);
            if price != 0. && crypto.current() == 0. {
                if state.modal == Some(InstrumentKind::Crypto(crypto.name)) {
                    state.modal = None;
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

        for stock in &mut self.stocks {
            stock.bump(inflation, &self.sectors);
        }

        for currency in &mut self.currencies {
            currency.bump(&self.countries, &self.commodities);
        }

        if self.date.day() == 1 && self.date.month() % 6 == 1 {
            for bond in self
                .bonds
                .iter_mut()
                .filter(|b| b.kind() == BondKind::Government)
            {
                bond.issue(&self.currencies, self.interest.current());
            }

            message.write(MessageEv {
                message: "New government bonds have been issued.".to_string(),
                level: MessageLevel::Info,
            });
        }

        if self.date.day() == 1 && self.date.month() == 1 {
            for bond in self
                .bonds
                .iter_mut()
                .filter(|b| b.kind() == BondKind::Corporate)
            {
                bond.issue(&self.currencies, self.interest.current());
            }

            message.write(MessageEv {
                message: "New corporate bonds have been issued.".to_string(),
                level: MessageLevel::Info,
            });
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
}

impl Default for GlobalEconomy {
    fn default() -> Self {
        Self {
            date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            clock: Timer::new(Duration::from_secs_f32(DEFAULT_SPEED), TimerMode::Repeating),
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
        }
    }
}
