use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::utils::NameFromEnum;

#[derive(EnumIter, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum CryptoName {
    #[default]
    Bitcoin,
    Cardano,
    Dogecoin,
    Ethereum,
    Litecoin,
    Pepe,
    Solana,
    Toncoin,
    Tron,
    Uniswap,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Crypto {
    /// The name of the cryptocurrency
    pub name: CryptoName,

    /// Default price of the commodity
    pub base_price: f32,

    /// The prices over time
    pub prices: Vec<f32>,

    /// Percentage of the base price that can change daily
    pub volatility: f32,

    /// Market capitalization of the coin in thousands of euros
    pub market_cap: f32,
}

impl Crypto {
    pub fn bump(&mut self, inflation: f32) -> f32 {
        let new_price = if self.current() == 0. {
            self.market_cap = 0.;
            0. // If the price is zero, the coin is considered dead and cannot be traded
        } else {
            self.base_price *= 1. + inflation / 100. / 365.;

            let volatility = self.base_price * self.volatility / 100.;
            (self.current() * (1. + inflation / 100. / 365.)
                + rng().random_range(-volatility..volatility))
            .max(0.)
        };

        self.prices.push(new_price);
        new_price
    }
}

impl Instrument for Crypto {
    fn name(&self) -> String {
        self.name.to_name()
    }

    fn lowername(&self) -> String {
        self.name.to_lowername()
    }

    fn description(&self) -> &str {
        match self.name {
            CryptoName::Bitcoin => {
                "Bitcoin is a digital currency launched in 2009 by an unknown person using the \
                name Satoshi Nakamoto. Bitcoin has a fixed supply of 21 million coins, with new \
                coins created through a process called mining, where participants solve \
                computational puzzles to validate transactions and secure the network."
            },
            CryptoName::Cardano => {
                "Cardano is a proof-of-stake blockchain platform launched in 2017 by Charles \
                Hoskinson, a co-founder of Ethereum. It emphasizes academic research, formal \
                methods, and peer-reviewed development."
            },
            CryptoName::Dogecoin => {
                "Dogecoin is a cryptocurrency launched in 2013 as a lighthearted meme-based \
                alternative to Bitcoin, featuring the Shiba Inu dog from the 'Doge' meme. \
                It started as a joke but gained popularity for its active community and use \
                in online tipping and donations. Dogecoin uses proof-of-work mining, has no \
                supply cap, and relies on inflationary issuance."
            },
            CryptoName::Ethereum => {
                "Ethereum was launched in 2015 by Vitalik Buterin, designed for building and \
                running smart contracts and decentralized applications (dApps). Unlike Bitcoin, \
                Ethereum is programmable, enabling complex logic directly on the blockchain."
            },
            CryptoName::Litecoin => {
                "Litecoin is a peer-to-peer cryptocurrency created in 2011 by Charlie Lee as a \
                'lite' version of Bitcoin. It features faster transaction times and a different \
                hashing algorithm (Scrypt) to allow for more efficient mining."
            },
            CryptoName::Pepe => {
                "Pepe is a meme-based cryptocurrency that gained popularity in 2023, inspired by \
                the Pepe the Frog meme. It is often used for community-driven projects and \
                speculative trading."
            },
            CryptoName::Solana => {
                "Solana is a high-performance blockchain platform launched in 2020, known for \
                its fast transaction speeds and low fees. It uses a unique consensus mechanism \
                called Proof of History (PoH) to achieve high throughput."
            },
            CryptoName::Toncoin => {
                "Toncoin is the native cryptocurrency of the TON blockchain, originally \
                developed by Telegram. It aims to provide fast and scalable transactions \
                with a focus on user-friendly applications."
            },
            CryptoName::Tron => {
                "Tron is a blockchain platform launched in 2017, designed for building \
                decentralized applications (dApps) and content sharing. It aims to create a \
                decentralized internet and has its own native cryptocurrency, TRX."
            },
            CryptoName::Uniswap => {
                "Uniswap is a decentralized exchange (DEX) protocol built on the Ethereum \
                blockchain, allowing users to trade cryptocurrencies directly from their \
                wallets without intermediaries. It uses an automated market maker (AMM) model."
            },
        }
    }

    fn kind(&self) -> InstrumentKind {
        InstrumentKind::Crypto(self.name)
    }

    fn all(&self) -> &Vec<f32> {
        &self.prices
    }

    fn current(&self) -> f32 {
        *self.prices.last().unwrap()
    }

    fn market_cap(&self) -> f32 {
        self.market_cap
    }

    fn volatility(&self) -> f32 {
        self.volatility
    }
}

pub fn start_cryptos() -> Vec<Crypto> {
    vec![
        Crypto {
            name: CryptoName::Bitcoin,
            base_price: 100000.,
            prices: vec![100000.],
            volatility: 4.1,
            market_cap: 1.9e12,
        },
        Crypto {
            name: CryptoName::Cardano,
            base_price: 0.64,
            prices: vec![0.64],
            volatility: 5.3,
            market_cap: 19.7e9,
        },
        Crypto {
            name: CryptoName::Dogecoin,
            base_price: 0.18,
            prices: vec![0.18],
            volatility: 7.8,
            market_cap: 24.4e9,
        },
        Crypto {
            name: CryptoName::Ethereum,
            base_price: 2616.,
            prices: vec![2616.],
            volatility: 4.8,
            market_cap: 293e9,
        },
        Crypto {
            name: CryptoName::Litecoin,
            base_price: 87.,
            prices: vec![87.],
            volatility: 6.0,
            market_cap: 6.1e9,
        },
        Crypto {
            name: CryptoName::Pepe,
            base_price: 0.02,
            prices: vec![0.02],
            volatility: 9.6,
            market_cap: 4.7e9,
        },
        Crypto {
            name: CryptoName::Solana,
            base_price: 125.,
            prices: vec![125.],
            volatility: 5.8,
            market_cap: 76.2e9,
        },
        Crypto {
            name: CryptoName::Toncoin,
            base_price: 2.8,
            prices: vec![2.8],
            volatility: 3.1,
            market_cap: 6.8e9,
        },
        Crypto {
            name: CryptoName::Tron,
            base_price: 0.28,
            prices: vec![0.28],
            volatility: 11.2,
            market_cap: 26.9e9,
        },
        Crypto {
            name: CryptoName::Uniswap,
            base_price: 7.3,
            prices: vec![7.3],
            volatility: 8.7,
            market_cap: 4.5e9,
        },
    ]
}
