use crate::core::assets::WorldAssets;
use crate::core::game_settings::GameSettings;
use crate::core::resources::ImageIds;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::epaint::text::{FontInsert, FontPriority, InsertFontFamily};
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{
    Color32, FontData, FontFamily, Image, Response, RichText, TextureId, Ui, WidgetText,
};

pub fn set_egui_style(mut contexts: EguiContexts, game_settings: Res<GameSettings>) {
    let context = contexts.ctx_mut();

    context.set_style(game_settings.theme.get().custom_style());

    context.add_font(FontInsert::new(
        "firamono",
        FontData::from_static(include_bytes!("../../../assets/fonts/FiraMono-Medium.ttf")),
        vec![
            InsertFontFamily {
                family: FontFamily::Proportional,
                priority: FontPriority::Highest,
            },
            InsertFontFamily {
                family: FontFamily::Monospace,
                priority: FontPriority::Lowest,
            },
        ],
    ));

    context.add_font(FontInsert::new(
        "firasans",
        FontData::from_static(include_bytes!("../../../assets/fonts/FiraSans-Bold.ttf")),
        vec![
            InsertFontFamily {
                family: FontFamily::Proportional,
                priority: FontPriority::Highest,
            },
            InsertFontFamily {
                family: FontFamily::Monospace,
                priority: FontPriority::Lowest,
            },
        ],
    ));
}

pub fn add_egui_images(
    mut contexts: EguiContexts,
    mut images: ResMut<ImageIds>,
    assets: Local<WorldAssets>,
) {
    for (k, v) in assets.images.iter() {
        let id = contexts.add_image(v.clone_weak());
        images.0.insert(k, id);
    }
}

/// Custom syntactic sugar for repetitive UI elements
pub trait CustomUi {
    fn add_text(&mut self, text: impl Into<String>, size: f32) -> Response;

    fn add_block(
        &mut self,
        text: impl Into<String>,
        hover_text: impl Into<WidgetText>,
        texture_id: TextureId,
        text_color: impl Into<Color32>,
        size: f32,
    );
}

impl CustomUi for Ui {
    fn add_text(&mut self, text: impl Into<String>, size: f32) -> Response {
        self.label(RichText::new(text).size(size))
    }

    fn add_block(
        &mut self,
        text: impl Into<String>,
        hover_text: impl Into<WidgetText>,
        texture_id: TextureId,
        color: impl Into<Color32>,
        size: f32,
    ) {
        self.horizontal_centered(|ui| {
            ui.add(Image::new(SizedTexture::new(texture_id, [size; 2])));
            ui.label(RichText::new(text).size(size).color(color))
        })
        .response
        .on_hover_text(hover_text);
    }
}

/// Standard text sizes as a fraction of the window width
pub trait TextSizes {
    const XXL_SIZE: f32 = 0.022;
    const XL_SIZE: f32 = 0.012;
    const L_SIZE: f32 = 0.011;
    const M_SIZE: f32 = 0.009;
    const S_SIZE: f32 = 0.008;

    fn xxl_size(&self) -> f32;
    fn xl_size(&self) -> f32;
    fn l_size(&self) -> f32;
    fn m_size(&self) -> f32;
    fn s_size(&self) -> f32;
}

impl TextSizes for Window {
    fn xxl_size(&self) -> f32 {
        self.width() * Self::XXL_SIZE
    }

    fn xl_size(&self) -> f32 {
        self.width() * Self::XL_SIZE
    }

    fn l_size(&self) -> f32 {
        self.width() * Self::L_SIZE
    }

    fn m_size(&self) -> f32 {
        self.width() * Self::M_SIZE
    }

    fn s_size(&self) -> f32 {
        self.width() * Self::S_SIZE
    }
}
