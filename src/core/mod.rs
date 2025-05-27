mod assets;
mod audio;
mod constants;
pub mod factors;
mod game_settings;
mod global_economy;
mod loans;
pub mod messages;
mod pause;
mod persistence;
mod player;
mod resources;
mod states;
mod systems;
mod ui;

use crate::core::audio::{PlayAudioEv, play_audio_event, toggle_music_keyboard};
use crate::core::game_settings::GameSettings;
use crate::core::global_economy::GlobalEconomy;
use crate::core::pause::toggle_pause_keyboard;
use crate::core::persistence::{LoadGameEv, SaveGameEv, load_game, save_game};
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::states::{AppState, GameState};
use crate::core::systems::time_pass;
use crate::core::ui::menu::{in_game_menu, toggle_menu_keyboard};
use crate::core::ui::state::UiState;
use crate::core::ui::systems::{
    add_egui_images, central_panel, check_keys, left_panel, set_egui_style, top_panel,
};
use bevy::prelude::*;

pub struct GamePlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct InGameSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct InRunningGameSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct InRunningOrPausedGameSet;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .init_state::<AppState>()
            .init_state::<GameState>()
            // Resources
            .init_resource::<ImageIds>()
            .init_resource::<UiState>()
            .init_resource::<GameSettings>()
            .init_resource::<GlobalEconomy>()
            .init_resource::<Player>()
            // Events
            .add_event::<LoadGameEv>()
            .add_event::<SaveGameEv>()
            .add_event::<PlayAudioEv>()
            // Sets
            .configure_sets(Update, InGameSet.run_if(in_state(AppState::Game)))
            .configure_sets(
                Update,
                InRunningGameSet
                    .run_if(in_state(GameState::Running))
                    .in_set(InGameSet),
            )
            .configure_sets(
                Update,
                InRunningOrPausedGameSet
                    .run_if(in_state(GameState::Running).or(in_state(GameState::Paused)))
                    .in_set(InGameSet),
            )
            // Audio
            .add_systems(Update, (toggle_music_keyboard, play_audio_event))
            // Persistence
            .add_systems(Update, (load_game, save_game))
            // Ui
            .add_systems(Startup, (set_egui_style, add_egui_images))
            .add_systems(
                Update,
                (top_panel, left_panel, central_panel, in_game_menu)
                    .chain()
                    .in_set(InGameSet),
            )
            // Systems
            .add_systems(
                Update,
                (
                    time_pass.in_set(InRunningGameSet),
                    toggle_pause_keyboard.in_set(InRunningOrPausedGameSet),
                    toggle_menu_keyboard.in_set(InGameSet),
                    check_keys.in_set(InRunningGameSet),
                ),
            );
    }
}
