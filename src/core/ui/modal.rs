use bevy::prelude::{EventWriter, Res, ResMut, Single, Window};
use bevy_egui::EguiContexts;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Button, ComboBox, Id, Image, Modal, Separator, Sides, Slider};
use chrono::NaiveDate;
use strum::IntoEnumIterator;

use crate::core::constants::CURRENCY;
use crate::core::factors::Factor;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::commodities::CommodityName;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::orders::{Command, Order, OrderDirection, OrderEv, OrderKind, OrderStatus};
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
                ComboBox::from_id_salt("commodity")
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
                                    0..=(instrument.current() * 5.) as u32,
                                )
                                .show_value(false)
                                .text(format!("{limit_stop} {CURRENCY}")),
                            )
                            .on_hover_text(
                                "If the commodity's price crosses this limit, the order \
                                    is executed.",
                            );
                        });

                        (limit_stop * amount) as f32
                    },
                    OrderKind::TrailingOrder => {
                        ui.horizontal(|ui| {
                            ui.label("Trailing stop:");

                            ui.add(
                                Slider::new(&mut state.modal_info.trailing_stop, 3..=50)
                                    .show_value(false)
                                    .text(format!("{trailing_stop}%")),
                            )
                            .on_hover_text(
                                "If the commodity's price evolves more than this percentage \
                                    away from the min/max price, the order is executed.",
                            );
                        });

                        ((100 + trailing_stop) / 100 * amount) as f32 * instrument.current()
                    },
                    _ => instrument.current() * amount as f32,
                };

                ui.add_space(window.height() * 0.02);

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.vertical(|ui| {
                            ui.label(format!(
                                "Storage costs: {:.0} {CURRENCY}/month",
                                amount as f32 * instrument.storage_cost() * 30.,
                            ))
                            .on_hover_text("Storage costs for the selected amount.");

                            ui.label(format!("{} price: {price:.0} {CURRENCY}", tab.abbr()))
                                .on_hover_text(format!(
                                    "{} price for the selected amount.",
                                    tab.abbr()
                                ));
                        });
                    });

                    if tab == OrderKind::MarketOrder && owned >= amount {
                        ui.add(Separator::default().vertical());

                        ui.vertical(|ui| {
                            ui.label(format!("Open storage costs: {storage_costs:.0} {CURRENCY}"))
                                .on_hover_text(
                                    "Storage costs for this month. If the commodity is sold, \
                                   the costs are deducted from the proceeds.",
                                );

                            ui.label(format!("Proceeds: {:.0} {CURRENCY}", price - storage_costs))
                                .on_hover_text(format!(
                                    "Amount of money earned when selling {} {} of {}. This \
                                    is equal to the selling price of the commodity minus the \
                                    open storage costs.",
                                    amount,
                                    instrument.unit(),
                                    instrument.lowername()
                                ));
                        });
                    }
                });

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
                        direction: if (limit_stop as f32) < instrument.current() {
                            OrderDirection::Lower
                        } else {
                            OrderDirection::Upper
                        },
                        amount,
                        price,
                        threshold: if tab == OrderKind::LimitOrder {
                            limit_stop
                        } else {
                            trailing_stop
                        },
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
