use std::collections::HashMap;

use bevy::prelude::Resource;
use bevy_egui::egui::TextureId;

use crate::core::player::InstrumentKind;

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
pub struct KeyMap(pub HashMap<u8, InstrumentKind>);
