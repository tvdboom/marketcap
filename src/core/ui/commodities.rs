use bevy::prelude::{EventWriter, Res, ResMut, Single, Window};
use bevy_egui::EguiContexts;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{
    Align, Button, ComboBox, Id, Image, Layout, Modal, ScrollArea, Separator, Sides, Slider, Ui,
};
use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::core::constants::CURRENCY;
use crate::core::factors::Factor;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::Instrument;
use crate::core::instruments::commodities::CommodityName;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::player::{InstrumentKind, OwnedInstrument, Player};
use crate::core::resources::ImageIds;
use crate::core::ui::state::{OrderOptions, TradeTab, UiState};
use crate::core::ui::utils::CustomUi;
use crate::utils::NameFromEnum;

pub fn commodities_panel(
    ui: &mut Ui,
    ui_state: &mut UiState,
    economy: &GlobalEconomy,
    player: &Player,
    images: &ImageIds,
    window: &Window,
) {
    ui.label(
        "Commodities are raw materials or primary agricultural products that can be traded. \
        They serve as the building blocks of the global economy, their prices often having a \
        direct impact on bond and stock prices.\n\n\
        Because commodities are physical instruments, they require storage facilities to preserve \
        the products before selling them. This incurs a storage cost, which is a variable price \
        per unit per month (with a minimum of one month). Storage cost prices increase with \
        inflation.",
    );

    ui.separator();

    ScrollArea::vertical().show(ui, |ui| {
        ui.set_width(ui.available_width());

        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            ui.set_max_height(window.height() * 0.05);

            ui.add_space(window.width() * 0.02);

            ComboBox::from_id_salt("order")
                .selected_text("Order by")
                .show_ui(ui, |ui| {
                    for order in OrderOptions::iter().filter(|o| {
                        !matches!(
                            o,
                            OrderOptions::HighestInterest | OrderOptions::LowestInterest
                        )
                    }) {
                        ui.selectable_value(
                            &mut ui_state.commodity_modal.order,
                            order,
                            order.to_name(),
                        );
                    }
                });
        });

        let commodities =
            economy
                .commodities
                .iter()
                .sorted_by(|a, b| match ui_state.commodity_modal.order {
                    OrderOptions::Alphabetical => a.name.to_lowername().cmp(&b.name.to_lowername()),
                    OrderOptions::OwnedAmount => player
                        .get_owned(&InstrumentKind::Commodity(b.name))
                        .cmp(&player.get_owned(&InstrumentKind::Commodity(a.name))),
                    OrderOptions::OwnedValue => player
                        .get_value(&InstrumentKind::Commodity(b.name), economy)
                        .partial_cmp(&player.get_value(&InstrumentKind::Commodity(a.name), economy))
                        .unwrap_or(std::cmp::Ordering::Equal),
                    OrderOptions::LowestPrice => a
                        .current()
                        .partial_cmp(&b.current())
                        .unwrap_or(std::cmp::Ordering::Equal),
                    OrderOptions::HighestPrice => b
                        .current()
                        .partial_cmp(&a.current())
                        .unwrap_or(std::cmp::Ordering::Equal),
                    OrderOptions::LowestVolatility => a
                        .volatility
                        .partial_cmp(&b.volatility)
                        .unwrap_or(std::cmp::Ordering::Equal),
                    OrderOptions::HighestVolatility => b
                        .volatility
                        .partial_cmp(&a.volatility)
                        .unwrap_or(std::cmp::Ordering::Equal),
                    _ => unreachable!(),
                });

        for commodity in commodities {
            let response = ui.add_commodity(commodity, images, window);

            if response.clicked() {
                ui_state.active_modal = Some(InstrumentKind::Commodity(commodity.name));
            }
        }
    });
}

