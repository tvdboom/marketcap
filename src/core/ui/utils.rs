use bevy::prelude::Window;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::*;
use chrono::{Datelike, Duration, NaiveDate};
use egui_plot::{AxisHints, GridMark, Line, Plot, PlotPoints};
use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::core::constants::{
    CURRENCY, CUSTOM_GREEN, DATE_FORMAT, HEIGHT, LINE_COLOR, LINE_WIDTH, WIDTH,
};
use crate::core::countries::Country;
use crate::core::global_economy::{GlobalEconomy, PoliticalLandscape};
use crate::core::instruments::bonds::BondIssuer;
use crate::core::instruments::instrument::{Instrument, InstrumentKind};
use crate::core::orders::{Command, Order};
use crate::core::player::Player;
use crate::core::research::{TechName, Technology};
use crate::core::resources::ImageIds;
use crate::core::ui::state::{OrderByState, OrderOptions, PlotRange};
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
            ui.painter().circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
        }

        response
    }
}

pub trait CustomUi {
    fn add_button(&mut self, text: impl Into<WidgetText>, window: &Window) -> Response;
    fn add_modal_button(&mut self, text: impl Into<WidgetText>, window: &Window) -> Response;
    fn add_image(&mut self, texture: impl Into<TextureId>, size: impl Into<Vec2>) -> Response;
    fn add_indicator(&mut self, diff: f32) -> Response;
    fn add_combobox(
        &mut self,
        title: &str,
        options: Vec<OrderOptions>,
        state: &mut OrderByState,
        window: &Window,
    );
    fn add_bar(&mut self, value: i32);
    fn add_technology(&mut self, research: &Technology) -> Response;
    fn add_country(&mut self, country: &Country, images: &ImageIds, window: &Window);
    fn add_plot(
        &mut self,
        data: &DQueue<f32>,
        today: NaiveDate,
        range: &PlotRange,
        orders: Option<Vec<&Order>>,
    );
    fn add_factor(
        &mut self,
        name: impl Into<RichText>,
        value: impl Into<WidgetText>,
        color: impl Into<Color32>,
        texture_id: TextureId,
        description: String,
        plot: Option<&DQueue<f32>>,
        today: NaiveDate,
        window: &Window,
    ) -> Response;
    fn add_instrument(
        &mut self,
        instrument: &dyn Instrument,
        economy: &GlobalEconomy,
        player: &Player,
        state: Option<&mut PlotRange>,
        images: &ImageIds,
        window: &Window,
    ) -> Response;
}

impl CustomUi for Ui {
    fn add_button(&mut self, text: impl Into<WidgetText>, window: &Window) -> Response {
        self.add_sized(
            [(window.width() * 0.2).min(300.), (window.height() * 0.075).min(70.)],
            Button::new(text),
        )
    }

    fn add_modal_button(&mut self, text: impl Into<WidgetText>, window: &Window) -> Response {
        self.add_sized([window.width() * 0.08, window.height() * 0.05], Button::new(text))
    }

    fn add_image(&mut self, texture: impl Into<TextureId>, size: impl Into<Vec2>) -> Response {
        self.add(Image::new(SizedTexture::new(texture, size)))
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

                ComboBox::from_id_salt(title).selected_text(state.order.to_name()).show_ui(
                    ui,
                    |ui| {
                        for order in options {
                            ui.selectable_value(&mut state.order, order, order.to_name());
                        }
                    },
                );

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

    fn add_bar(&mut self, value: i32) {
        let norm = (value / PoliticalLandscape::RANGE) as f32;

        let (rect, _) =
            self.allocate_exact_size(vec2(self.available_width(), 40.), Sense::hover());
        let painter = self.painter();

        let center_x = rect.center().x;
        let y_range = rect.top()..=rect.bottom();

        let bar_width = rect.width() * norm.abs() * 0.5;
        let (x0, x1) = if norm > 0.0 {
            (center_x, center_x + bar_width)
        } else {
            (center_x - bar_width, center_x)
        };
        let bar_rect = Rect::from_x_y_ranges(x0..=x1, y_range.clone());
        painter.rect_filled(bar_rect, 2.0, Color32::RED);

        painter.rect_stroke(rect, 2.0, (1.0, Color32::LIGHT_GRAY), StrokeKind::Middle);

        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            format!("{:+}", value),
            TextStyle::Body.resolve(self.style()),
            Color32::WHITE,
        );
    }

