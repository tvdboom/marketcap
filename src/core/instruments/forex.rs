use std::fmt::Display;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::constants::CURRENCY;
use crate::core::countries::{Country, CountryName};
use crate::core::instruments::commodities::Commodity;
use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::utils::{DQueue, NameFromEnum};

#[derive(EnumIter, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CurrencyName {
    AUD,
    BRL,
    CAD,
    CNY,
    EUR,
    JPY,
    RUB,
    SAR,
    UAH,
    USD,
    VES,
    ZAR,
}

impl CurrencyName {
    pub fn symbol(&self) -> &str {
        match self {
            CurrencyName::AUD => "A$",
            CurrencyName::BRL => "R$",
            CurrencyName::CAD => "C$",
            CurrencyName::CNY => "¥",
            CurrencyName::EUR => "€",
            CurrencyName::JPY => "¥",
            CurrencyName::RUB => "₽",
            CurrencyName::SAR => "﷼",
            CurrencyName::UAH => "₴",
            CurrencyName::USD => "$",
            CurrencyName::VES => "Bs",
            CurrencyName::ZAR => "R",
        }
    }

    pub fn fullname(&self) -> &str {
        match self {
            CurrencyName::AUD => "Australian Dollar",
            CurrencyName::BRL => "Brazilian Real",
            CurrencyName::CAD => "Canadian Dollar",
            CurrencyName::CNY => "Chinese Yuan",
            CurrencyName::EUR => "European Euro",
            CurrencyName::JPY => "Japanese Yen",
            CurrencyName::RUB => "Russian Ruble",
            CurrencyName::SAR => "Saudi Arabian Riyal",
            CurrencyName::UAH => "Ukrainian Hryvnia",
            CurrencyName::USD => "United States Dollar",
            CurrencyName::VES => "Venezuelan Bolivar",
            CurrencyName::ZAR => "South African Rand",
        }
    }
}

impl Display for CurrencyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Currency {
    /// The currency's name
    pub name: CurrencyName,

    /// The country that uses this currency
    pub country: CountryName,

    /// Default value of the currency per euro
    pub base_value: f32,

    /// The values over time per euro
    pub values: DQueue<f32>,
}

impl Currency {
    pub fn bump(&mut self, countries: &Vec<Country>, commodities: &Vec<Commodity>) -> f32 {
        if self.name == CURRENCY {
            return self.current(); // The base currency doesn't change
        }

        let country = countries.iter().find(|c| c.name == self.country).unwrap();
        let mut new_value = self.current()
            + (1.
                * country
                    .production
                    .iter()
                    .map(|(n, w)| {
                        commodities
                            .iter()
                            .find(|c| c.name == *n)
                            .map_or(0., |c| w * (c.current() - c.base_price) / c.base_price)
                    })
                    .sum::<f32>()
                / 100.);

        // Adjust value to tend towards the base value
        let deviation = (new_value - self.base_value) / self.base_value;
        new_value *= 1. + -deviation * deviation.abs() / 10.;

        new_value = new_value.max(0.001);

        self.values.push(new_value);
        new_value
    }
}

impl Instrument for Currency {
    fn name(&self) -> String {
        self.name.to_name()
    }

    fn lowername(&self) -> String {
        self.name.to_lowername()
    }

    fn fullname(&self) -> String {
        self.name.fullname().to_string()
    }

