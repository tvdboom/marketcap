use bevy::prelude::Window;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::*;
use chrono::{Datelike, Duration};
use egui_plot::{AxisHints, GridMark, Line, Plot, PlotPoints};
use itertools::Itertools;

use crate::core::constants::{CURRENCY, HEIGHT, LINE_COLOR, LINE_WIDTH, WIDTH};
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::bonds::BondIssuer;
use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::core::orders::{Command, Order};
use crate::core::player::Player;
use crate::core::resources::ImageIds;
use crate::core::ui::state::{OrderByState, OrderOptions};
use crate::utils::{DQueue, EnhFloat, NameFromEnum, create_guid, get_ratio};

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

pub trait CustomUi {
    fn add_button(&mut self, text: impl Into<WidgetText>, window: &Window) -> Response;
    fn add_modal_button(&mut self, text: impl Into<WidgetText>, window: &Window) -> Response;
    fn add_indicator(&mut self, diff: f32) -> Response;
    fn add_combobox(
        &mut self,
        title: &str,
        options: Vec<OrderOptions>,
        state: &mut OrderByState,
        window: &Window,
    );
    fn add_plot(&mut self, data: &DQueue<f32>, orders: Option<Vec<&Order>>);
    fn add_factor(
        &mut self,
        name: impl Into<RichText>,
        value: impl Into<WidgetText>,
        color: impl Into<Color32>,
        texture_id: TextureId,
        description: String,
        plot: Option<&DQueue<f32>>,
        window: &Window,
    ) -> Response;
    fn add_instrument(
        &mut self,
        instrument: &dyn Instrument,
        economy: &GlobalEconomy,
        player: &Player,
        images: &ImageIds,
        window: &Window,
    ) -> Response;
}

impl CustomUi for Ui {
    fn add_button(&mut self, text: impl Into<WidgetText>, window: &Window) -> Response {
        self.add_sized(
            [
                (window.width() * 0.2).min(300.),
                (window.height() * 0.075).min(70.),
            ],
            Button::new(text),
        )
    }

    fn add_modal_button(&mut self, text: impl Into<WidgetText>, window: &Window) -> Response {
        self.add_sized(
            [window.width() * 0.08, window.height() * 0.05],
            Button::new(text),
        )
    }

    fn add_indicator(&mut self, diff: f32) -> Response {
        self.label(
            RichText::new(format!(
                "  {}{diff:.1}%",
                match diff {
                    d if d > 0. => "▲",
                    d if d < 0. => "▼",
                    _ => "→",
                }
            ))
                .color(match diff {
                    d if d >= 0.05 => Color32::GREEN,
                    d if d <= -0.05 => Color32::RED,
                    _ => Color32::WHITE,
                }),
        )
            .on_hover_text(
                "Percentage difference between the current price and the average price of the last month.",
            )
    }

    fn add_combobox(
        &mut self,
        title: &str,
        options: Vec<OrderOptions>,
        state: &mut OrderByState,
        window: &Window,
    ) {
        Sides::new().show(
            self,
            |ui| ui.heading(title),
            |ui| {
                ui.add_space(window.width() * 0.02);

                ComboBox::from_id_salt(title)
                    .selected_text(state.order.to_name())
                    .show_ui(ui, |ui| {
                        for order in options {
                            ui.selectable_value(&mut state.order, order, order.to_name());
                        }
                    });

                let descending = ui
                    .label(if state.descending {
                        "▼ Descending"
                    } else {
                        "▲ Ascending"
                    })
                    .interact(Sense::click());

                if descending.clicked() {
                    state.descending = !state.descending;
                }
            },
        );
    }

