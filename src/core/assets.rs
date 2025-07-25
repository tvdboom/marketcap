use std::collections::HashMap;

use bevy::asset::{AssetServer, Handle};
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use strum::IntoEnumIterator;

use crate::core::countries::CountryName;
use crate::core::instruments::commodities::CommodityName;
use crate::core::instruments::crypto::CryptoName;
use crate::core::instruments::forex::CurrencyName;
use crate::core::instruments::stocks::Company;
use crate::utils::NameFromEnum;

pub struct WorldAssets {
    pub audio: HashMap<&'static str, Handle<AudioSource>>,
    pub images: HashMap<&'static str, Handle<Image>>,
}

impl WorldAssets {
    fn get_asset<'a, T: Clone>(
        &self,
        map: &'a HashMap<&str, T>,
        name: &str,
        asset_type: &str,
    ) -> &'a T {
        map.get(name).expect(&format!("No asset for {asset_type} {name}"))
    }

    pub fn audio(&self, name: &str) -> Handle<AudioSource> {
        self.get_asset(&self.audio, name, "audio").clone_weak()
    }
}

impl FromWorld for WorldAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.get_resource::<AssetServer>().unwrap();

        let audio = HashMap::from([
            ("button", assets.load("audio/button.ogg")),
            ("message", assets.load("audio/message.ogg")),
            ("warning", assets.load("audio/warning.ogg")),
            ("error", assets.load("audio/error.ogg")),
            ("defeat", assets.load("audio/defeat.ogg")),
            ("music", assets.load("audio/music.ogg")),
        ]);

        let mut images: HashMap<&'static str, Handle<Image>> = HashMap::from([
            // Splash art
            ("cover", assets.load("images/cover.png")),
            ("trading", assets.load("images/trading.png")),
            ("victory", assets.load("images/victory.png")),
            ("game-over", assets.load("images/game-over.png")),
            // Icons
            ("aum", assets.load("images/icons/aum.png")),
            ("cash", assets.load("images/icons/cash.png")),
            ("credit-score", assets.load("images/icons/credit-score.png")),
            ("economic", assets.load("images/icons/economic.png")),
            ("inflation", assets.load("images/icons/inflation.png")),
            ("influence", assets.load("images/icons/influence.png")),
            ("interest", assets.load("images/icons/interest.png")),
            ("logo", assets.load("images/icons/logo.png")),
            ("netflow", assets.load("images/icons/netflow.png")),
            ("research", assets.load("images/icons/research.png")),
            ("time", assets.load("images/icons/time.png")),
            ("time-paused", assets.load("images/icons/time-paused.png")),
            // Events
            ("brazil-politics", assets.load("images/events/brazil-politics.png")),
            ("ceo-resignation", assets.load("images/events/ceo-resignation.png")),
            ("covid", assets.load("images/events/covid.png")),
            ("crimea", assets.load("images/events/crimea.png")),
            ("crypto-crash", assets.load("images/events/crypto-crash.png")),
            ("crypto-fan", assets.load("images/events/crypto-fan.png")),
            ("ddos", assets.load("images/events/ddos.png")),
            ("drought", assets.load("images/events/drought.png")),
            ("esg-scandal", assets.load("images/events/esg-scandal.png")),
            ("gas-discovery", assets.load("images/events/gas-discovery.png")),
            ("gold-rush", assets.load("images/events/gold-rush.png")),
            ("grounded", assets.load("images/events/grounded.png")),
            ("harvest", assets.load("images/events/harvest.png")),
            ("interest-bump", assets.load("images/events/interest-bump.png")),
            ("mining-strike", assets.load("images/events/mining-strike.png")),
            ("merger", assets.load("images/events/merger.png")),
            ("new-contract", assets.load("images/events/new-contract.png")),
            ("new-product", assets.load("images/events/new-product.png")),
            ("oil-discovery", assets.load("images/events/oil-discovery.png")),
            ("oil-disruption", assets.load("images/events/oil-disruption.png")),
            ("rail", assets.load("images/events/rail.png")),
            ("recession", assets.load("images/events/recession.png")),
            ("regulatory-crackdown", assets.load("images/events/regulatory-crackdown.png")),
            ("russia-war", assets.load("images/events/russia-war.png")),
            ("sovereign-debt", assets.load("images/events/sovereign-debt.png")),
            ("storage-costs", assets.load("images/events/storage-costs.png")),
            ("trade-war", assets.load("images/events/trade-war.png")),
            ("trial", assets.load("images/events/trial.png")),
            ("vaccine", assets.load("images/events/vaccine.png")),
        ]);

        for stock in Company::iter() {
            let name = Box::leak(Box::new(stock.to_lowername())).as_str();
            images.insert(name, assets.load(format!("images/stocks/{}.png", name)));

            let name = Box::leak(Box::new(format!("{}-bond", stock.to_lowername()))).as_str();
            images.insert(name, assets.load(format!("images/bonds/{}.png", stock.to_lowername())));
        }

        for country in CountryName::iter() {
            let name = Box::leak(Box::new(country.to_lowername())).as_str();
            images.insert(name, assets.load(format!("images/bonds/{}.png", name)));

            let name = Box::leak(Box::new(format!("{}-flag", country.to_lowername()))).as_str();
            images.insert(name, assets.load(format!("images/countries/{}.png", name)));
        }

        for currency in CurrencyName::iter() {
            let name = Box::leak(Box::new(currency.to_lowername())).as_str();
            images.insert(name, assets.load(format!("images/forex/{}.png", name)));
        }

        for commodity in CommodityName::iter() {
            let name = Box::leak(Box::new(commodity.to_lowername())).as_str();
            images.insert(name, assets.load(format!("images/commodities/{}.png", name)));
        }

        for crypto in CryptoName::iter() {
            let name = Box::leak(Box::new(crypto.to_lowername())).as_str();
            images.insert(name, assets.load(format!("images/crypto/{}.png", name)));
        }

        Self {
            audio,
            images,
        }
    }
}