    fn description(&self) -> &str {
        match self.name {
            CurrencyName::AUD => {
                "\
                The Australian dollar (AUD) is a commodity-linked currency heavily influenced \
                by global demand for resources like iron ore, and gold. It's considered a \
                risk-sensitive currency, often gaining value in times of global economic \
                optimism and declining during risk-off sentiment. The Reserve Bank of Australia \
                (RBA) plays a major role in guiding its value through interest rate policy. \
                AUD is actively traded in the Asia-Pacific region and is popular in carry trades \
                due to historically higher interest rates compared to other developed economies."
            },
            CurrencyName::BRL => {
                "\
                The Brazilian real (BRL) is the official currency of Brazil and is considered an \
                emerging market currency. It's influenced by commodity prices, especially Brazil's \
                key exports: coffee and cocoa. The real is sensitive to political risk, inflation, \
                and fiscal policy, with the Central Bank of Brazil playing a key role through \
                interest rate adjustments. BRL tends to be volatile and is often used by traders \
                to gain exposure to emerging markets or to capitalize on carry trade opportunities \
                during periods of high domestic interest rates."
            },
            CurrencyName::CAD => {
                "\
                The Canadian dollar (CAD) is a commodity-linked currency closely tied to the \
                price of oil, as Canada is one of the world's largest oil exporters. It's \
                considered a stable and highly liquid currency, actively traded during North \
                American market hours. While relatively less volatile than some emerging market \
                currencies, CAD still responds to shifts in global risk sentiment and U.S. \
                economic data due to the countries' close trade relationship."
            },
            CurrencyName::CNY => {
                "\
                The Chinese yuan (CNY), also known as the renminbi (RMB), is the official currency \
                of the People's Republic of China. It is partially managed by the People's Bank \
                of China, with a tightly controlled exchange rate regime that allows limited \
                daily movement against a basket of currencies. While not fully convertible, the \
                yuan's global influence has grown due to China's economic size and trade \
                relationships. The currency is sensitive to government policy, trade balances, \
                and geopolitical tensions."
            },
            CurrencyName::EUR => {
                "\
                The euro (EUR) is the official currency of the Eurozone, used by 20 of the 27 EU \
                member states, making it the second most traded and held currency globally after \
                the U.S. dollar. Issued by the European Central Bank (ECB), the euro is seen as \
                a stable, low-volatility currency backed by a large, diversified economy. Its \
                value is influenced by ECB monetary policy, inflation data, and political \
                developments within the EU."
            },
            CurrencyName::JPY => {
                "\
                The Japanese yen (JPY) is one of the most traded currencies globally, known for \
                its high liquidity and role as a safe-haven asset. It is heavily influenced by \
                monetary policy from the Bank of Japan, which has maintained ultra-low interest \
                rates for decades. The yen tends to strengthen during periods of global uncertainty \
                as investors seek safety. It's also widely used in carry trades due to its \
                historically low yields. JPY trading is most active during Asian market hours \
                and is a key currency in global forex pairs, especially against USD and EUR."
            },
            CurrencyName::RUB => {
                "\
                The Russian ruble (RUB) is the official currency of the Russian Federation and \
                is considered a high-volatility emerging market currency. It is heavily influenced \
                by global energy prices—especially oil and gas exports — as well as geopolitical \
                developments and domestic fiscal policy. The Central Bank of Russia manages the \
                ruble through interest rates and, at times, capital controls. Sanctions, political \
                risk, and trade restrictions can cause sharp movements in RUB."
            },
            CurrencyName::SAR => {
                "\
                The Saudi riyal (SAR) value is indirectly influenced by global oil prices, as \
                Saudi Arabia's economy is heavily reliant on petroleum exports. While not widely \
                traded internationally, the SAR is important in regional finance and reflects \
                the kingdom's fiscal health and oil market dynamics."
            },
            CurrencyName::UAH => {
                "\
                The Ukrainian hryvnia (UAH) is considered an emerging market currency with high \
                volatility due to ongoing political and economic instability. While less liquid \
                than major currencies, UAH plays a crucial role in Ukraine's economy and regional \
                trade."
            },
            CurrencyName::USD => {
                "\
                The U.S. dollar (USD) is the world's primary reserve and most traded currency, \
                widely used in global trade, finance, and as a benchmark currency. Issued by the \
                Federal Reserve, its value is influenced by U.S. economic data, interest rates, \
                and geopolitical events. The dollar acts as a safe haven during market uncertainty \
                and is central to many commodities priced globally. USD liquidity is highest \
                during overlapping North American and European trading sessions, making it the \
                dominant currency in forex markets."
            },
            CurrencyName::VES => {
                "\
                The Venezuelan bolívar (VES) is the official currency of Venezuela and is highly \
                volatile due to severe hyperinflation, political instability, and economic crisis. \
                Its value has drastically depreciated over recent years, leading to widespread \
                dollarization in the economy. The Central Bank of Venezuela has limited ability to \
                stabilize the currency, and exchange rates vary significantly between official and \
                black markets. Despite reforms, the bolívar remains weak and illiquid, reflecting \
                the country's ongoing economic challenges."
            },
            CurrencyName::ZAR => {
                "\
                The South African rand (ZAR) is the official currency of South Africa and a key \
                emerging market currency. It is commodity-sensitive, influenced heavily by prices \
                of gold and other minerals, which are major exports. The rand is known for its \
                volatility due to political risk, domestic economic challenges, and shifts in \
                global risk sentiment. It is actively traded during African and European market \
                hours and often used by traders seeking exposure to African markets."
            },
        }
    }

    fn kind(&self) -> InstrumentKind {
        InstrumentKind::Forex(self.name)
    }

    fn all(&self) -> &DQueue<f32> {
        &self.values
    }

    fn symbol(&self) -> &str {
        self.name.symbol()
    }
}

pub fn start_currencies() -> Vec<Currency> {
    vec![
        Currency {
            name: CurrencyName::AUD,
            country: CountryName::Australia,
            base_value: 0.56,
            values: DQueue::from([0.56]),
        },
        Currency {
            name: CurrencyName::BRL,
            country: CountryName::Brazil,
            base_value: 0.16,
            values: DQueue::from([0.16]),
        },
        Currency {
            name: CurrencyName::CAD,
            country: CountryName::Canada,
            base_value: 0.6,
            values: DQueue::from([0.6]),
        },
        Currency {
            name: CurrencyName::CNY,
            country: CountryName::China,
            base_value: 0.12,
            values: DQueue::from([0.12]),
        },
        Currency {
            name: CurrencyName::EUR,
            country: CountryName::EU,
            base_value: 1.0,
            values: DQueue::from([1.0]),
        },
        Currency {
            name: CurrencyName::JPY,
            country: CountryName::Japan,
            base_value: 0.006,
            values: DQueue::from([0.006]),
        },
        Currency {
            name: CurrencyName::RUB,
            country: CountryName::Russia,
            base_value: 0.01,
            values: DQueue::from([0.01]),
        },
        Currency {
            name: CurrencyName::SAR,
            country: CountryName::SaudiArabia,
            base_value: 0.23,
            values: DQueue::from([0.23]),
        },
        Currency {
            name: CurrencyName::UAH,
            country: CountryName::Ukraine,
            base_value: 0.02,
            values: DQueue::from([0.02]),
        },
        Currency {
            name: CurrencyName::USD,
            country: CountryName::USA,
            base_value: 0.85,
            values: DQueue::from([0.85]),
        },
        Currency {
            name: CurrencyName::VES,
            country: CountryName::Venezuela,
            base_value: 0.008,
            values: DQueue::from([0.008]),
        },
        Currency {
            name: CurrencyName::ZAR,
            country: CountryName::SouthAfrica,
            base_value: 0.05,
            values: DQueue::from([0.05]),
        },
    ]
}
