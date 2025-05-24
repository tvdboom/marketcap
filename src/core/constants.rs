use bevy_egui::egui::Color32;

/// General
pub const DEFAULT_SPEED: f32 = 1.;
pub const GAME_SPEED_STEP: f32 = 0.5;
pub const MAX_GAME_SPEED: f32 = 10.;
pub const MESSAGE_DURATION: u64 = 4; // Seconds that messages are shown

/// Colors
pub const GREEN: Color32 = Color32::from_rgb(79, 170, 102);

/// Ui
pub const TOP_LABEL_FRAC: f32 = 0.1; // Fraction of the screen height for the top label
pub const LEFT_LABEL_FRAC: f32 = 0.14; // Fraction of the screen height for the left label
pub const DATE_FORMAT: &str = "%d-%m-%Y";

/// Credit
pub const LOAN_STEP: u32 = 1_000;
