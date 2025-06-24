use std::collections::HashMap;

use bevy::asset::{AssetServer, Handle};
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use strum::IntoEnumIterator;

use crate::core::countries::CountryName;
use crate::core::instruments::commodities::CommodityName;
use crate::core::instruments::crypto::CryptoName;
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
        map.get(name)
            .expect(&format!("No asset for {asset_type} {name}"))
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
            // Icons
            ("cash", assets.load("images/icons/cash.png")),
            ("credit-score", assets.load("images/icons/credit-score.png")),
            ("economic", assets.load("images/icons/economic.png")),
            ("enterprise", assets.load("images/icons/enterprise.png")),
            ("inflation", assets.load("images/icons/inflation.png")),
            ("influence", assets.load("images/icons/influence.png")),
            ("interest", assets.load("images/icons/interest.png")),
            ("logo", assets.load("images/icons/logo.png")),
            ("netflow", assets.load("images/icons/netflow.png")),
            ("time", assets.load("images/icons/time.png")),
            ("time-paused", assets.load("images/icons/time-paused.png")),
        ]);

        for stock in Company::iter() {
            let name = Box::leak(Box::new(stock.to_lowername())).as_str();
            images.insert(name, assets.load(format!("images/stocks/{}.png", name)));

            let name = Box::leak(Box::new(format!("{}-bond", stock.to_lowername()))).as_str();
            images.insert(
                name,
                assets.load(format!("images/bonds/{}.png", stock.to_lowername())),
            );
        }

        for country in CountryName::iter() {
            let name = Box::leak(Box::new(country.to_lowername())).as_str();
            images.insert(name, assets.load(format!("images/bonds/{}.png", name)));
        }

        for commodity in CommodityName::iter() {
            let name = Box::leak(Box::new(commodity.to_lowername())).as_str();
            images.insert(
                name,
                assets.load(format!("images/commodities/{}.png", name)),
            );
        }

        for crypto in CryptoName::iter() {
            let name = Box::leak(Box::new(crypto.to_lowername())).as_str();
            images.insert(name, assets.load(format!("images/crypto/{}.png", name)));
        }

        Self { audio, images }
    }
}
