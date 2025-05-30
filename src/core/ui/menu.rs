use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::{Id, Modal, Separator, Slider};
use bevy_kira_audio::AudioControl;
use bevy_kira_audio::prelude::Audio;
use strum::IntoEnumIterator;

use crate::core::constants::{GAME_SPEED_STEP, MAX_GAME_SPEED};
use crate::core::game_settings::{AudioSetting, GameSettings, Theme};
use crate::core::global_economy::GlobalEconomy;
use crate::core::persistence::SaveGameEv;
use crate::core::states::{AppState, GameState};
use crate::core::ui::state::UiState;
use crate::core::ui::utils::{CustomUi, TextSizes, add_text};
use crate::utils::NameFromEnum;

pub fn in_game_menu(
    mut contexts: EguiContexts,
    mut game_settings: ResMut<GameSettings>,
    mut economy: ResMut<GlobalEconomy>,
    game_state: Res<State<GameState>>,
    mut save_game_ev: EventWriter<SaveGameEv>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    audio: Res<Audio>,
    window: Single<&Window>,
) {
    if matches!(
        *game_state.get(),
        GameState::InGameMenu | GameState::Settings
    ) {
        let modal = Modal::new(Id::new("menu")).show(contexts.ctx_mut(), |ui| {
            ui.set_width(window.width() * 0.27);

            ui.vertical_centered(|ui| match *game_state.get() {
                GameState::InGameMenu => {
                    ui.add_space(window.height() * 0.05);

                    if ui.add_button("Continue", &window).clicked() {
                        next_game_state.set(GameState::Running);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if ui.add_button("Save game", &window).clicked() {
                            save_game_ev.write(SaveGameEv);
                        }
                    }

                    if ui.add_button("Settings", &window).clicked() {
                        next_game_state.set(GameState::Settings);
                    }

                    if ui.add_button("Exit", &window).clicked() {
                        next_game_state.set(GameState::Running);
                        next_app_state.set(AppState::MainMenu);
                    }

                    ui.add_space(window.height() * 0.05);
                },
                GameState::Settings => {
                    ui.add_space(window.height() * 0.02);

                    ui.add_text("Settings", window.xxl_size());

                    ui.add(Separator::default().shrink(100.));

                    ui.add_space(window.height() * 0.05);

                    ui.add_text("Theme", window.l_size());

                    ui.horizontal(|ui| {
                        ui.add_space(window.width() * 0.07);

                        for label in Theme::iter() {
                            ui.selectable_value(
                                &mut game_settings.theme,
                                label.clone(),
                                add_text(
                                    format!("{} {}", label.emoji(), label.to_name()),
                                    window.l_size(),
                                ),
                            );
                        }
                    });

                    ui.add_space(window.height() * 0.02);

                    ui.add_text("Game speed", window.l_size());

                    ui.horizontal(|ui| {
                        ui.add_space(window.width() * 0.065);

                        let speed = game_settings.speed;
                        ui.spacing_mut().slider_width = window.width() * 0.1;
                        ui.add(
                            Slider::new(&mut game_settings.speed, GAME_SPEED_STEP..=MAX_GAME_SPEED)
                                .show_value(false)
                                .step_by(GAME_SPEED_STEP as f64)
                                .text(add_text(format!("{:.1}x", speed), window.l_size())),
                        );
                    });

                    ui.add_space(window.height() * 0.02);

                    ui.add_text("Audio", window.l_size());

                    ui.horizontal(|ui| {
                        ui.add_space(window.width() * 0.025);

                        for label in AudioSetting::iter() {
                            ui.selectable_value(
                                &mut game_settings.audio,
                                label.clone(),
                                add_text(
                                    format!("{} {}", label.emoji(), label.to_name()),
                                    window.l_size(),
                                ),
                            );
                        }
                    });

                    ui.add_space(window.height() * 0.08);

                    if ui.add_button("Back", &window).clicked() {
                        next_game_state.set(GameState::InGameMenu);
                    }

                    ui.add_space(window.height() * 0.02);
                },
                _ => {},
            });
        });

        // Adjust settings based on the current choice
        contexts
            .ctx_mut()
            .set_style(game_settings.theme.get().custom_style());

        economy
            .clock
            .set_duration(Duration::from_secs_f32(1. / game_settings.speed));

        if matches!(
            game_settings.audio,
            AudioSetting::Mute | AudioSetting::NoMusic
        ) {
            audio.stop();
        }

        if modal.should_close() {
            next_game_state.set(GameState::Running);
        }
    }
}

pub fn toggle_menu_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<UiState>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match *game_state.get() {
            GameState::Running | GameState::Paused => {
                if ui_state.trade_modal.is_some() {
                    ui_state.trade_modal = None;
                } else {
                    next_game_state.set(GameState::InGameMenu);
                }
            },
            GameState::InGameMenu => next_game_state.set(GameState::Running),
            GameState::Settings => next_game_state.set(GameState::InGameMenu),
        }
    }
}
