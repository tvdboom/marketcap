use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::{
    Align, CentralPanel, Frame, Id, Layout, Modal, RichText, Separator, Slider, Ui, UiBuilder,
};
use bevy_kira_audio::AudioControl;
use bevy_kira_audio::prelude::Audio;
use strum::IntoEnumIterator;

use crate::TITLE;
use crate::core::constants::{GAME_SPEED_STEP, HEIGHT, MAX_GAME_SPEED, WIDTH};
use crate::core::game_settings::{AudioSetting, GameSettings, Theme};
use crate::core::global_economy::GlobalEconomy;
use crate::core::persistence::{LoadGameEv, SaveGameEv};
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::states::{AppState, GameState};
use crate::core::ui::state::UiState;
use crate::core::ui::utils::CustomUi;
use crate::utils::NameFromEnum;

fn load_settings(ui: &mut Ui, game_settings: &mut GameSettings, window: &Window) {
    ui.heading("Theme");

    ui.horizontal(|ui| {
        ui.add_space(ui.available_width() * 0.5 - 100.);

        for label in Theme::iter() {
            ui.selectable_value(
                &mut game_settings.theme,
                label,
                format!("{} {}", label.emoji(), label.to_name()),
            );
        }
    });

    ui.add_space(window.height() * 0.02);

    ui.heading("Game speed");

    ui.horizontal(|ui| {
        ui.add_space(ui.available_width() * 0.5 - 100.);

        let speed = game_settings.speed;
        ui.spacing_mut().slider_width = (window.width() * 0.1).min(250.);
        ui.add(
            Slider::new(&mut game_settings.speed, GAME_SPEED_STEP..=MAX_GAME_SPEED)
                .show_value(false)
                .step_by(GAME_SPEED_STEP as f64)
                .text(format!("{speed:.1}x")),
        );
    });

    ui.add_space(window.height() * 0.02);

    ui.heading("Audio");

    ui.horizontal(|ui| {
        ui.add_space(ui.available_width() * 0.5 - 180.);
        for label in AudioSetting::iter() {
            ui.selectable_value(
                &mut game_settings.audio,
                label,
                format!("{} {}", label.emoji(), label.to_name()),
            );
        }
    });

    ui.add_space(window.height() * 0.08);
}

fn update_settings(
    contexts: &mut EguiContexts,
    game_settings: &mut GameSettings,
    economy: &mut GlobalEconomy,
    audio: &Audio,
    window: &Window,
) {
    // Adjust settings based on the current choice
    contexts
        .ctx_mut()
        .set_style(game_settings.theme.get().custom_style(window.width(), window.height()));

    economy.clock.set_duration(Duration::from_secs_f32(1. / game_settings.speed));

    if matches!(game_settings.audio, AudioSetting::Mute | AudioSetting::NoMusic) {
        audio.stop();
    }
}

pub fn main_menu(
    mut contexts: EguiContexts,
    mut game_settings: ResMut<GameSettings>,
    mut economy: ResMut<GlobalEconomy>,
    mut load_game_ev: EventWriter<LoadGameEv>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    images: Res<ImageIds>,
    audio: Res<Audio>,
    window: Single<&Window>,
) {
    CentralPanel::default().frame(Frame::default().inner_margin(0.)).show(
        contexts.ctx_mut(),
        |ui| {
            let response = ui.add_image(images.get("cover"), ui.available_size());

            ui.allocate_new_ui(UiBuilder::new().max_rect(response.rect), |ui| {
                ui.add_space(window.height() * 0.02);
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new(TITLE).size(window.width() * 0.1));

                    ui.add_space(window.height() * 0.1);

                    ui.vertical_centered(|ui| match *app_state.get() {
                        AppState::MainMenu => {
                            if ui.add_button(RichText::new("New game").heading(), &window).clicked()
                            {
                                next_app_state.set(AppState::Game);
                            }

                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                if ui
                                    .add_button(RichText::new("Load game").heading(), &window)
                                    .clicked()
                                {
                                    load_game_ev.write(LoadGameEv);
                                }
                            }

                            if ui.add_button(RichText::new("Settings").heading(), &window).clicked()
                            {
                                next_app_state.set(AppState::Settings);
                            }

                            if ui.add_button(RichText::new("Exit").heading(), &window).clicked() {
                                std::process::exit(0);
                            }
                        },
                        AppState::Settings => {
                            load_settings(ui, &mut game_settings, &window);

                            if ui.add_button(RichText::new("Back").heading(), &window).clicked() {
                                next_app_state.set(AppState::MainMenu);
                            }
                        },
                        _ => {},
                    });
                });
            });
        },
    );

    update_settings(&mut contexts, &mut game_settings, &mut economy, &audio, &window);
}

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
    if matches!(*game_state.get(), GameState::InGameMenu | GameState::Settings) {
        let modal = Modal::new(Id::new("menu")).show(contexts.ctx_mut(), |ui| {
            ui.set_width((window.width() * 0.25).min(450.));

            ui.add_space(window.height() * 0.02);

            ui.vertical_centered(|ui| match *game_state.get() {
                GameState::InGameMenu => {
                    if ui.add_button(RichText::new("Continue").heading(), &window).clicked() {
                        next_game_state.set(GameState::Running);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if ui.add_button(RichText::new("Save game").heading(), &window).clicked() {
                            save_game_ev.write(SaveGameEv);
                        }
                    }

                    if ui.add_button(RichText::new("Settings").heading(), &window).clicked() {
                        next_game_state.set(GameState::Settings);
                    }

                    if ui.add_button(RichText::new("Exit").heading(), &window).clicked() {
                        next_game_state.set(GameState::Running);
                        next_app_state.set(AppState::MainMenu);
                    }
                },
                GameState::Settings => {
                    ui.heading("Settings");
                    ui.add(Separator::default().shrink(50.));
                    ui.add_space(window.height() * 0.05);

                    load_settings(ui, &mut game_settings, &window);

                    if ui.add_button(RichText::new("Back").heading(), &window).clicked() {
                        next_game_state.set(GameState::InGameMenu);
                    }
                },
                _ => {},
            });

            ui.add_space(window.height() * 0.01);
        });

        if modal.should_close() {
            next_game_state.set(GameState::Running);
        }

        update_settings(&mut contexts, &mut game_settings, &mut economy, &audio, &window);
    }
}

