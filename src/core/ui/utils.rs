use bevy::prelude::{Local, ResMut};
use bevy_egui::egui::epaint::text::{FontInsert, FontPriority, InsertFontFamily};
use bevy_egui::egui::{FontData, FontFamily};
use bevy_egui::EguiContexts;
use crate::core::assets::WorldAssets;
use crate::core::resources::ImageIds;
use crate::core::ui::themes::{Aesthetics, NordDark};

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
