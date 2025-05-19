mod assets;
mod constants;
mod game_params;
mod game_settings;
mod pause;
mod player;
mod resources;
mod states;
mod systems;
pub mod ui;

use crate::core::game_params::GameParams;
use crate::core::game_settings::GameSettings;
use crate::core::pause::toggle_pause_keyboard;
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::states::{AppState, AudioState, GameState};
use crate::core::systems::time_pass;
use crate::core::ui::systems::{left_panel, top_panel};
use crate::core::ui::utils::{add_egui_images, set_egui_style};
use bevy::prelude::*;

pub struct GamePlugin;

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
            .init_resource::<GameParams>()
            .init_resource::<Player>()
            // Ui
            .add_systems(Startup, (set_egui_style, add_egui_images))
            .add_systems(Update, (left_panel, top_panel))
            // Systems
            .add_systems(
                Update,
                (
                    time_pass.run_if(in_state(GameState::Running)),
                    toggle_pause_keyboard,
                ),
            );
    }
}
