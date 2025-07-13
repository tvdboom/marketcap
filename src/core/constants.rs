use bevy_egui::egui::Color32;
use chrono::NaiveDate;

use crate::core::instruments::forex::CurrencyName;

/// Window
pub const HEIGHT: f32 = 900.;
pub const WIDTH: f32 = 1600.;

/// General
pub const DEFAULT_SPEED: f32 = 1.;
pub const GAME_SPEED_STEP: f32 = 0.5;
pub const MAX_GAME_SPEED: f32 = 10.;
pub const MESSAGE_DURATION: u64 = 4; // Seconds that messages are shown
pub const VICTORY_AMOUNT: f32 = 1e12;
pub const START_DATE: NaiveDate = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

/// Colors
pub const CUSTOM_GREEN: Color32 = Color32::from_rgb(79, 170, 102);

/// Ui
pub const TOP_LABEL_FRAC: f32 = 0.1; // Fraction of the screen height for the top label
pub const LEFT_LABEL_FRAC: f32 = 0.14; // Fraction of the screen height for the left label
pub const NA: &str = "---";
pub const DATE_FORMAT: &str = "%d-%m-%Y";
pub const CURRENCY: CurrencyName = CurrencyName::EUR; // Default currency for the game
pub const LINE_WIDTH: f32 = 2.5;
pub const LINE_COLOR: Color32 = Color32::LIGHT_BLUE;

/// Credit
pub const LOAN_STEP: u32 = 1_000;