    fn add_plot(&mut self, data: &DQueue<f32>, orders: Option<Vec<&Order>>) {
        let date = GlobalEconomy::default().date;

        let start = data.len().saturating_sub(190); // 6 months approx.
        let points: PlotPoints = data
            .iter()
            .skip(start)
            .enumerate()
            .map(|(i, &v)| [(start + i) as f64, v as f64])
            .collect();

        Plot::new(create_guid())
            .sense(Sense::empty()) // Disable dragging
            .view_aspect(WIDTH / HEIGHT)
            .show_background(false)
            .x_grid_spacer(|grid| {
                (grid.bounds.0 as i64..grid.bounds.1 as i64)
                    .map(|x| {
                        let d = date + Duration::days(x);
                        GridMark {
                            value: x as f64,
                            step_size: if d.day() == 1 { 30. } else { 0. },
                        }
                    })
                    .collect()
            })
            .custom_x_axes(vec![AxisHints::new_x().formatter(|mark, _| {
                let d = date + Duration::days(mark.value as i64);
                format!("{:02}-{}", d.month(), d.year())
            })])
            .custom_y_axes(vec![
                AxisHints::new_x().formatter(|mark, _| (mark.value as f32).format()),
            ])
            .show(self, |plot_ui| {
                plot_ui.line(
                    Line::new("price", points)
                        .width(LINE_WIDTH)
                        .color(LINE_COLOR),
                );

                if let Some(orders) = orders {
                    for order in orders {
                        let points: PlotPoints = data
                            .iter()
                            .skip(start)
                            .enumerate()
                            .map(|(i, _)| {
                                [
                                    (start + i) as f64,
                                    if order.created <= date + Duration::days((start + i) as i64) {
                                        order.limit_price() as f64
                                    } else {
                                        f64::NAN
                                    },
                                ]
                            })
                            .collect();

                        plot_ui.line(
                            Line::new(create_guid(), points)
                                .width(LINE_WIDTH - 1.)
                                .color(match order.command {
                                    Command::Buy => Color32::LIGHT_GREEN,
                                    Command::Sell => Color32::LIGHT_RED,
                                    Command::Close => Color32::RED,
                                }),
                        );
                    }
                }
            });
    }

    fn add_factor(
        &mut self,
        name: impl Into<RichText>,
        value: impl Into<WidgetText>,
        color: impl Into<Color32>,
        texture_id: TextureId,
        description: String,
        plot: Option<&DQueue<f32>>,
        window: &Window,
    ) -> Response {
        self.horizontal_centered(|ui| {
            ui.add(Image::new(SizedTexture::new(
                texture_id,
                [get_ratio(window.width(), window.height(), TextStyle::Heading); 2],
            )));
            ui.label(value.into().heading().color(color))
        })
        .response
        .on_hover_ui(|ui| {
            ui.set_min_width(window.width() * 0.4);

            ui.label(name.into());
            ui.add_space(window.height() * 0.01);
            ui.label(description);
            if let Some(values) = plot {
                ui.add_space(window.height() * 0.01);
                ui.add_plot(values, None);
            }
        })
    }

