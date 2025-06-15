use bevy::prelude::{EventWriter, Res, ResMut, Single, Window};
use bevy_egui::EguiContexts;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Button, ComboBox, Id, Image, Modal, Sense, Sides, Slider};
use chrono::NaiveDate;
use strum::IntoEnumIterator;

use crate::core::constants::CURRENCY;
use crate::core::factors::Factor;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::commodities::CommodityName;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::orders::{Command, Order, OrderEv, OrderKind, OrderStatus};
use crate::core::player::{InstrumentKind, Player};
use crate::core::resources::ImageIds;
use crate::core::ui::state::UiState;
use crate::core::ui::utils::CustomUi;
use crate::utils::{NameFromEnum, create_guid};

pub fn trade_modal(
    mut contexts: EguiContexts,
    mut state: ResMut<UiState>,
    economy: Res<GlobalEconomy>,
    mut player: ResMut<Player>,
    mut order_ev: EventWriter<OrderEv>,
    mut message: EventWriter<MessageEv>,
    images: Res<ImageIds>,
    window: Single<&Window>,
) {
    let kind = if let Some(InstrumentKind::Commodity(name)) = state.modal {
        InstrumentKind::Commodity(name)
    } else {
        return;
    };

    let instrument = economy.get(&kind);
    let owned = player.get_owned(&kind);
    let tab = state.modal_info.tab;
    let amount = state.modal_info.amount;
    let limit_stop = state.modal_info.limit_stop;
    let trailing_stop = state.modal_info.trailing_stop;
    let storage_costs = (amount * 30) as f32 * instrument.storage_cost();

    let modal = Modal::new(Id::new("modal")).show(contexts.ctx_mut(), |ui| {
        ui.set_min_width(window.width() * 0.5);

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ComboBox::from_id_salt("instrument")
                    .selected_text(instrument.name())
                    .show_ui(ui, |ui| {
                        for item in CommodityName::iter() {
                            ui.selectable_value(
                                &mut state.modal,
                                Some(InstrumentKind::Commodity(item)),
                                item.to_name(),
                            );
                        }
                    });

                ui.add(Image::new(SizedTexture::new(
                    images.get(instrument.lowername().as_str()),
                    [window.height() * 0.2; 2],
                )));
            });

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    for tab in [
                        OrderKind::MarketOrder,
                        OrderKind::LimitOrder,
                        OrderKind::TrailingOrder,
                        OrderKind::ShortSelling,
                        OrderKind::Futures,
                    ] {
                        ui.selectable_value(
                            &mut state.modal_info.tab,
                            tab,
                            format!("{}  {}", tab.emoji(), tab.to_name()),
                        )
                        .on_hover_text(tab.description());
                    }
                });

                ui.add_space(window.height() * 0.02);

                ui.horizontal(|ui| {
                    ui.label(format!(
                        "Price: {:.0} {CURRENCY}/{}",
                        instrument.current(),
                        instrument.unit()
                    ));

                    ui.add_indicator(instrument.diff());
                });

                ui.label(format!("Owned: {owned} {}", instrument.unit()));
                ui.label(format!(
                    "Value: {:.0} {CURRENCY}",
                    player.get_value(&kind, &economy)
                ));

                ui.spacing_mut().slider_width = window.width() * 0.17;

                ui.horizontal(|ui| {
                    ui.label("Quantity:");

                    ui.add(
                        Slider::new(
                            &mut state.modal_info.amount,
                            0..=((player.cash.current() / instrument.current()) as u32).max(owned),
                        )
                        .show_value(false)
                        .text(format!("{amount} {}", instrument.unit())),
                    );
                });

                let price = match tab {
                    OrderKind::LimitOrder => {
                        ui.horizontal(|ui| {
                            ui.label("Limit stop:");

                            ui.add(
                                Slider::new(
                                    &mut state.modal_info.limit_stop,
                                    0..=(instrument.current() * 2.) as u32,
                                )
                                .show_value(false)
                                .text(format!("{limit_stop} {CURRENCY}")),
                            )
                            .on_hover_text("If the price crosses this limit, the order is executed.");
                        });

                        (limit_stop * amount) as f32
                    },
                    OrderKind::TrailingOrder => {
                        ui.horizontal(|ui| {
                            ui.label("Trailing stop:");

                            ui.add(
                                Slider::new(&mut state.modal_info.trailing_stop, 5..=30)
                                    .show_value(false)
                                    .text(format!("{trailing_stop}%")),
                            )
                            .on_hover_text(
                                "If the price evolves more than this percentage \
                                    away from the min/max price, the order is executed.",
                            );
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Bound:").on_hover_text(
                                "Whether the trailing stop should be applied to the \
                                    upper or lower bound.",
                            );
                            let dir = ui.label(if state.modal_info.lower_bound {
                                "▼ Lower"
                            } else {
                                "▲ Upper"
                            }).on_hover_text(if state.modal_info.lower_bound {
                                format!("Lower bound, i.e., the order is executed when te price surpasses {trailing_stop}% above the minimum.")
                            } else {
                                format!("Upper bound, i.e., the order is executed when the price decreases {trailing_stop}% below the maximum.")
                            }).interact(Sense::click());
                            
                            if dir.clicked() {
                                state.modal_info.lower_bound = !state.modal_info.lower_bound;
                            }
                        });

                        if state.modal_info.lower_bound {
                            (100 + trailing_stop) as f32 / 100. * amount as f32 * instrument.current()
                        } else {
                            (100 - trailing_stop) as f32 / 100. * amount as f32 * instrument.current()
                        }
                    },
                    _ => instrument.current() * amount as f32,
                };

                if matches!(kind, InstrumentKind::Commodity(_)) {
                    ui.label(format!("Storage costs: {storage_costs:.0} {CURRENCY}/month"))
                        .on_hover_text(
                            "Storage costs for the selected amount. This amount is deducted \
                            from the proceeds of a sale to pay for the open costs of the current \
                            month.",
                        );
                }

                if tab == OrderKind::TrailingOrder {
                    ui.label(format!("Trailing price: {:.0} {CURRENCY}", if amount == 0 { 0. } else { price / amount as f32}))
                        .on_hover_text(
                            "If the price surpasses this value (greater for lower bound \
                            or lesser for upper bound), the order is executed.",
                        );
                }
                
                ui.label(format!("Total price: {price:.0} {CURRENCY}"));

                ui.add_space(window.height() * 0.02);

                let mut buy_clicked = false;
                let mut sell_clicked = false;
                let mut close_clicked = false;

                Sides::new().show(
                    ui,
                    |ui| {
                        ui.add_enabled_ui(
                            amount > 0
                                && (tab != OrderKind::MarketOrder
                                    || player.cash.current() >= price),
                            |ui| {
                                let button = ui
                                    .add_sized(
                                        [window.width() * 0.08, window.height() * 0.05],
                                        Button::new(if tab == OrderKind::MarketOrder {
                                            "Buy"
                                        } else {
                                            "Buy order"
                                        }),
                                    )
                                    .on_hover_text(format!(
                                        "Buy {} {} of {}.",
                                        amount,
                                        instrument.unit(),
                                        instrument.lowername(),
                                    ));

                                if button.clicked() {
                                    buy_clicked = true;
                                }
                            },
                        );
                    },
                    |ui| {
                        ui.add_enabled_ui(owned > 0 || tab != OrderKind::MarketOrder, |ui| {
                            let button = ui
                                .add_sized(
                                    [window.width() * 0.08, window.height() * 0.05],
                                    Button::new(if tab == OrderKind::MarketOrder {
                                        "Close position"
                                    } else {
                                        "Close order"
                                    }),
                                )
                                .on_hover_text(format!(
                                    "Sell all owned {}.",
                                    instrument.lowername()
                                ))
                                .on_disabled_hover_text(format!(
                                    "No {} to sell",
                                    instrument.lowername()
                                ));

                            if button.clicked() {
                                close_clicked = true;
                            }

                            ui.add_enabled_ui(
                                amount > 0 && (tab != OrderKind::MarketOrder || owned >= amount),
                                |ui| {
                                    let button = ui
                                        .add_sized(
                                            [window.width() * 0.08, window.height() * 0.05],
                                            Button::new(if tab == OrderKind::MarketOrder {
                                                "Sell"
                                            } else {
                                                "Sell order"
                                            }),
                                        )
                                        .on_hover_text(format!(
                                            "Sell {} {} of {}.",
                                            amount,
                                            instrument.unit(),
                                            instrument.lowername()
                                        ))
                                        .on_disabled_hover_text(format!(
                                            "Not enough units of {} to sell.",
                                            instrument.lowername(),
                                        ));

                                    if button.clicked() {
                                        sell_clicked = true;
                                    }
                                },
                            );
                        });
                    },
                );

                let command = if buy_clicked {
                    Some(Command::Buy)
                } else if sell_clicked {
                    Some(Command::Sell)
                } else if close_clicked {
                    Some(Command::Close)
                } else {
                    None
                };

                if let Some(command) = command {
                    let mut order = Order {
                        id: create_guid(),
                        created: economy.date,
                        instrument: kind,
                        command,
                        kind: tab,
                        lower_bound: if tab == OrderKind::LimitOrder {
                            (limit_stop as f32) < instrument.current()
                        } else {
                            state.modal_info.lower_bound
                        },
                        amount,
                        price: price
                            - if command == Command::Buy {
                                0.
                            } else {
                                storage_costs
                            },
                        threshold: if tab == OrderKind::LimitOrder {
                            limit_stop
                        } else {
                            trailing_stop
                        },
                        bound: instrument.current(),
                        processed: NaiveDate::default(),
                        status: OrderStatus::Executed,
                    };

                    if tab != OrderKind::MarketOrder {
                        order.status = OrderStatus::Pending;

                        message.write(MessageEv {
                            message: format!(
                                "Created {} {} order {}.",
                                tab.abbr().to_lowercase(),
                                command.to_lowername(),
                                order.id
                            ),
                            level: MessageLevel::Info,
                        });
                    } else {
                        order_ev.write(OrderEv {
                            id: order.id.clone(),
                            price,
                        });
                    }

                    player.orders.push(order);
                }
            });
        });
    });

    if modal.should_close() {
        state.modal = None;
    }
}
