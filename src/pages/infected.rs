use std::path::Path;
use iced::widget::{button, scrollable, text, Column, Container, Image, Row, Space};
use iced::{Alignment, Element, Length, Sandbox};
use iced_aw::{Icon, Modal, ICON_FONT};
use log::{error, debug};
use rust_i18n::t;

use crate::backend::config_file::Config;
use crate::components::modal_widget::DefaultModal;
use crate::components::virus_card::VirusComp;
use crate::{Message, Page};

#[derive(Debug, Clone)]
pub struct InfectedPage {
    virus_list: Vec<VirusComp>,
    show_modal: bool,
}

#[derive(Debug, Clone)]
pub enum InfectedMessage {
    SetScanResult(Vec<String>),
    CloseModal,
}

impl Sandbox for InfectedPage {
    type Message = Message;

    fn new() -> Self {
        InfectedPage {
            virus_list: Vec::new(),
            show_modal: false,
        }
    }

    fn title(&self) -> String {
        String::from("Raspirus | Infected")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Infected(infected_message) => match infected_message {
                InfectedMessage::SetScanResult(scan_res) => {
                    if !scan_res.is_empty() {
                        for entry in &scan_res {
                            let title: &str;
                            if let Some(file_name) = Path::new(entry)
                                .file_name()
                                .and_then(std::ffi::OsStr::to_str)
                            {
                                title = file_name;
                            } else {
                                title = entry;
                            }
                            let text = entry;

                            // Create the VirusComp instance and push it to the Column
                            let virus_comp = VirusComp::new(
                                title.to_string(),
                                text.to_string(),
                                Icon::ExclamationTriangle,
                            )
                            .clone();
                            self.virus_list.push(virus_comp);
                        }
                    } else {
                        // Output the error message
                        self.show_modal = true;
                        error!("Virus list is empty");
                    }
                }
                InfectedMessage::CloseModal => {
                    self.show_modal = false;
                }
            },
            _ => {}
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let mut config = Config::new();
        config = config.load().expect("Error loading config");

        /* OBFUSCATED MODE COMPONENTS */
        let title = text(t!("infected_title")).size(50);

        let image = Image::new("assets/images/failure_image.png")
            .width(500)
            .height(500);

        let back_button = button(
            Row::new()
                .push(text(Icon::House.to_string()).font(ICON_FONT))
                .push(Space::with_width(5))
                .push(text(t!("home_btn"))),
        )
        .on_press(Message::ChangePage(Page::Home, None))
        .padding(10);

        /* REGULAR MODE COMPONENTS */
        let reg_back_button = button(
            Row::new()
                .push(text(Icon::House.to_string()).font(ICON_FONT))
                .push(Space::with_width(5))
                .push(text(t!("home_btn"))),
        )
        .on_press(Message::ChangePage(Page::Home, None))
        .padding(10);

        let infected_title = text(t!("infected_title")).size(40);

        let reg_title = Row::new()
            .push(reg_back_button)
            .push(Space::with_width(10))
            .push(infected_title)
            .align_items(Alignment::Start);

        let mut virus_comp = Vec::new();
        for comp in &self.virus_list {
            virus_comp.push(comp.view());
        }
        let infected_comps = Column::with_children(virus_comp);

        // Coonditional rendering
        if config.obfuscated_is_active {
            debug!("OBFUSCATED IS ACTIVE");
            let content = Column::new()
                .push(title)
                .push(image)
                .push(back_button)
                .align_items(Alignment::Center)
                .padding(10);

            let container = Container::new(content)
                .center_x()
                .center_y()
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(20);

            Modal::new(self.show_modal, container, || DefaultModal.into())
                .backdrop(Message::Infected(InfectedMessage::CloseModal))
                .on_esc(Message::Infected(InfectedMessage::CloseModal))
                .into()
        } else {
            debug!("OBFUSCATED NOT ACTIVE");
            let content = scrollable(
                Column::new()
                    .spacing(20)
                    .push(reg_title)
                    .push(infected_comps)
                    .width(Length::Fill),
            );

            let container = Container::new(content)
                .center_x()
                .center_y()
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(20);

            Modal::new(self.show_modal, container, || DefaultModal.into())
                .backdrop(Message::Infected(InfectedMessage::CloseModal))
                .on_esc(Message::Infected(InfectedMessage::CloseModal))
                .into()
        }
    }
}
