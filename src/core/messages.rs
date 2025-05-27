use crate::core::audio::PlayAudioEv;
use crate::core::constants::MESSAGE_DURATION;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::WidgetText;
use egui_notify::{Anchor, Toast, Toasts};
use std::time::Duration;

pub enum MessageLevel {
    Info,
    Warning,
    Error,
}

#[derive(Event)]
pub struct MessageEv {
    pub message: String,
    pub level: MessageLevel,
}

#[derive(Resource)]
pub struct Messages(pub Toasts);

impl Messages {
    pub fn info(&mut self, message: impl Into<WidgetText>) -> &mut Toast {
        self.0
            .info(message)
            .duration(Some(Duration::from_secs(MESSAGE_DURATION)))
    }

    pub fn warning(&mut self, message: impl Into<WidgetText>) -> &mut Toast {
        self.0
            .warning(message)
            .duration(Some(Duration::from_secs(MESSAGE_DURATION)))
    }

    pub fn error(&mut self, message: impl Into<WidgetText>) -> &mut Toast {
        self.0
            .error(message)
            .duration(Some(Duration::from_secs(MESSAGE_DURATION)))
            .closable(true)
    }
}

fn check_messages(
    contexts: EguiContexts,
    mut messages: ResMut<Messages>,
    mut play_audio_ev: EventWriter<PlayAudioEv>,
    mut message_ev: EventReader<MessageEv>,
) {
    for MessageEv { message, level } in message_ev.read() {
        match level {
            MessageLevel::Info => {
                play_audio_ev.write(PlayAudioEv::new("message"));
                messages.info(message)
            }
            MessageLevel::Warning => {
                play_audio_ev.write(PlayAudioEv::new("warning"));
                messages.warning(message)
            }
            MessageLevel::Error => {
                play_audio_ev.write(PlayAudioEv::new("error"));
                messages.error(message)
            }
        };
    }

    messages.0.show(contexts.ctx());
}

pub struct MessagesPlugin {
    builder: Option<fn() -> Toasts>,
}

impl Default for MessagesPlugin {
    fn default() -> Self {
        Self {
            builder: Some(|| {
                Toasts::default()
                    .with_margin([0., 70.].into())
                    .with_anchor(Anchor::TopRight)
            }),
        }
    }
}

impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Messages(self.builder.map(|f| f()).unwrap_or_default()))
            .add_systems(Update, check_messages);
    }
}
