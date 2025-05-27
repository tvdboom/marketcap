use bevy::prelude::Window;
use bevy_egui::egui::{load::SizedTexture, *};

/// Add text widget with custom size
pub fn add_text(text: impl Into<String>, size: f32) -> RichText {
    RichText::new(text).size(size)
}

/// Custom IOS style toggle for UI
pub fn toggle(on: &mut bool) -> impl Widget + '_ {
    move |ui: &mut Ui| {
        let desired_size = ui.spacing().interact_size.y * Vec2::new(2.0, 1.0);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());
        if response.clicked() {
            *on = !*on;
            response.mark_changed();
        }

        response
            .widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, ui.is_enabled(), *on, ""));

        if ui.is_rect_visible(rect) {
            let how_on = ui.ctx().animate_bool_responsive(response.id, *on);
            let visuals = ui.style().interact_selectable(&response, *on);
            let rect = rect.expand(visuals.expansion);
            let radius = 0.5 * rect.height();
            ui.painter().rect(
                rect,
                radius,
                visuals.bg_fill,
                visuals.bg_stroke,
                StrokeKind::Outside,
            );
            let circle_x = lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
            let center = Pos2::new(circle_x, rect.center().y);
            ui.painter()
                .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
        }

        response
    }
}

/// Custom syntactic sugar for repetitive UI elements
pub trait CustomUi {
    fn add_text(&mut self, text: impl Into<String>, size: f32) -> Response;
    fn add_button(&mut self, text: impl Into<String>, window: &Window) -> Response;
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

    fn add_button(&mut self, text: impl Into<String>, window: &Window) -> Response {
        self.add_sized(
            [window.width() * 0.2, window.height() * 0.075],
            Button::new(add_text(text, window.xl_size())),
        )
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

/// Standard text sizes as a fraction of the window size
pub trait TextSizes {
    const XXL_SIZE: f32 = 0.034;
    const XL_SIZE: f32 = 0.024;
    const L_SIZE: f32 = 0.022;
    const M_SIZE: f32 = 0.018;
    const S_SIZE: f32 = 0.016;
    const XS_SIZE: f32 = 0.014;

    fn xxl_size(&self) -> f32;
    fn xl_size(&self) -> f32;
    fn l_size(&self) -> f32;
    fn m_size(&self) -> f32;
    fn s_size(&self) -> f32;
    fn xs_size(&self) -> f32;
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
        self.width().min(self.height()) * Self::S_SIZE
    }

    fn xs_size(&self) -> f32 {
        self.width().min(self.height()) * Self::XS_SIZE
    }
}
