use crate::core::constants::DEFAULT_SPEED;
use crate::core::ui::systems::Tab;
use bevy::prelude::*;
use chrono::{Local, NaiveDate};
use rand::{Rng, rng};
use std::time::Duration;

#[derive(Clone)]
pub struct GlobalEconomicFactor(Vec<f32>);

impl GlobalEconomicFactor {
    const MIN: f32 = 0.;
    const MAX: f32 = 100.;

    pub fn description(&self) -> &str {
        "Global economic factor\n\n\
        The global economy represents the overall financial health and activity of the \
        world. It fluctuates based on trade, market sentiment and events. A strong global \
        economy means higher consumer confidence, robust industry growth, and increased \
        investments. A weak global economy signals recessions, crises, or reduced spending, \
        making businesses expansion difficult.\n\n\
        In the game, the global economic factor (0-100) serves as a macro-scale indicator, \
        affecting stock markets and interest rates dynamically."
    }

    pub fn current(&self) -> f32 {
        *self.0.last().unwrap()
    }

    pub fn bump(&mut self) {
        self.0
            .push((self.current() + rng().random_range(-2.5..2.5)).clamp(Self::MIN, Self::MAX))
    }
}

impl Default for GlobalEconomicFactor {
    fn default() -> Self {
        Self(Vec::from([(Self::MIN + Self::MAX) * 0.5]))
    }
}

#[derive(Clone)]
pub struct GlobalInterestRate(Vec<f32>);

impl GlobalInterestRate {
    const MIN: f32 = 2.;
    const MAX: f32 = 10.;
    const ADJUSTMENT_RATE: f32 = 0.1;

    pub fn description(&self) -> &str {
        "Global interest rate\n\n\
        The global interest rate is set by the central bank and determines the cost of \
        borrowing money. It rises when the economy struggles, making loans expensive, and \
        falls when the economy thrives, encouraging investment.\n\n\
        In the game, interest rates directly impact debt strategies. Players must decide \
        whether to take loans during cheap borrowing periods or avoid debt when rates rise."
    }

    pub fn current(&self) -> f32 {
        *self.0.last().unwrap()
    }

    pub fn bump(&mut self, global_economic_factor: f32) {
        self.0.push(
            self.current()
                + Self::ADJUSTMENT_RATE
                    * (Self::MAX
                        - ((Self::MAX - Self::MIN) * global_economic_factor
                            / GlobalEconomicFactor::MAX)
                        - self.current()),
        )
    }
}

impl Default for GlobalInterestRate {
    fn default() -> Self {
        Self(Vec::from([(Self::MIN + Self::MAX) * 0.5]))
    }
}

#[derive(Resource, Clone)]
pub struct GameParams {
    /// Current in-game date
    pub date: NaiveDate,

    /// Timer for the game clock
    pub clock: Timer,

    /// Active tab in the game's UI
    pub tab: Tab,

    /// Economic factor (0-100)
    pub economic_factor: GlobalEconomicFactor,

    /// Interest rate (2-10)
    pub interest_rate: GlobalInterestRate,
}

impl Default for GameParams {
    fn default() -> Self {
        Self {
            date: Local::now().date_naive(),
            clock: Timer::new(Duration::from_secs_f32(DEFAULT_SPEED), TimerMode::Repeating),
            tab: Tab::Home,
            economic_factor: GlobalEconomicFactor::default(),
            interest_rate: GlobalInterestRate::default(),
        }
    }
}
