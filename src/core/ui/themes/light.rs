use bevy_egui::egui::{Color32, Vec2};

use crate::core::ui::themes::aesthetics::Aesthetics;

pub struct NordLight;

impl Aesthetics for NordLight {
    fn name(&self) -> &'static str {
        "Nord Light"
    }

    fn primary_accent_color_visuals(&self) -> Color32 {
        Color32::from_rgb(104, 161, 210)
    }

    fn bg_primary_color_visuals(&self) -> Color32 {
        Color32::from_rgb(216, 222, 233)
    }

    fn bg_secondary_color_visuals(&self) -> Color32 {
        Color32::from_rgb(229, 233, 240)
    }

    fn bg_triage_color_visuals(&self) -> Color32 {
        Color32::from_rgb(255, 255, 255)
    }

    fn bg_auxiliary_color_visuals(&self) -> Color32 {
        Color32::from_rgb(206, 212, 224)
    }

    fn bg_contrast_color_visuals(&self) -> Color32 {
        Color32::from_rgb(180, 186, 189)
    }

    fn fg_primary_text_color_visuals(&self) -> Option<Color32> {
        Some(Color32::BLACK)
    }

    fn fg_warn_text_color_visuals(&self) -> Color32 {
        Color32::from_rgb(255, 179, 71)
    }

    fn fg_error_text_color_visuals(&self) -> Color32 {
        Color32::from_rgb(228, 97, 107)
    }

    fn dark_mode_visuals(&self) -> bool {
        false
    }

    fn margin_style(&self) -> i8 {
        12
    }

    fn button_padding(&self) -> Vec2 {
        Vec2 {
            x: 12.0,
            y: 10.0,
        }
    }

    fn item_spacing_style(&self) -> f32 {
        18.0
    }

    fn rounding_visuals(&self) -> u8 {
        6
    }
}
