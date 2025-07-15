use crate::core::constants::{HEIGHT, WIDTH};
use crate::core::global_economy::GlobalEconomy;
use crate::core::resources::ImageIds;
use crate::core::states::GameState;
use crate::core::ui::state::UiState;
use crate::core::ui::utils::CustomUi;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Align, Frame, Id, Image, Layout, Modal, UiBuilder};

pub fn event_modal(
    mut contexts: EguiContexts,
    mut state: ResMut<UiState>,
    economy: ResMut<GlobalEconomy>,
    mut next_game_state: ResMut<NextState<GameState>>,
    images: Res<ImageIds>,
    window: Single<&Window>,
) {
    let event = if let Some(name) = &state.active_event {
        economy.events.iter().find(|e| e.name == *name).unwrap()
    } else {
        return;
    };

    let modal = Modal::new(Id::new("event_modal"))
        .frame(Frame::default().inner_margin(0.))
        .show(contexts.ctx_mut(), |ui| {
            ui.set_width((window.width() * 0.55).max(WIDTH * 0.55));
            ui.set_height((window.height() * 0.5).max(HEIGHT * 0.5));

            let response = ui.add(Image::new(SizedTexture::new(
                images.get(event.image().as_str()),
                ui.available_size(),
            )));

            ui.allocate_new_ui(UiBuilder::new().max_rect(response.rect), |ui| {
                ui.add_space(window.height() * 0.02);
                ui.vertical_centered(|ui| {
                    ui.heading(event.title());
                });

                ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                    ui.add_space(window.height() * 0.05);

                    ui.horizontal(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                            ui.add_space(window.width() * 0.02);

                            if ui.add_modal_button("Continue", &window).clicked() {
                                state.active_event = None;
                                next_game_state.set(GameState::Running);
                            }
                        });
                    });
                });
            });
        });

    if modal.should_close() {
        state.active_event = None;
        next_game_state.set(GameState::Running);
    }
}
