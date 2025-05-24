mod assets;
mod constants;
pub mod factors;
mod game_settings;
mod global_economy;
mod loans;
pub mod messages;
mod pause;
mod player;
mod resources;
mod states;
mod systems;
mod ui;

use crate::core::game_settings::GameSettings;
use crate::core::global_economy::GlobalEconomy;
use crate::core::pause::toggle_pause_keyboard;
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::states::{AppState, AudioState, GameState};
use crate::core::systems::time_pass;
use crate::core::ui::state::UiState;
use crate::core::ui::systems::{
    add_egui_images, central_panel, left_panel, set_egui_style, top_panel,
};
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
            .init_resource::<UiState>()
            .init_resource::<GameSettings>()
            .init_resource::<GlobalEconomy>()
            .init_resource::<Player>()
            // Ui
            .add_systems(Startup, (set_egui_style, add_egui_images))
            .add_systems(Update, (top_panel, left_panel, central_panel).chain())
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