    fn add_technology(&mut self, technology: &Technology) -> Response {
        self.scope_builder(
            UiBuilder::new().id_salt(technology.name.to_name()).sense(Sense::click()),
            |ui| {
                let response = ui.response();
                let visuals = ui.style().interact(&response);

                Frame::canvas(ui.style())
                    .fill(if technology.progress == 100. {
                        CUSTOM_GREEN
                    } else if technology.researching {
                        visuals.bg_fill
                    } else {
                        visuals.bg_fill.gamma_multiply(0.7)
                    })
                    .stroke(visuals.bg_stroke)
                    .inner_margin(ui.spacing().menu_margin)
                    .show(ui, |ui| {
                        ui.set_width(180.);

                        ui.vertical_centered(|ui| {
                            ui.add_space(5.);
                            Label::new(RichText::new(technology.name.to_name()).strong())
                                .selectable(false)
                                .ui(ui);

                            ui.add_space(20.);

                            ui.add(
                                ProgressBar::new(technology.progress / 100.)
                                    .show_percentage()
                                    .corner_radius(5.),
                            );
                        });
                    });
            },
        )
        .response
        .on_hover_text(technology.name.description())
        .on_disabled_hover_text(format!(
            "{}{}",
            technology.name.description(),
            if let Some(deps) = &technology.dependencies {
                format!("\n\nDependencies: {}.", deps.iter().map(|d| d.to_name()).join(", "))
            } else {
                String::new()
            }
        ))
        .on_hover_cursor(CursorIcon::PointingHand)
    }

