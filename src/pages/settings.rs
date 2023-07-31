use iced::widget::{button, text, Column, Container, Row, Space};
use iced::{alignment, Alignment, Element, Length, Sandbox};
use iced_aw::{Icon, ICON_FONT};
use rust_i18n::t;

use crate::backend::config_file::Config;
use crate::{Message, Page};

pub struct SettingsPage {
    config: Config,
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    AutoUpdateTimeSet(String),
    AutoUpdateWeekSet(i32),
    LoggingToggle,
    ObfuscatedToggle,
}

impl Sandbox for SettingsPage {
    type Message = Message;

    fn new() -> Self {
        let config = Config::new();
        config
            .set_path()
            .expect("Error while setting path of config");

        SettingsPage {
            config: config
                .load()
                .expect("Failed to load config in settings page"),
        }
    }

    fn title(&self) -> String {
        String::from("Raspirus | Settings")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Settings(settings_message) => match settings_message {
                SettingsMessage::AutoUpdateTimeSet(update_time) => {
                    self.config.db_update_time = update_time;
                    self.config.save().expect("Error while saving config");
                }
                SettingsMessage::AutoUpdateWeekSet(update_week) => {
                    self.config.db_update_weekday = update_week;
                    self.config.save().expect("Error while saving config");
                }
                SettingsMessage::LoggingToggle => {
                    self.config.logging_is_active = !self.config.logging_is_active;
                    self.config.save().expect("Error while saving config");
                }
                SettingsMessage::ObfuscatedToggle => {
                    self.config.obfuscated_is_active = !self.config.obfuscated_is_active;
                    self.config.save().expect("Error while saving config");
                }
            },
            _ => {}
        }
    }

    fn view(&self) -> Element<Self::Message> {
        self.config
            .set_path()
            .expect("Error while setting path of config");
        self.config.load().expect("Error while loading config");

        let back_button = button(
            Row::new()
                .push(text(Icon::House.to_string()).font(ICON_FONT))
                .push(Space::with_width(5))
                .push(
                    text(t!("home_btn")), // Add your custom text here
                ),
        )
        .on_press(Message::ChangePage(Page::Home, None))
        .padding(10);

        let info_title = text(t!("settings_title")).size(40);

        let title = Row::new()
            .push(back_button)
            .push(Space::with_width(10))
            .push(info_title)
            .align_items(Alignment::Start);

        // UPDATE COMPONENT
        let update_comp = Row::new()
            .push(
                text(Icon::Wrench.to_string())
                    .font(ICON_FONT)
                    .size(64)
                    .height(72)
                    .height(72)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(Space::with_width(Length::FillPortion(2)))
            .push(
                Column::new()
                    .push(text(t!("update_db")).size(30))
                    .push(Space::with_height(5))
                    .push(text(t!("update_db_val")).size(20))
                    .push(Space::with_height(5))
                    .push(
                        text(format!(
                            "{}: {} | {}: {}",
                            t!("update_db_1"),
                            self.config.hashes_in_db, 
                            self.config.last_db_update,
                            t!("update_db_2")
                        ))
                        .size(14),
                    ),
            )
            .push(Space::with_width(Length::FillPortion(2)))
            .push(button(text(t!("update_db_btn"))).on_press(Message::ChangePage(Page::Updating, None)))
            .align_items(alignment::Alignment::Center)
            .padding([20, 200]);

        // LOGGING COMPONENT
        let logging_comp = Row::new()
            .push(
                text(Icon::Book.to_string())
                    .font(ICON_FONT)
                    .size(64)
                    .height(72)
                    .height(72)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(Space::with_width(Length::FillPortion(2)))
            .push(
                Column::new()
                    .push(text(t!("activate_logs")).size(30))
                    .push(Space::with_height(5))
                    .push(text(t!("activate_logs_val")).size(20)),
            )
            .push(Space::with_width(Length::FillPortion(2)))
            .push(
                button(if self.config.logging_is_active {
                    text(t!("settings_on"))
                } else {
                    text(t!("settings_off"))
                })
                .on_press(Message::Settings(SettingsMessage::LoggingToggle)),
            )
            .align_items(alignment::Alignment::Center)
            .padding([20, 200]);

        // OBFUSCATION COMPONENT
        let obfuscation_comp = Row::new()
            .push(
                text(Icon::EyeSlash.to_string())
                    .font(ICON_FONT)
                    .size(64)
                    .height(72)
                    .height(72)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(Space::with_width(Length::FillPortion(2)))
            .push(
                Column::new()
                    .push(text(t!("obfuscated_mode")).size(30))
                    .push(Space::with_height(5))
                    .push(text(t!("obfuscated_mode_val")).size(20)),
            )
            .push(Space::with_width(Length::FillPortion(2)))
            .push(
                button(if self.config.obfuscated_is_active {
                    text(t!("settings_on"))
                } else {
                    text(t!("settings_off"))
                })
                .on_press(Message::Settings(SettingsMessage::ObfuscatedToggle)),
            )
            .align_items(alignment::Alignment::Center)
            .padding([20, 200]);

        // UPDATE SCHEDULER COMPONENT
        let scheduler_comp =
            Row::new()
                .push(
                    text(Icon::Clock.to_string())
                        .font(ICON_FONT)
                        .size(64)
                        .height(72)
                        .height(72)
                        .vertical_alignment(alignment::Vertical::Center),
                )
                .push(Space::with_width(Length::FillPortion(2)))
                .push(
                    Column::new()
                        .push(text(t!("auto_db")).size(30))
                        .push(Space::with_height(5))
                        .push(text(t!("auto_db_val")).size(20)),
                )
                .push(Space::with_width(Length::FillPortion(2)))
                .push(button(text(t!("auto_db_btn"))).on_press(Message::Settings(
                    SettingsMessage::AutoUpdateTimeSet("20:00".to_string()),
                )))
                .align_items(alignment::Alignment::Center)
                .padding([20, 200]);

        let setting_comps = Column::new()
            .push(update_comp)
            .push(logging_comp)
            .push(obfuscation_comp)
            .push(scheduler_comp)
            .align_items(Alignment::Center)
            .height(Length::Fill);

        let content = Column::new()
            .spacing(20)
            .push(title)
            .push(setting_comps)
            .width(Length::Fill);

        Container::new(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}
