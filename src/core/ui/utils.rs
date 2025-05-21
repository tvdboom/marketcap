use crate::core::assets::WorldAssets;
use crate::core::constants::TOP_LABEL_FRAC;
use crate::core::resources::ImageIds;
use crate::core::ui::themes::{Aesthetics, NordDark};
use bevy::prelude::{Local, ResMut, Window};
use bevy_egui::EguiContexts;
use bevy_egui::egui::epaint::text::{FontInsert, FontPriority, InsertFontFamily};
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Color32, FontData, FontFamily, Image, RichText, TextureId, Ui, WidgetText};

pub fn set_egui_style(mut contexts: EguiContexts) {
    let context = contexts.ctx_mut();

    context.set_style(NordDark.custom_style());

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
    fn add_block(
        &mut self,
        text: impl Into<String>,
        hover_text: impl Into<WidgetText>,
        texture_id: TextureId,
        text_color: impl Into<Color32>,
        window: &Window,
    );
}

impl CustomUi for Ui {
    fn add_block(
        &mut self,
        text: impl Into<String>,
        hover_text: impl Into<WidgetText>,
        texture_id: TextureId,
        color: impl Into<Color32>,
        window: &Window,
    ) {
        let height = window.height() * TOP_LABEL_FRAC;

        self.horizontal_centered(|ui| {
            ui.add(Image::new(SizedTexture::new(texture_id, [height * 0.4; 2])));
            ui.label(RichText::new(text).size(height * 0.4).color(color))
        })
        .response
        .on_hover_text(hover_text);

        self.add_space(window.width() * 0.01);
    }
}
