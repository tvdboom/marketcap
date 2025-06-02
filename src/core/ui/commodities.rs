use crate::core::factors::Factor;
use crate::core::global_economy::GlobalEconomy;
use crate::core::instruments::commodities::CommodityName;
use crate::core::messages::{MessageEv, MessageLevel};
use crate::core::player::{OwnedCommodity, Player};
use crate::core::resources::ImageIds;
use crate::core::ui::state::{ActiveModal, TradeTab, UiState};
use crate::core::ui::utils::{CustomHover, CustomUi, TextSizes, add_text};
use crate::utils::{NameFromEnum, create_guid};
use bevy::prelude::{EventWriter, Res, ResMut, Single, Window};
use bevy_egui::EguiContexts;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Button, ComboBox, Id, Image, Modal, ScrollArea, Sides, Slider, Ui};
use itertools::Itertools;
use strum::IntoEnumIterator;

pub fn commodities_panel(
    ui: &mut Ui,
    ui_state: &mut UiState,
    economy: &GlobalEconomy,
    images: &ImageIds,
    window: &Window,
) {
    ui.add_text(
        "Commodities are raw materials or primary agricultural products that can be \
        bought, sold and traded. They serve as the building blocks of the global economy,\
        their prices often having a direct impact on stock prices.\n\n\
        Commodities typically present volatile price movements. Be aware, commodities are \
        natural products that degrade over time. Be sure to sell them before they lose their \
        value.",
        window.m_size(),
    );

    ui.separator();

    ui.add_space(window.height() * 0.02);

    ScrollArea::vertical().show(ui, |ui| {
        for security in economy.commodities.iter() {
            let response = ui.add_commodity(security, images, window);

            if response.clicked() {
                ui_state.commodity_modal.name = security.name;
                ui_state.commodity_modal.active = true;
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
        let commodity = economy.get_commodity(&ui_state.commodity_modal.name);

        let owned = player
            .commodities
            .iter()
            .filter_map(|s| (s.name == commodity.name).then_some(s.amount))
            .sum::<u32>();

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
                        images.get(commodity.name.to_lowername().as_str()),
                        [window.height() * 0.2; 2],
                    )));
                });

                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        for tab in TradeTab::iter() {
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

                    ui.add_text(
                        format!("Unit price: {:.0}", commodity.current()),
                        window.m_size(),
                    );
                    ui.add_text(
                        format!(
                            "Owned: {owned}          Value: {:.0}",
                            owned as f32 * commodity.current()
                        ),
                        window.m_size(),
                    );

                    ui.horizontal(|ui| {
                        ui.add_text("Quantity:", window.m_size());

                        let amount = ui_state.commodity_modal.amount;
                        ui.spacing_mut().slider_width = window.width() * 0.15;
                        ui.add(
                            Slider::new(
                                &mut ui_state.commodity_modal.amount,
                                0..=((player.cash.current() / commodity.current()) as u32)
                                    .max(owned),
                            )
                            .show_value(false)
                            .text(add_text(amount.to_string(), window.m_size())),
                        );

                        ui.add_space(window.width() * 0.02);

                        ui.add_text(
                            format!(
                                "Total price: {:.0}",
                                commodity.current() * ui_state.commodity_modal.amount as f32
                            ),
                            window.m_size(),
                        );
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
                                        >= commodity.current()
                                            * ui_state.commodity_modal.amount as f32,
                                |ui| {
                                    let button = ui
                                        .add_sized(
                                            [window.width() * 0.08, window.height() * 0.05],
                                            Button::new(add_text("Buy", window.xl_size())),
                                        )
                                        .on_hover(
                                            format!(
                                                "Buy {} units of {}.",
                                                ui_state.commodity_modal.amount,
                                                commodity.name.to_lowername()
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
                                        format!(
                                            "Sell all units of {}.",
                                            commodity.name.to_lowername()
                                        ),
                                        window.m_size(),
                                    )
                                    .on_disabled_hover(
                                        format!("No {} to sell", commodity.name.to_lowername()),
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
                                                "Sell {} units of {}.",
                                                ui_state.commodity_modal.amount,
                                                commodity.name.to_lowername()
                                            ),
                                            window.m_size(),
                                        )
                                        .on_disabled_hover(
                                            format!(
                                                "Not enough units of {} to sell.",
                                                commodity.name.to_lowername(),
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
                        player.cash.amount -=
                            commodity.current() * ui_state.commodity_modal.amount as f32;
                        player.commodities.push(OwnedCommodity {
                            id: create_guid(),
                            name: commodity.name,
                            amount: ui_state.commodity_modal.amount,
                            buy_date: economy.date,
                            buy_price: commodity.current(),
                            warning: false,
                        });

                        messages.write(MessageEv {
                            message: format!(
                                "Bought {} {}.",
                                ui_state.commodity_modal.amount,
                                commodity.name.to_lowername()
                            ),
                            level: MessageLevel::Info,
                        });
                    }

                    if close_clicked {
                        player.cash.amount += commodity.current() * owned as f32;
                        player.commodities.retain(|s| s.name != commodity.name);

                        messages.write(MessageEv {
                            message: format!("Closed {} position.", commodity.name.to_lowername()),
                            level: MessageLevel::Info,
                        });
                    }

                    if sell_clicked {
                        player.cash.amount +=
                            commodity.current() * ui_state.commodity_modal.amount as f32;

                        let mut remaining = ui_state.commodity_modal.amount;
                        player
                            .commodities
                            .iter_mut()
                            .filter(|s| s.name == commodity.name)
                            .sorted_by_key(|s| s.buy_date)
                            .for_each(|s| {
                                let to_deduct = remaining.min(s.amount);
                                s.amount -= to_deduct;
                                remaining -= to_deduct;
                            });

                        player.commodities.retain(|s| s.amount > 0);

                        messages.write(MessageEv {
                            message: format!(
                                "Sold {} {}.",
                                ui_state.commodity_modal.amount,
                                commodity.name.to_lowername()
                            ),
                            level: MessageLevel::Info,
                        });
                    }
                });
            });
        });

        if modal.should_close() {
            ui_state.commodity_modal.active = false;
        }
    }
}
