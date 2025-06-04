use bevy::prelude::Window;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::*;
use chrono::{Datelike, Duration};
use egui_plot::{AxisHints, GridMark, Line, Plot, PlotPoints};
use strum::IntoEnumIterator;

use crate::core::constants::{CURRENCY, HEIGHT, LINE_COLOR, LINE_WIDTH, WIDTH};
use crate::core::countries::Country;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::Instrument;
use crate::core::instruments::commodities::Commodity;
use crate::core::resources::ImageIds;
use crate::utils::{NameFromEnum, format_number};

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
pub fn line_plot(ui: &mut Ui, data: &Vec<f32>) {
    let start = data.len().saturating_sub(190); // 6 months approx.
    let sin: PlotPoints = data
        .iter()
        .skip(start)
        .enumerate()
        .map(|(i, &v)| [(start + i) as f64, v as f64])
        .collect();

    Plot::new("plot")
        .view_aspect(WIDTH / HEIGHT)
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
        .custom_y_axes(vec![
            AxisHints::new_x().formatter(|mark, _| format_number(mark.value as f32)),
        ])
        .show(ui, |plot_ui| {
            plot_ui.line(Line::new("line", sin).width(LINE_WIDTH).color(LINE_COLOR))
        });
}

/// Custom syntactic sugar for repetitive UI elements
pub trait CustomHover {
    fn on_hover(self, text: impl Into<String>, size: f32) -> Response;
    fn on_disabled_hover(self, text: impl Into<String>, size: f32) -> Response;
}

impl CustomHover for Response {
    fn on_hover(self, text: impl Into<String>, size: f32) -> Response {
        self.on_hover_ui(|ui| {
            ui.add_text(text, size);
        })
    }

    fn on_disabled_hover(self, text: impl Into<String>, size: f32) -> Response {
        self.on_disabled_hover_ui(|ui| {
            ui.add_text(text, size);
        })
    }
}

pub trait CustomUi {
    fn add_text(&mut self, text: impl Into<String>, size: f32) -> Response;
    fn add_button(&mut self, text: impl Into<String>, window: &Window) -> Response;
    fn add_indicator(&mut self, diff: f32, window: &Window) -> Response;
    fn add_factor(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
        color: impl Into<Color32>,
        texture_id: TextureId,
        description: String,
        plot: Option<&Vec<f32>>,
        window: &Window,
    ) -> Response;
    fn add_commodity(
        &mut self,
        commodity: &Commodity,
        images: &ImageIds,
        window: &Window,
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

    fn add_indicator(&mut self, diff: f32, window: &Window) -> Response {
        self.label(
            RichText::new(format!(
                "  {}{diff:.1}%",
                match diff {
                    d if d >= 0. => "▲",
                    _ => "▼",
                }
            ))
            .color(match diff {
                d if d >= 0.05 => Color32::GREEN,
                d if d <= -0.05 => Color32::RED,
                _ => Color32::WHITE,
            })
            .size(window.m_size()),
        )
        .on_hover(
            "Percentage difference between the current price and the average price of the last month.",
            window.m_size(),
        )
    }

    fn add_factor(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
        color: impl Into<Color32>,
        texture_id: TextureId,
        description: String,
        plot: Option<&Vec<f32>>,
        window: &Window,
    ) -> Response {
        self.horizontal_centered(|ui| {
            ui.add(Image::new(SizedTexture::new(
                texture_id,
                [window.xxl_size(); 2],
            )));
            ui.label(add_text(value, window.xxl_size()).color(color))
        })
        .response
        .on_hover_ui(|ui| {
            ui.set_min_width(window.width() * 0.4);

            ui.add_text(name, window.l_size());
            ui.add_space(window.height() * 0.01);
            ui.add_text(description, window.m_size());
            if let Some(values) = plot {
                ui.add_space(window.height() * 0.01);
                line_plot(ui, values);
            }
        })
    }

    fn add_commodity(
        &mut self,
        commodity: &Commodity,
        images: &ImageIds,
        window: &Window,
    ) -> Response {
        Frame::new()
            .stroke(Stroke::new(1.0, Color32::GRAY))
            .corner_radius(5.0)
            .inner_margin(25.0)
            .show(self, |ui| {
                ui.set_width(ui.available_width() * 0.98);

                ui.horizontal(|ui| {
                    ui.add(Image::new(SizedTexture::new(
                        images.get(commodity.name.to_lowername().as_str()),
                        [window.height() * 0.2; 2],
                    )))
                    .on_hover_ui(|ui| {
                        ui.set_min_width(window.width() * 0.4);

                        ui.add_text(commodity.name.to_name(), window.l_size());
                        ui.add_space(window.height() * 0.01);
                        ui.add_text(commodity.description(), window.m_size());
                        ui.add_space(window.height() * 0.01);
                        line_plot(ui, &commodity.prices);
                    });

                    ui.vertical(|ui| {
                        ui.add_text(commodity.name.to_name(), window.l_size());

                        ui.horizontal(|ui| {
                            ui.add_text(
                                format!(
                                    "Price: {:.0} {CURRENCY}/{}",
                                    commodity.current(),
                                    commodity.unit()
                                ),
                                window.m_size(),
                            )
                            .on_hover(
                                format!(
                                    "Current price of the commodity per {}.",
                                    commodity.unit.to_lowername()
                                ),
                                window.m_size(),
                            );

                            ui.add_indicator(commodity.diff(), window);
                        });

                        ui.add_text(
                            format!("Volatility: {:.1}%", commodity.volatility),
                            window.m_size(),
                        )
                        .on_hover(
                            "Maximum daily price fluctuation as percentage of the current price.",
                            window.m_size(),
                        );

                        ui.add_text(
                            format!(
                                "Storage cost: {:.0} {CURRENCY}/{}/month",
                                commodity.storage_cost * 30.,
                                commodity.unit.abbr(),
                            ),
                            window.m_size(),
                        )
                        .on_hover(
                            "Current price of storage per day. Note that this price increases \
                            with inflation. Storage costs are deducted every month or when the \
                            commodity is sold.",
                            window.m_size(),
                        );

                        ui.add_text(
                            format!(
                                "Production: {}",
                                Country::iter()
                                    .filter_map(|c| c
                                        .production()
                                        .contains(&commodity.name)
                                        .then_some(c.to_name()))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            ),
                            window.m_size(),
                        )
                        .on_hover(
                            "Countries producing this commodity. Bond prices for \
                            these countries might be affected by the commodity price.",
                            window.m_size(),
                        );
                    });
                })
            })
            .inner
            .response
            .interact(Sense::hover())
            .interact(Sense::click())
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
        self.width().min(self.height()).min(1.2 * HEIGHT) * Self::XXL_SIZE
    }

    fn xl_size(&self) -> f32 {
        self.width().min(self.height()).min(1.2 * HEIGHT) * Self::XL_SIZE
    }

    fn l_size(&self) -> f32 {
        self.width().min(self.height()).min(1.2 * HEIGHT) * Self::L_SIZE
    }

    fn m_size(&self) -> f32 {
        self.width().min(self.height()).min(1.2 * HEIGHT) * Self::M_SIZE
    }

    fn s_size(&self) -> f32 {
        self.width().min(self.height()).min(1.2 * HEIGHT) * Self::S_SIZE
    }

    fn xs_size(&self) -> f32 {
        self.width().min(self.height()).min(1.2 * HEIGHT) * Self::XS_SIZE
    }
}
