use std::collections::HashMap;

use bevy::prelude::Resource;
use bevy_egui::egui::TextureId;
use itertools::Itertools;

use crate::core::instruments::instrument::InstrumentKind;

#[derive(Resource, Default)]
pub struct ImageIds(pub HashMap<&'static str, TextureId>);

impl ImageIds {
    pub fn get(&self, key: &str) -> TextureId {
        *self
            .0
            .get(key)
            .expect(format!("No image found with name: {}", key).as_str())
    }
}

#[derive(Resource, Default)]
pub struct Favourites(pub HashMap<u8, InstrumentKind>);

impl Favourites {
    pub fn contains(&self, instrument: &InstrumentKind) -> bool {
        self.0.values().contains(&instrument)
    }
}
