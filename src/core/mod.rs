mod assets;
mod constants;
mod game_settings;
mod player;
mod resources;
mod states;
pub mod ui;

use crate::core::game_settings::GameSettings;
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::states::{AppState, AudioState, GameState};
use crate::core::ui::systems::{top_panel};
use bevy::prelude::*;
use crate::core::ui::utils::{add_egui_images, set_egui_style};

pub struct GamePlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct InGameSet;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .init_state::<AppState>()
            .init_state::<GameState>()
            .init_state::<AudioState>()
            // Resources
            .init_resource::<ImageIds>()
            .init_resource::<GameSettings>()
            .init_resource::<Player>()
            // Ui
            .add_systems(Startup, (set_egui_style, add_egui_images))
            .add_systems(Update, top_panel);
    }
}