pub fn commodity_modal(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    economy: Res<GlobalEconomy>,
    mut player: ResMut<Player>,
    mut messages: EventWriter<MessageEv>,
    images: Res<ImageIds>,
    window: Single<&Window>,
) {
    if matches!(ui_state.active_modal, Some(InstrumentKind::Commodity(_))) {
        let kind = &ui_state.active_modal.unwrap();
        let instrument = economy.get(kind);

        // Number of units of this commodity owned by the player
        let owned = player.get_owned(kind);

        // Selected amount to buy/sell
        let amount = ui_state.commodity_modal.amount;

        // Storage cost for the selected amount for 30 days
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
                                    &mut ui_state.active_modal,
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
                            TradeTab::MarketOrder,
                            TradeTab::LimitOrder,
                            TradeTab::Futures,
                        ] {
                            ui.selectable_value(
                                &mut ui_state.commodity_modal.tab,
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
                        player.get_value(kind, &economy)
                    ));

                    ui.horizontal(|ui| {
                        ui.label("Quantity:");

                        ui.spacing_mut().slider_width = window.width() * 0.15;
                        ui.add(
                            Slider::new(
                                &mut ui_state.commodity_modal.amount,
                                0..=((player.cash.current() / instrument.current()) as u32)
                                    .max(owned),
                            )
                            .show_value(false)
                            .text(format!("{amount} {}", instrument.unit())),
                        );
                    });

                    ui.add_space(window.height() * 0.02);

                    ui.horizontal(|ui| {
                        if player.cash.current() >= instrument.current() * amount as f32 {
                            ui.vertical(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(format!(
                                        "Storage costs: {:.0} {CURRENCY}/month",
                                        instrument.storage_cost() * 30.,
                                    ))
                                    .on_hover_text("Storage costs for the selected amount.");

                                    ui.label(format!(
                                        "Price: {:.0} {CURRENCY}",
                                        amount as f32 * instrument.current(),
                                    ))
                                    .on_hover_text("Total price for the selected amount.");
                                });
                            });
                        }

                        if owned >= amount {
                            if player.cash.current() >= instrument.current() * amount as f32 {
                                ui.add(Separator::default().vertical());
                            }

                            ui.vertical(|ui| {
                                ui.label(format!(
                                    "Open storage costs: {storage_costs:.0} {CURRENCY}"
                                ))
                                .on_hover_text(
                                    "Storage costs for this month. If the commodity is sold, \
                                    the costs are deducted from the proceeds.",
                                );

                                ui.label(format!(
                                    "Proceeds: {:.0} {CURRENCY}",
                                    instrument.current() * amount as f32 - storage_costs
                                ))
                                .on_hover_text(format!(
                                    "Amount of money earned when selling {} {} of {}. This \
                                    is equal to the current market price of the commodity minus \
                                    the open storage costs.",
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
                                    && player.cash.current()
                                        >= instrument.current() * amount as f32,
                                |ui| {
                                    let button = ui
                                        .add_sized(
                                            [window.width() * 0.08, window.height() * 0.05],
                                            Button::new("Buy"),
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
                            ui.add_enabled_ui(owned > 0, |ui| {
                                let button = ui
                                    .add_sized(
                                        [window.width() * 0.08, window.height() * 0.05],
                                        Button::new("Close position"),
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

                                ui.add_enabled_ui(amount > 0 && owned >= amount, |ui| {
                                    let button = ui
                                        .add_sized(
                                            [window.width() * 0.08, window.height() * 0.05],
                                            Button::new("Sell"),
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
                                });
                            });
                        },
                    );

                    // Resolve button clicks
                    if buy_clicked {
                        if let Some(owned) = player.instruments.iter_mut().find(|o| o.kind == *kind)
                        {
                            owned.amount += amount;
                        } else {
                            player.instruments.push(OwnedInstrument {
                                kind: kind.clone(),
                                amount,
                                interest: 0.,
                            });
                        }

                        player.cash.amount -= instrument.current() * amount as f32;

                        messages.write(MessageEv {
                            message: format!(
                                "Bought {} {} of {}.",
                                amount,
                                instrument.unit(),
                                instrument.lowername()
                            ),
                            level: MessageLevel::Info,
                        });
                    }

                    if close_clicked {
                        player.cash.amount += instrument.current() * owned as f32 - storage_costs;
                        player.instruments.retain(|s| s.kind != *kind);

                        messages.write(MessageEv {
                            message: format!("Closed {} position.", instrument.lowername()),
                            level: MessageLevel::Info,
                        });
                    }

                    if sell_clicked {
                        player.instruments.retain_mut(|o| {
                            if o.kind == *kind {
                                o.amount = o.amount.saturating_sub(amount);
                            }
                            o.amount > 0
                        });

                        player.cash.amount += instrument.current() * amount as f32 - storage_costs;

                        messages.write(MessageEv {
                            message: format!(
                                "Sold {} {} of {}.",
                                amount,
                                instrument.unit(),
                                instrument.lowername()
                            ),
                            level: MessageLevel::Info,
                        });
                    }
                });
            });
        });

        if modal.should_close() {
            ui_state.active_modal = None;
        }
    }
}
