use bevy::asset::{AssetServer, Handle};
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use std::collections::HashMap;

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

    pub fn image(&self, name: &str) -> &Handle<Image> {
        self.get_asset(&self.images, name, "image")
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

        let images: HashMap<&'static str, Handle<Image>> = HashMap::from([
            // Icons
            ("cash", assets.load("images/icons/cash.png")),
            ("credit-score", assets.load("images/icons/credit-score.png")),
            ("economic", assets.load("images/icons/economic.png")),
            ("enterprise", assets.load("images/icons/enterprise.png")),
            ("global", assets.load("images/icons/global.png")),
            ("inflation", assets.load("images/icons/inflation.png")),
            ("interest", assets.load("images/icons/interest.png")),
            ("logo", assets.load("images/icons/logo.png")),
            ("netflow", assets.load("images/icons/netflow.png")),
            ("time", assets.load("images/icons/time.png")),
            ("time-paused", assets.load("images/icons/time-paused.png")),
        ]);

        Self { audio, images }
    }
}