    fn add_instrument(
        &mut self,
        instrument: &dyn Instrument,
        economy: &GlobalEconomy,
        player: &Player,
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
                    ui.vertical(|ui| {
                        if matches!(instrument.kind(), InstrumentKind::Forex(_)) {
                            ui.heading(format!("{}/{}", instrument.name(), CURRENCY.to_name()));
                        } else {
                            ui.heading(instrument.name());
                        }

                        ui.add(Image::new(SizedTexture::new(
                            images.get(instrument.image().as_str()),
                            [window.height() * 0.2; 2],
                        )));
                    });

                    ui.vertical(|ui| {
                        ui.set_width(ui.available_width() * 0.5);

                        ui.label(instrument.description());
                        ui.add_space(window.height() * 0.01);

                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(format!(
                                        "{}: {}{CURRENCY}{}",
                                        if matches!(instrument.kind(), InstrumentKind::Forex(_)) {
                                            "Exchange rate"
                                        } else {
                                            "Price"
                                        },
                                        instrument.current().clean(),
                                        instrument.per_unit()
                                    ));

                                    if !matches!(instrument.kind(), InstrumentKind::Bond(_)) {
                                        ui.add_indicator(instrument.diff());
                                    }
                                });

                                if instrument.volatility() > 0. {
                                    ui.label(format!("Volatility: {:.2}%", instrument.volatility() * 0.5))
                                        .on_hover_text(
                                            "Median daily price fluctuation as percentage of the initial price.",
                                        );
                                }

                                match instrument.kind() {
                                    InstrumentKind::Stock(_) => {
                                        ui.label(format!("Dividend: {}", instrument.dividend().clean()))
                                            .on_hover_text(
                                                "The dividend is a portion of the company's earnings \
                                        distributed to shareholders. It is paid quarterly and the \
                                        amount is at the discretion of the company.",
                                            );

                                        ui.label(format!("Sentiment: {}", instrument.sentiment()))
                                            .on_hover_text(
                                                "People's feelings towards the company (0-100). Higher \
                                        scores means favorable sentiment, thus usually higher stock \
                                        prices.",
                                            );

                                        ui.label(format!("ESG: {}", instrument.esg().to_name()))
                                            .on_hover_text(format!(
                                                "ESG ratings evaluate a company's performance in three key \
                                        areas: Environmental, Social, and Governance. These scores \
                                        help investors assess how responsibly a company operates \
                                        beyond financial metrics. Score {}: {}", instrument.esg().to_name(), instrument.esg().description()));
                                    },
                                    InstrumentKind::Bond(issuer) => {
                                        ui.label(format!("Quality: {}", instrument.quality().to_name()))
                                            .on_hover_text(instrument.quality().description());

                                        if let BondIssuer::Government(country) = issuer {
                                            let country = economy.countries.iter().find(|c| c.name == country).unwrap();
                                            ui.label(format!("Classification: {}", country.market.to_name()))
                                                .on_hover_text(country.market.description());
                                        }

                                        ui.label(format!("Interest: {:.1}%", instrument.interest()))
                                            .on_hover_text(
                                                "Also known as the coupon payment. Fixed interest \
                                        paid to the holder as percentage of the face value.",
                                            );
                                    },
                                    InstrumentKind::Forex(_) => {
                                        ui.label(format!(
                                            "Currency: {} ({})",
                                            instrument.fullname(),
                                            instrument.symbol()
                                        ));
                                    },
                                    InstrumentKind::Commodity(name) => {
                                        ui.label(format!(
                                            "Storage costs: {:.0}{CURRENCY}{}/month",
                                            instrument.storage_cost() * 30.,
                                            instrument.per_unit(),
                                        ))
                                            .on_hover_text(
                                                "Current price of storage per month. Note that this \
                                        price increases with inflation. Storage costs are deducted \
                                        every month or when the commodity is sold.",
                                            );

                                        ui.label(format!(
                                            "Production: {}",
                                            economy
                                                .countries
                                                .iter()
                                                .filter_map(|c| c
                                                    .production
                                                    .keys()
                                                    .contains(&name)
                                                    .then_some(c.name.to_name()))
                                                .collect::<Vec<_>>()
                                                .join(", ")
                                        ))
                                            .on_hover_text(
                                                "Countries producing this commodity. Bond prices for \
                                        these countries might be affected by the commodity price.",
                                            );
                                    },
                                    InstrumentKind::Crypto(_) => {
                                        ui.label(format!(
                                            "Market cap: {}{CURRENCY}",
                                            instrument.market_cap().format()
                                        ))
                                            .on_hover_text(
                                                "Total market capitalization of the cryptocurrency. This \
                                    is a good indication of the coin's popularity and adoption.",
                                            );
                                    },
                                }
                            });

                            if matches!(instrument.kind(), InstrumentKind::Stock(_)) {
                                ui.add_space(window.width() * 0.02);
                                ui.vertical(|ui| {
                                    ui.label("Sector");
                                    for (name, weight) in instrument.sector().iter().sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap()) {
                                        ui.add(
                                            ProgressBar::new(*weight)
                                                .text(RichText::new(format!("{} {}", name.emoji(), name.to_name())).small())
                                                .corner_radius(5.)
                                                .desired_width(ui.available_width() * 0.6)
                                        ).on_hover_text(name.description());
                                    }
                                });
                            }
                        });
                    });

                    if !matches!(instrument.kind(), InstrumentKind::Bond(_)) {
                        ui.add_space(window.width() * 0.01);
                        ui.vertical(|ui| {
                            ui.add_plot(instrument.all(), Some(player.pending_orders().into_iter().filter(|o| o.instrument == instrument.kind()).collect()));
                        });
                    }
                })
            })
            .inner
            .response
            .interact(Sense::hover())
            .interact(Sense::click())
    }
}
