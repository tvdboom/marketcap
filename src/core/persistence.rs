use crate::core::game_settings::GameSettings;
use crate::core::global_economy::GlobalEconomy;
use crate::core::player::Player;
use crate::core::states::AppState;
use bevy::prelude::*;
use bincode::config::standard;
use bincode::serde::{decode_from_slice, encode_into_slice};
#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::{Read, Write};

#[derive(Event)]
pub struct LoadGameEv;

#[derive(Event)]
pub struct SaveGameEv;

#[derive(Serialize, Deserialize)]
pub struct SaveAll {
    pub game_settings: GameSettings,
    pub global_economy: GlobalEconomy,
    pub player: Player,
}

fn save_to_bin(file_path: &str, data: &SaveAll) -> io::Result<()> {
    let mut buffer = vec![];
    encode_into_slice(data, &mut buffer, standard()).expect("Failed to serialize data.");

    let mut file = File::create(file_path)?;
    file.write_all(&buffer)?;

    Ok(())
}

fn load_from_bin(file_path: &str) -> io::Result<SaveAll> {
    let mut buffer = vec![];

    let mut file = File::open(file_path)?;
    file.read_to_end(&mut buffer)?;

    let (data, _) = decode_from_slice(&buffer, standard()).expect("Failed to deserialize data.");
    Ok(data)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_game(
    mut load_game_ev: EventReader<LoadGameEv>,
    mut game_settings: ResMut<GameSettings>,
    mut global_economy: ResMut<GlobalEconomy>,
    mut player: ResMut<Player>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for _ in load_game_ev.read() {
        if let Some(file_path) = FileDialog::new().pick_file() {
            let file_path_str = file_path.to_string_lossy().to_string();
            let data = load_from_bin(&file_path_str).expect("Failed to load the game.");

            *game_settings = data.game_settings;
            *global_economy = data.global_economy;
            *player = data.player;

            next_app_state.set(AppState::Game);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_game(
    mut save_game_ev: EventReader<SaveGameEv>,
    game_settings: Res<GameSettings>,
    global_economy: Res<GlobalEconomy>,
    player: Res<Player>,
) {
    for _ in save_game_ev.read() {
        if let Some(mut file_path) = FileDialog::new().save_file() {
            if !file_path.extension().map(|e| e == "bin").unwrap_or(false) {
                file_path.set_extension("bin");
            }

            let file_path_str = file_path.to_string_lossy().to_string();
            let data = SaveAll {
                game_settings: game_settings.clone(),
                global_economy: global_economy.clone(),
                player: player.clone(),
            };

            save_to_bin(&file_path_str, &data).expect("Failed to save the game.");
        }
    }
}
