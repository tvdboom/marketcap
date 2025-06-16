use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use crate::core::instruments::Instrument;
use crate::utils::NameFromEnum;

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum CryptoName {
    #[default]
    Bitcoin,
    Cardano,
    Dogecoin,
    Ethereum,
    Pepe,
    Solana,
    Toncoin,
    Tron,
    Uniswap,
    USDC,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Crypto {
    /// The name of the cryptocurrency
    pub name: CryptoName,
    
    /// The prices over time
    pub prices: Vec<f32>,

    /// Percentage of price that can change daily
    pub volatility: f32,
}

impl Crypto {
    pub fn description(&self) -> &str {
        match self {
            CryptoName::Bitcoin => "\
            "
        }
    }
}

impl Instrument for Crypto {
    fn name(&self) -> String {
        self.name.to_name()
    }

    fn lowername(&self) -> String {
        self.name.to_lowername()
    }

    fn all(&self) -> &Vec<f32> {
        &self.prices
    }
    
    fn current(&self) -> f32 {
        *self.prices.last().unwrap()
    }
}

pub fn start_cryptos() -> Vec<Crypto> {
    vec![Crypto {
        name: CryptoName::Bitcoin,
        prices: vec![100000.],
        volatility: 0.05,
    }]
}
