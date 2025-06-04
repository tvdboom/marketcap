use bevy::prelude::{EventWriter, Res, ResMut, Single, Window};
use bevy_egui::EguiContexts;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{
    Align, Button, ComboBox, Id, Image, Layout, Modal, ScrollArea, Sides, Slider, Ui,
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
use crate::core::ui::state::{ActiveModal, OrderOptions, TradeTab, UiState};
use crate::core::ui::utils::{CustomHover, CustomUi, TextSizes, add_text};
use crate::utils::NameFromEnum;

pub fn commodities_panel(
    ui: &mut Ui,
    ui_state: &mut UiState,
    economy: &GlobalEconomy,
    player: &Player,
    images: &ImageIds,
    window: &Window,
) {
    ui.add_text(
        "Commodities are raw materials or primary agricultural products that can be traded. \
        They serve as the building blocks of the global economy, their prices often having a \
        direct impact on bond and stock prices.\n\n\
        Because commodities are physical instruments, they require storage facilities to preserve \
        the products before selling them. This incurs a storage cost, which is a variable price \
        per unit per month. Storage cost prices increase with inflation.",
        window.m_size(),
    );

    ui.separator();

    ScrollArea::vertical().show(ui, |ui| {
        ui.set_width(ui.available_width());

        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            ui.set_max_height(window.height() * 0.05);

            ComboBox::from_id_salt("order")
                .selected_text(add_text("Order by", window.m_size()))
                .show_ui(ui, |ui| {
                    for order in OrderOptions::iter() {
                        ui.selectable_value(
                            &mut ui_state.commodity_modal.order,
                            order,
                            add_text(order.to_name(), window.s_size()),
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
                });

        for commodity in commodities {
            let response = ui.add_commodity(commodity, images, window);

            if response.clicked() {
                ui_state.active_modal = Some(ActiveModal::Commodity);
                ui_state.commodity_modal.name = commodity.name;
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
    if ui_state.active_modal == Some(ActiveModal::Commodity) {
        let kind = &InstrumentKind::Commodity(ui_state.commodity_modal.name);
        let instrument = economy.get(kind);

        let owned = player.get_owned(kind);
        let storage_costs =
            (ui_state.commodity_modal.amount * 30) as f32 * instrument.storage_cost();

        let modal = Modal::new(Id::new("modal")).show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ComboBox::from_id_salt("commodity")
                        .selected_text(add_text(
                            ui_state.commodity_modal.name.to_name(),
                            window.xl_size(),
                        ))
                        .show_ui(ui, |ui| {
                            for name in CommodityName::iter() {
                                ui.selectable_value(
                                    &mut ui_state.commodity_modal.name,
                                    name,
                                    add_text(name.to_name(), window.s_size()),
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
                                add_text(
                                    format!("{}  {}", tab.emoji(), tab.to_name()),
                                    window.l_size(),
                                ),
                            )
                            .on_hover(tab.description(), window.m_size());
                        }
                    });

                    ui.add_space(window.height() * 0.02);

                    ui.horizontal(|ui| {
                        ui.add_text(
                            format!(
                                "Price: {:.0} {CURRENCY}/{}",
                                instrument.current(),
                                instrument.unit()
                            ),
                            window.m_size(),
                        );

                        ui.add_indicator(instrument.diff(), &window);
                    });

                    ui.add_text(
                        format!("Owned: {owned} {}", instrument.unit()),
                        window.m_size(),
                    );
                    ui.add_text(
                        format!("Value: {:.0} {CURRENCY}", player.get_value(kind, &economy)),
                        window.m_size(),
                    );

                    ui.horizontal(|ui| {
                        ui.add_text("Quantity:", window.m_size());

                        // ui.add_slider(&mut ui_state.commodity_modal.amount, ((player.cash.current() / instrument.current()) as u32).max(owned));
                        let amount = ui_state.commodity_modal.amount;
                        ui.spacing_mut().slider_width = window.width() * 0.15;
                        ui.add(
                            Slider::new(
                                &mut ui_state.commodity_modal.amount,
                                0..=((player.cash.current() / instrument.current()) as u32)
                                    .max(owned),
                            )
                            .show_value(false)
                            .text(add_text(
                                format!("{amount} {}", instrument.unit()),
                                window.m_size(),
                            )),
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            if owned > 0 {
                                ui.add_text(
                                    format!("Open storage costs: {storage_costs:.0} {CURRENCY}"),
                                    window.m_size(),
                                )
                                .on_hover(
                                    "Storage costs for this month. If the commodity is sold, \
                                    the costs are deducted from the proceeds.",
                                    window.m_size(),
                                );
                            }

                            ui.add_text(
                                format!(
                                    "Proceeds: {:.0} {CURRENCY}",
                                    instrument.current() * ui_state.commodity_modal.amount as f32 - storage_costs
                                ),
                                window.m_size(),
                            )
                            .on_hover(
                                format!(
                                    "Amount of money earned when selling {} {} of {}. This \
                                    is equal to the current market price of the commodity minus \
                                    the open storage costs.",
                                    ui_state.commodity_modal.amount,
                                    instrument.unit(),
                                    instrument.lowername()
                                ),
                                window.m_size(),
                            );
                        });
                    });

                    let mut buy_clicked = false;
                    let mut sell_clicked = false;
                    let mut close_clicked = false;

                    Sides::new().show(
                        ui,
                        |ui| {
                            ui.add_enabled_ui(
                                ui_state.commodity_modal.amount > 0
                                    && player.cash.current()
                                        >= instrument.current()
                                            * ui_state.commodity_modal.amount as f32,
                                |ui| {
                                    let button = ui
                                        .add_sized(
                                            [window.width() * 0.08, window.height() * 0.05],
                                            Button::new(add_text("Buy", window.xl_size())),
                                        )
                                        .on_hover(
                                            format!(
                                                "Buy {} {} of {}.",
                                                ui_state.commodity_modal.amount,
                                                instrument.unit(),
                                                instrument.lowername(),
                                            ),
                                            window.m_size(),
                                        );

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
                                        Button::new(add_text("Close position", window.xl_size())),
                                    )
                                    .on_hover(
                                        format!("Sell all {}.", instrument.lowername()),
                                        window.m_size(),
                                    )
                                    .on_disabled_hover(
                                        format!("No {} to sell", instrument.lowername()),
                                        window.m_size(),
                                    );

                                if button.clicked() {
                                    close_clicked = true;
                                }

                                ui.add_enabled_ui(owned >= ui_state.commodity_modal.amount, |ui| {
                                    let button = ui
                                        .add_sized(
                                            [window.width() * 0.08, window.height() * 0.05],
                                            Button::new(add_text("Sell", window.xl_size())),
                                        )
                                        .on_hover(
                                            format!(
                                                "Sell {} {} of {}.",
                                                ui_state.commodity_modal.amount,
                                                instrument.unit(),
                                                instrument.lowername()
                                            ),
                                            window.m_size(),
                                        )
                                        .on_disabled_hover(
                                            format!(
                                                "Not enough units of {} to sell.",
                                                instrument.lowername(),
                                            ),
                                            window.m_size(),
                                        );

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
                            owned.amount += ui_state.commodity_modal.amount;
                        } else {
                            player.instruments.push(OwnedInstrument {
                                kind: kind.clone(),
                                amount: ui_state.commodity_modal.amount,
                                interest: 0.,
                            });
                        }

                        player.cash.amount -=
                            instrument.current() * ui_state.commodity_modal.amount as f32;

                        messages.write(MessageEv {
                            message: format!(
                                "Bought {} {} of {}.",
                                ui_state.commodity_modal.amount,
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
                                o.amount = o.amount.saturating_sub(ui_state.commodity_modal.amount);
                            }
                            o.amount > 0
                        });

                        player.cash.amount += instrument.current()
                            * ui_state.commodity_modal.amount as f32
                            - storage_costs;

                        messages.write(MessageEv {
                            message: format!(
                                "Sold {} {} of {}.",
                                ui_state.commodity_modal.amount,
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
