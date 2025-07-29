use std::collections::HashMap;

use bevy::utils::default;
use rand::{rng, Rng};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::core::factors::inflation::Inflation;
use crate::core::global_economy::PoliticalLandscape;
use crate::core::instruments::commodities::{Commodity, CommodityName};
use crate::core::instruments::instrument::Instrument;
use crate::core::politics::{Culture, Government, Ideology, Orientation, Politics};

#[derive(EnumIter, Clone, Copy, Default, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SectorName {
    #[default]
    Energy,
    Fashion,
    Finance,
    Food,
    Healthcare,
    Materials,
    Military,
    Retail,
    Technology,
    Transport,
}

impl SectorName {
    pub fn emoji(&self) -> &str {
        match self {
            SectorName::Energy => "⚡",
            SectorName::Fashion => "👗",
            SectorName::Finance => "💰",
            SectorName::Food => "🍔",
            SectorName::Healthcare => "💊",
            SectorName::Materials => "🔨",
            SectorName::Military => "🔫",
            SectorName::Retail => "👤",
            SectorName::Technology => "💻",
            SectorName::Transport => "🚚",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            SectorName::Energy => {
                "The energy sector encompasses companies that are involved in the exploration, \
                extraction, production, refinement, and distribution of energy resources essential \
                to modern life. This includes traditional fossil fuels such as oil, natural gas, \
                and coal, as well as renewable and alternative energy sources like wind, solar, \
                hydroelectric, geothermal, and bioenergy. The sector includes upstream companies \
                (exploration and drilling), midstream (transportation and storage), and downstream \
                (refining and marketing), along with utility providers and infrastructure developers \
                focused on global energy supply and sustainability."
            },
            SectorName::Fashion => {
                "The fashion sector includes a diverse range of businesses involved in the \
                conceptualization, design, production, marketing, and retail of apparel, footwear, \
                and accessories. This spans luxury and haute couture houses, fast-fashion brands, \
                and independent designers. It also includes the full supply chain—from textile \
                manufacturing and ethical sourcing of materials to global distribution networks. \
                The sector is increasingly influenced by social trends, cultural movements, \
                sustainability concerns, and the integration of technology through digital \
                fashion, e-commerce platforms, and AI-driven design."
            },
            SectorName::Finance => {
                "The finance sector includes institutions and companies that manage capital flows, \
                risk, and financial services on both individual and institutional levels. This \
                includes commercial banks, investment banks, insurance firms, private equity, \
                hedge funds, credit rating agencies, and financial technology (fintech) startups. \
                These entities offer services such as lending, investing, underwriting, wealth \
                management, currency exchange, and financial advisory. The sector is heavily \
                influenced by the global economy. It's value is not only dependent on primary \
                commodities, but also on the current inflation."
            },
            SectorName::Food => {
                "The food sector encompasses all businesses involved in the production, processing, \
                packaging, transportation, marketing, and sale of food products. This includes \
                agricultural enterprises, aquaculture, food manufacturers, wholesalers, retailers, \
                restaurants, and delivery services. The sector also integrates global supply chain \
                logistics, sustainability practices, food safety standards, and evolving consumer \
                trends such as plant-based diets, organic labeling, and personalized nutrition. \
                Innovation in food technology, biotechnology, and climate-resilient agriculture \
                also plays a growing role."
            },
            SectorName::Healthcare => {
                "The healthcare sector is composed of a wide array of organizations and companies \
                focused on the prevention, diagnosis, treatment, and management of illness and \
                injury. This includes hospitals, clinics, pharmaceutical companies, biotechnology \
                firms, medical device manufacturers, diagnostics providers, and telemedicine \
                platforms. The sector also encompasses public health systems, health insurance \
                providers, and regulatory bodies. It is characterized by rapid innovation, long \
                product development cycles, complex regulation, and an ever-growing demand due to \
                aging populations and global health challenges."
            },
            SectorName::Materials => {
                "The materials sector includes companies that discover, extract, process, and \
                distribute essential raw materials used across industrial and consumer applications. \
                This spans metals and mining (e.g., iron, copper, aluminum, gold), chemicals, \
                forestry products, plastics, cement, and construction materials. These businesses \
                are crucial to infrastructure, manufacturing, energy, and technology sectors. The \
                industry is influenced by global commodity markets, environmental regulations, \
                sustainability initiatives, and geopolitical resource competition."
            },
            SectorName::Military => {
                "The military sector involves corporations that research, develop, manufacture, \
                and maintain defense systems and technologies. This includes arms manufacturers, \
                aerospace contractors, cybersecurity firms, and logistics providers that supply \
                national militaries, intelligence agencies, and defense alliances. The sector \
                plays a pivotal role in national security, geopolitical strategy, and technological \
                innovation. It includes traditional weapons systems, satellite and drone \
                technologies, defense AI, and next-generation surveillance, often under strict \
                government oversight and long-term procurement contracts."
            },
            SectorName::Retail => {
                "The retail sector consists of companies that sell finished goods and services \
                directly to consumers. This includes physical brick-and-mortar stores, online \
                retailers (e-commerce), and hybrid omnichannel operations. It spans categories \
                like apparel, electronics, groceries, furniture, and luxury items. The sector is \
                highly consumer-driven and influenced by trends in disposable income, seasonal \
                demand, marketing strategies, and technological adoption (e.g., mobile commerce, \
                AR shopping, loyalty systems). Retail is a dynamic sector shaped by supply chain \
                efficiency, globalization, and evolving customer experiences."
            },
            SectorName::Technology => {
                "The technology sector comprises companies focused on innovation and the application \
                of science and engineering in software, hardware, and digital infrastructure. It \
                includes developers of operating systems, mobile apps, cloud computing services, \
                AI platforms, semiconductor fabrication, networking equipment, and consumer \
                electronics. The sector also overlaps with fintech, edtech, medtech, and other \
                specialized domains. It is characterized by rapid cycles of innovation, high R&D \
                expenditure, global competition, and significant influence over how businesses and \
                individuals interact with information and the world."
            },
            SectorName::Transport => {
                "The transport sector includes all businesses involved in the movement of people \
                and goods. This spans aviation, maritime shipping, railways, trucking, public \
                transit, freight logistics, and emerging areas such as electric mobility and \
                drone delivery. It also encompasses infrastructure development such as roads, \
                ports, and airports, as well as digital logistics platforms. The sector is \
                sensitive to fuel costs, geopolitical disruptions, environmental regulations, \
                and technological innovations such as autonomous vehicles and smart traffic systems."
            },
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Sector {
    pub name: SectorName,
    pub value: i8,
    pub politics: Politics,
    pub commodities: HashMap<CommodityName, f32>,
}

impl Sector {
    pub const RANGE: i8 = 50;

    pub fn bump(
        &mut self,
        inflation: f32,
        commodities: &Vec<Commodity>,
        landscape: &PoliticalLandscape,
    ) {
        let mut bump = self.politics.get_score(&landscape)
            + (0.01
                * commodities
                    .iter()
                    .map(|c| {
                        self.commodities
                            .get(&c.name)
                            .map(|weight| weight * (c.current() - c.base_price))
                            .unwrap_or_default()
                    })
                    .sum::<f32>());

        // Finance is special since it also depends on inflation
        if self.name == SectorName::Finance {
            bump += 0.01 * (inflation - Inflation::DEFAULT) / 100.;
        }

        if rng().random::<f32>() > bump.abs() {
            if bump > 0. {
                self.value += 1;
            } else {
                self.value -= 1;
            }
        }

        self.value = self.value.clamp(-Self::RANGE, Self::RANGE);
    }

    pub fn update(&mut self, amount: i8) {
        self.value = (self.value + amount).clamp(-Self::RANGE, Self::RANGE);
    }
}

pub fn start_sectors() -> Vec<Sector> {
    vec![
        Sector {
            name: SectorName::Energy,
            value: 0,
            politics: Politics {
                ideology: Ideology::Right,
                culture: Culture::Conservative,
                ..default()
            },
            commodities: HashMap::from([
                (CommodityName::LNG, 0.4),
                (CommodityName::Oil, 0.4),
                (CommodityName::Ethanol, 0.2),
            ]),
        },
        Sector {
            name: SectorName::Fashion,
            value: 0,
            politics: Politics {
                government: Government::Democracy,
                culture: Culture::Progressive,
                orientation: Orientation::Capitalism,
                ..default()
            },
            commodities: HashMap::from([
                (CommodityName::Cotton, 0.6),
                (CommodityName::Silver, 0.2),
                (CommodityName::Gold, 0.1),
            ]),
        },
        Sector {
            name: SectorName::Finance,
            value: 0,
            politics: Politics {
                government: Government::Democracy,
                ideology: Ideology::Right,
                orientation: Orientation::Capitalism,
                ..default()
            },
            commodities: HashMap::from([
                (CommodityName::Gold, 0.3),
                (CommodityName::Silver, 0.2),
                (CommodityName::Silicon, 0.1),
            ]),
        },
        Sector {
            name: SectorName::Food,
            value: 0,
            politics: Politics {
                ideology: Ideology::Left,
                ..default()
            },
            commodities: HashMap::from([
                (CommodityName::Wheat, 0.3),
                (CommodityName::Corn, 0.2),
                (CommodityName::Cocoa, 0.2),
                (CommodityName::Coffee, 0.2),
            ]),
        },
        Sector {
            name: SectorName::Healthcare,
            value: 0,
            politics: Politics {
                government: Government::Democracy,
                ideology: Ideology::Left,
                orientation: Orientation::Socialism,
                ..default()
            },
            commodities: HashMap::from([
                (CommodityName::Ethanol, 0.2),
                (CommodityName::LNG, 0.1),
                (CommodityName::Silver, 0.1),
                (CommodityName::Corn, 0.1),
                (CommodityName::Cotton, 0.1),
            ]),
        },
        Sector {
            name: SectorName::Materials,
            value: 0,
            politics: Politics {
                culture: Culture::Conservative,
                ..default()
            },
            commodities: HashMap::from([
                (CommodityName::Gold, 0.1),
                (CommodityName::Silver, 0.1),
                (CommodityName::Iron, 0.1),
                (CommodityName::Silicon, 0.1),
                (CommodityName::Aluminium, 0.1),
                (CommodityName::Copper, 0.1),
            ]),
        },
        Sector {
            name: SectorName::Military,
            value: 0,
            politics: Politics {
                government: Government::Autocracy,
                ideology: Ideology::Right,
                culture: Culture::Conservative,
                orientation: Orientation::Capitalism,
            },
            commodities: HashMap::from([
                (CommodityName::Iron, 0.5),
                (CommodityName::Aluminium, 0.3),
                (CommodityName::Oil, 0.3),
            ]),
        },
        Sector {
            name: SectorName::Retail,
            value: 0,
            politics: Politics {
                government: Government::Democracy,
                culture: Culture::Progressive,
                orientation: Orientation::Capitalism,
                ..default()
            },
            commodities: HashMap::from([
                (CommodityName::Cotton, 0.2),
                (CommodityName::Silver, 0.1),
                (CommodityName::Aluminium, 0.1),
                (CommodityName::Coffee, 0.1),
                (CommodityName::Cocoa, 0.1),
            ]),
        },
        Sector {
            name: SectorName::Technology,
            value: 0,
            politics: Politics {
                culture: Culture::Progressive,
                orientation: Orientation::Capitalism,
                ..default()
            },
            commodities: HashMap::from([
                (CommodityName::Silicon, 0.7),
                (CommodityName::Copper, 0.3),
            ]),
        },
        Sector {
            name: SectorName::Transport,
            value: 0,
            politics: Politics {
                government: Government::Democracy,
                ..default()
            },
            commodities: HashMap::from([
                (CommodityName::Iron, 0.4),
                (CommodityName::Aluminium, 0.25),
                (CommodityName::Oil, 0.25),
                (CommodityName::Copper, 0.1),
            ]),
        },
    ]
}