pub fn end_game_menu(
    mut commands: Commands,
    mut contexts: EguiContexts,
    economy: Res<GlobalEconomy>,
    mut player: ResMut<Player>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    images: Res<ImageIds>,
    window: Single<&Window>,
) {
    let defeat = player.aum(&economy) <= 0.;
    Modal::new(Id::new("end_game")).frame(Frame::default().inner_margin(0.)).show(
        contexts.ctx_mut(),
        |ui| {
            ui.set_width((window.width() * 0.75).max(WIDTH * 0.75));
            ui.set_height((window.height() * 0.7).max(HEIGHT * 0.7));

            let response = ui.add_image(
                images.get(if defeat {
                    "game-over"
                } else {
                    "victory"
                }),
                ui.available_size(),
            );

            ui.allocate_new_ui(UiBuilder::new().max_rect(response.rect), |ui| {
                ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                    ui.add_space(window.height() * 0.05);

                    ui.horizontal(|ui| {
                        if !defeat {
                            ui.add_space(window.width() * 0.02);

                            if ui.add_modal_button("Continue", &window).clicked() {
                                player.has_continued = true;
                                next_game_state.set(GameState::Running);
                            }
                        }

                        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                            ui.horizontal(|ui| {
                                ui.add_space(window.width() * 0.02);

                                if ui.add_modal_button("Exit to main menu", &window).clicked() {
                                    next_game_state.set(GameState::Running);
                                    next_app_state.set(AppState::MainMenu);
                                }

                                if ui.add_modal_button("New game", &window).clicked() {
                                    commands.insert_resource(GlobalEconomy::default());
                                    commands.insert_resource(Player::default());
                                    commands.insert_resource(UiState::default());
                                    next_game_state.set(GameState::Running);
                                }
                            });
                        });
                    });
                });
            });
        },
    );
}

pub fn main_menu_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter) {
        match app_state.get() {
            AppState::Settings => next_app_state.set(AppState::MainMenu),
            _ => (),
        }
    }
}

pub fn in_game_menu_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<UiState>,
    game_state: Res<State<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match *game_state.get() {
            GameState::StartGame => next_game_state.set(GameState::Running),
            GameState::Running | GameState::Paused => {
                if state.modal.is_some() {
                    state.modal = None;
                } else if state.active_event.is_some() {
                    next_game_state.set(GameState::Running);
                    state.active_event = None;
                } else {
                    next_game_state.set(GameState::InGameMenu);
                }
            },
            GameState::InGameMenu => next_game_state.set(GameState::Running),
            GameState::Settings => next_game_state.set(GameState::InGameMenu),
            GameState::GameEnd => {
                next_game_state.set(GameState::Running);
                next_app_state.set(AppState::MainMenu);
            },
        }
    }

    if keyboard.just_pressed(KeyCode::Enter)
        && (*game_state.get() == GameState::StartGame || state.active_event.is_some())
    {
        next_game_state.set(GameState::Running);
        state.active_event = None;
    }
}
