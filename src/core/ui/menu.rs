use crate::core::states::GameState;
use crate::core::ui::utils::CustomUi;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::{Id, Modal};

pub fn in_game_menu(
    mut contexts: EguiContexts,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    window: Single<&Window>,
) {
    match *game_state.get() {
        GameState::InGameMenu => {
            let modal = Modal::new(Id::new("in-game-menu")).show(contexts.ctx_mut(), |ui| {
                ui.set_width(window.width() * 0.35);
                ui.set_height(window.height() * 0.6);

                ui.vertical_centered(|ui| {
                    ui.add_space(window.height() * 0.05);

                    if ui.add_button("Continue", &window).clicked() {
                        next_game_state.set(GameState::Running);
                    }

                    if ui.add_button("Save game", &window).clicked() {
                        next_game_state.set(GameState::Running);
                    }

                    if ui.add_button("Settings", &window).clicked() {
                        next_game_state.set(GameState::Settings);
                    }

                    if ui.add_button("Exit", &window).clicked() {
                        next_game_state.set(GameState::Running);
                    }

                    ui.add_space(window.height() * 0.04);
                });
            });

            if modal.should_close() {
                next_game_state.set(GameState::Running);
            }
        }
        _ => {}
    }
}

pub fn toggle_menu_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_game_state.set(match *game_state.get() {
            GameState::Running | GameState::Paused => GameState::InGameMenu,
            GameState::InGameMenu => GameState::Running,
            GameState::Settings => GameState::InGameMenu,
        });
    }
}
