use bevy::prelude::Window;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::*;
use chrono::{Datelike, Duration};
use egui_plot::{AxisHints, GridMark, Line, Plot, PlotPoints};

use crate::core::constants::LINE_WIDTH;
use crate::core::global_economy::GlobalEconomy;

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

/// Make a line plot with the last 6 months of data
pub fn line_plot(ui: &mut Ui, data: &Vec<f32>, color: Color32) {
    let start = data.len().saturating_sub(190); // 6 months approx.
    let sin: PlotPoints = data
        .iter()
        .skip(start)
        .enumerate()
        .map(|(i, &v)| [(start + i) as f64, v as f64])
        .collect();

    Plot::new("plot")
        .show_background(false)
        .x_grid_spacer(|grid| {
            (grid.bounds.0 as i64..grid.bounds.1 as i64)
                .map(|x| {
                    let d = GlobalEconomy::default().date + Duration::days(x);
                    GridMark {
                        value: x as f64,
                        step_size: if d.day() == 1 { 30. } else { 0. },
                    }
                })
                .collect()
        })
        .custom_x_axes(vec![AxisHints::new_x().formatter(|mark, _| {
            let d = GlobalEconomy::default().date + Duration::days(mark.value as i64);
            format!("{:02}-{}", d.month(), d.year())
        })])
        .show(ui, |plot_ui| {
            plot_ui.line(Line::new("line", sin).width(LINE_WIDTH).color(color))
        });
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
    ) -> Response;
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
    ) -> Response {
        self.horizontal_centered(|ui| {
            ui.add(Image::new(SizedTexture::new(texture_id, [size; 2])));
            ui.label(add_text(text, size).color(color))
        })
        .response
        .on_hover_text(hover_text)
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