    fn add_country(&mut self, country: &Country, images: &ImageIds, window: &Window) {
        let ratio = 5. * get_ratio(window.width(), window.height(), TextStyle::Heading);

        self.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading(country.name.to_name());
                ui.add_image(
                    images.get(format!("{}-flag", country.name.to_lowername()).as_str()),
                    [ratio, ratio / 16. * 9.],
                );
            });

            ui.add_space(window.width() * 0.02);

            ui.vertical(|ui| {
                ui.label(country.name.description());

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Characteristics");

                        ui.label(format!(
                            "Currency: {} ({})",
                            country.currency.fullname(),
                            country.currency.symbol()
                        ));
                        ui.label(format!("Classification: {}", country.market.to_name()));
                        ui.label(format!("GDP: {} trillion euros", country.gdp));
                    });

                    ui.add_space(window.width() * 0.02);

                    ui.vertical(|ui| {
                        ui.label("Politics");

                        ui.label(format!(
                            "👑 Governance: {}",
                            country.politics.governance.to_name()
                        ));
                        ui.label(format!("🍀 Ideology: {}", country.politics.ideology.to_name()));
                        ui.label(format!("👨‍ Culture: {}", country.politics.culture.to_name()));
                        ui.label(format!(
                            "💲 Orientation: {}",
                            country.politics.orientation.to_name()
                        ));
                    });

                    ui.add_space(window.width() * 0.02);

                    ui.vertical(|ui| {
                        ui.label("Production");

                        let production = country
                            .production
                            .iter()
                            .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap())
                            .collect::<Vec<_>>();
                        let max = production.first().unwrap().1;
                        for (name, weight) in production {
                            ui.horizontal(|ui| {
                                ui.add_image(images.get(name.to_lowername().as_str()), [20.; 2]);
                                ui.add(
                                    ProgressBar::new(weight / max)
                                        .text(RichText::new(name.to_name()).small())
                                        .corner_radius(5.)
                                        .desired_width(ui.available_width().max(250.)),
                                );
                            });
                        }
                    });
                });
            });
        });
    }

    fn add_plot(
        &mut self,
        data: &DQueue<f32>,
        today: NaiveDate,
        range: &PlotRange,
        orders: Option<Vec<&Order>>,
    ) {
        let days = (range.days(&today) as usize).min(data.len());
        let start = data.len() - days;
        let init_date = today - Duration::days(days as i64);

        let points: PlotPoints =
            data.iter().skip(start).enumerate().map(|(i, &v)| [i as f64, v as f64]).collect();

        Plot::new(create_guid())
            .sense(Sense::empty()) // Disable dragging
            .view_aspect(WIDTH / HEIGHT)
            .show_background(false)
            .x_grid_spacer(|grid| {
                (grid.bounds.0 as i64..grid.bounds.1 as i64)
                    .map(|x| {
                        let d = init_date + Duration::days(x);
                        GridMark {
                            value: x as f64,
                            step_size: if d.day() == 1
                                && (days <= 180
                                    || (days > 180 && days <= 365 && d.month() % 2 == 0)
                                    || (days > 365 && d.month() % 3 == 0))
                            {
                                300.
                            } else {
                                0.
                            },
                        }
                    })
                    .collect()
            })
            .custom_x_axes(vec![AxisHints::new_x().formatter(|mark, _| {
                let d = init_date + Duration::days(mark.value as i64);
                format!("{:02}-{}", d.month(), d.year())
            })])
            .custom_y_axes(vec![
                AxisHints::new_x().formatter(|mark, _| (mark.value as f32).format()),
            ])
            .label_formatter(|name, point| {
                if !name.is_empty() {
                    let d = init_date + Duration::days(point.x as i64);
                    format!(
                        "price: {}{CURRENCY}\ndate: {}",
                        (point.y as f32).clean(),
                        d.format(DATE_FORMAT)
                    )
                } else {
                    "".to_owned()
                }
            })
            .show(self, |plot_ui| {
                plot_ui.line(Line::new("price", points).width(LINE_WIDTH).color(LINE_COLOR));

                if let Some(orders) = orders {
                    for order in orders {
                        let points: PlotPoints = data
                            .iter()
                            .skip(start)
                            .enumerate()
                            .map(|(i, _)| {
                                [
                                    i as f64,
                                    if order.created <= init_date + Duration::days(i as i64) {
                                        order.limit_price() as f64
                                    } else {
                                        f64::NAN
                                    },
                                ]
                            })
                            .collect();

                        plot_ui.line(
                            Line::new(create_guid(), points).width(LINE_WIDTH - 1.).color(
                                match order.command {
                                    Command::Buy => Color32::LIGHT_GREEN,
                                    Command::Sell => Color32::LIGHT_RED,
                                    Command::Close => Color32::RED,
                                },
                            ),
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
        today: NaiveDate,
        window: &Window,
    ) -> Response {
        self.horizontal_centered(|ui| {
            ui.add_image(
                texture_id,
                [get_ratio(window.width(), window.height(), TextStyle::Heading); 2],
            );
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
                ui.add_plot(values, today, &PlotRange::default(), None);
            }
        })
    }

    fn add_instrument(
        &mut self,
        instrument: &dyn Instrument,
        economy: &GlobalEconomy,
        player: &Player,
        state: Option<&mut PlotRange>,
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
                    let response = ui.vertical(|ui| {
                        if matches!(instrument.kind(), InstrumentKind::Forex(_)) {
                            ui.heading(format!("{}/{}", instrument.name(), CURRENCY.to_name()));
                        } else {
                            ui.heading(instrument.name());
                        }

                        ui.add_image(
                            images.get(instrument.image().as_str()),
                            [window.height() * 0.2; 2],
                        );
                    });

                    if let Some(country) = instrument.country() {
                        let country = economy.countries.iter().find(|c| c.name == country).unwrap();
                        response.response.on_hover_ui(|ui| ui.add_country(country, images, window));
                    }

                    ui.vertical(|ui| {
                        ui.set_width(ui.available_width() * 0.5);

                        ui.label(instrument.description());
                        ui.add_space(window.height() * 0.01);

                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(format!(
                                        "{}: {}{CURRENCY}{}",
                                        match instrument.kind() {
                                            InstrumentKind::Bond(_) => "Face value",
                                            InstrumentKind::Forex(_) => "Exchange rate",
                                            _ => "Price"
                                        },
                                        instrument.current().clean(),
                                        if let InstrumentKind::Bond(BondIssuer::Government(country)) = instrument.kind() {
                                            let currency = economy
                                                .currencies
                                                .iter()
                                                .find(|c| c.country == country)
                                                .unwrap();

                                            if currency.name != CURRENCY {
                                                format!(" (10.000{})", currency.symbol())
                                            } else {
                                                String::new()
                                            }
                                        } else {
                                            instrument.per_unit()
                                        }
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
                                        ui.label(format!("Dividend: {}{CURRENCY}/share", instrument.dividend().clean()))
                                            .on_hover_text(
                                                "The dividend is a portion of the company's earnings \
                                                distributed to shareholders. It is paid quarterly and the \
                                                amount is at the discretion of the company (the shown amount \
                                                is an indication).",
                                            );

                                        ui.label(format!("Sentiment: {}", instrument.sentiment()))
                                            .on_hover_text(
                                                "People's feelings towards the company (0-100). Higher \
                                                scores means favorable sentiment, thus usually higher stock \
                                                prices.",
                                            );

                                        if player.has_tech(&TechName::ESG) {
                                            ui.label(format!("ESG: {}", instrument.esg().to_name()))
                                                .on_hover_text(format!(
                                                    "ESG ratings evaluate a company's performance in three key \
                                                    areas: Environmental, Social, and Governance. These scores \
                                                    help investors assess how responsibly a company operates \
                                                    beyond financial metrics. Score {}: {}", instrument.esg().to_name(), instrument.esg().description())
                                                );
                                        }
                                    },
                                    InstrumentKind::Bond(issuer) => {
                                        ui.label(format!("Quality: {}", instrument.quality().to_name()))
                                            .on_hover_text(instrument.quality().description());

                                        if let BondIssuer::Government(country) = issuer {
                                            let country = economy.countries.iter().find(|c| c.name == country).unwrap();
                                            ui.label(format!("Classification: {}", country.market.to_name()))
                                                .on_hover_text(country.market.description());
                                        }

                                        ui.label(format!("Interest: ≥{:.1}%", instrument.interest()))
                                            .on_hover_text(
                                                "Also known as the coupon payment. Fixed interest \
                                                paid to the holder as percentage of the face value. \
                                                The interest increases with the bond's term.",
                                            );
                                    },
                                    InstrumentKind::Forex(_) => {
                                        ui.label(format!("Country: {}", instrument.country().to_name()));

                                        ui.label(format!(
                                            "Currency: {} ({})",
                                            instrument.fullname(),
                                            instrument.symbol()
                                        ));
                                    },
                                    InstrumentKind::Commodity(_) => {
                                        ui.label(format!(
                                            "Classification: {}",
                                            instrument.group().to_name(),
                                        ))
                                            .on_hover_text(
                                                "Group the commodity belongs to. Some events \
                                                or policies might affect a specific group only."
                                            );

                                        ui.label(format!(
                                            "Storage costs: {:.0}{CURRENCY}{}/month",
                                            instrument.storage_cost(economy, player) * 30.,
                                            instrument.per_unit(),
                                        ))
                                            .on_hover_text(
                                                "Current price of storage per month. Note that this \
                                                price increases with inflation. Storage costs are deducted \
                                                every month or when the commodity is sold.",
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

                            match instrument.kind() {
                                InstrumentKind::Stock(_) => {
                                    ui.add_space(window.width() * 0.02);
                                    ui.vertical(|ui| {
                                        ui.label("Sectors").on_hover_text("The sectors the company operates in.");
                                        for (name, weight) in instrument.sectors().iter().sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap()) {
                                            ui.add(
                                                ProgressBar::new(*weight)
                                                    .text(RichText::new(format!("{} {}", name.emoji(), name.to_name())).small())
                                                    .corner_radius(5.)
                                                    .desired_width(ui.available_width() * 0.6)
                                            ).on_hover_text(name.description());
                                        }
                                    });
                                },
                                InstrumentKind::Commodity(name) => {
                                    ui.add_space(window.width() * 0.02);
                                    ui.vertical(|ui| {
                                        ui.label("Production").on_hover_text("The countries that produce this commodity.");

                                        let mut production = economy
                                            .countries
                                            .iter()
                                            .filter_map(|c| c
                                                .production
                                                .iter()
                                                .find(|(n, _)| **n == name)
                                                .map(|(_, w)| (c, *w)))
                                            .collect::<Vec<_>>();

                                        production.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

                                        let max = production.iter().map(|(_, w)| *w).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
                                        for (country, weight) in production {
                                            ui.horizontal(|ui| {
                                                ui.add_image(
                                                    images.get(format!("{}-flag", country.name.to_lowername()).as_str()),
                                                    [30., 17.],
                                                );

                                                ui.add(
                                                    ProgressBar::new(weight / max)
                                                        .text(RichText::new(country.name.to_name()).small())
                                                        .corner_radius(5.)
                                                        .desired_width(ui.available_width() * 0.8)
                                                );
                                            })
                                            .response
                                            .on_hover_ui(|ui| ui.add_country(country, images, window));
                                        }
                                    });
                                }
                                _ => ()
                            }
                        });
                    });

                    if let Some(state) = state {
                        ui.add_space(window.width() * 0.01);

                        ui.vertical(|ui| {
                            let orders = player.pending_orders().into_iter().filter(|o| o.instrument == instrument.kind()).collect();
                            ui.add_plot(instrument.all(), economy.date, state, Some(orders));

                            ui.horizontal(|ui| {
                                for tab in PlotRange::iter() {
                                    ui.selectable_value(
                                        state,
                                        tab.clone(),
                                        RichText::new(tab.display()).small(),
                                    ).on_hover_text(tab.description());
                                }
                            });
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
