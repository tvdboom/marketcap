use bevy::prelude::Window;
use bevy_egui::egui::{load::SizedTexture, *};

/// Add text widget with custom size
fn add_text(text: impl Into<String>, size: f32) -> RichText {
    RichText::new(text).size(size)
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
            ui.label(add_text(text, size).color(color))
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
        self.width().min(self.height()) * Self::XXL_SIZE
    }

    fn xl_size(&self) -> f32 {
        self.width().min(self.height()) * Self::XL_SIZE
    }

    fn l_size(&self) -> f32 {
        self.width().min(self.height()) * Self::L_SIZE
    }

    fn m_size(&self) -> f32 {
        self.width().min(self.height()) * Self::M_SIZE
    }

    fn s_size(&self) -> f32 {
        self.width() * Self::S_SIZE
    }
}
