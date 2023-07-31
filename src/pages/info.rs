use iced::widget::{button, scrollable, text, Column, Container, Image, Row, Space};
use iced::{Alignment, Element, Length, Sandbox};
use iced_aw::{Icon, ICON_FONT};
use rust_i18n::t;

use crate::components::information_card::InfoComp;
use crate::{Message, Page};

pub struct InfoPage {
    name: InfoComp,
    description: InfoComp,
    maintainers: InfoComp,
    version: InfoComp,
    license: InfoComp,
}

impl Sandbox for InfoPage {
    type Message = Message;

    fn new() -> Self {
        InfoPage {
            name: InfoComp::new(t!("app_name").to_string(), "Raspirus".to_string(), Icon::Box),
            description: InfoComp::new(
                t!("description").to_string(),
                t!("description_val").to_string(),
                Icon::Book,
            ),
            maintainers: InfoComp::new(
                t!("maintainers").to_string(),
                t!("maintainers_val").to_string(),
                Icon::Person,
            ),
            version: InfoComp::new(
                t!("version").to_string(),
                env!("CARGO_PKG_VERSION").to_string(),
                Icon::InfoCircle,
            ),

            license: InfoComp::new(
                t!("license").to_string(),
                t!("license_val").to_string(),
                Icon::Flag,
            ),
        }
    }

    fn title(&self) -> String {
        String::from("Raspirus | Info")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            _ => {}
        }
    }

    fn view(&self) -> Element<Self::Message> {
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

        let info_title = text(t!("info_title")).size(40);

        // 1856 x 1024
        let banner_image = Image::new("assets/images/banner.png")
            .width(928)
            .height(512);

        let info_comps = Column::new()
            .push(self.name.view())
            .push(self.description.view())
            .push(self.maintainers.view())
            .push(self.version.view())
            .push(self.license.view());

        let title = Row::new()
            .push(back_button)
            .push(Space::with_width(10))
            .push(info_title)
            .align_items(Alignment::Start);

        let content = scrollable(
            Column::new()
                .spacing(20)
                .push(title)
                .push(banner_image)
                .push(info_comps)
                .width(Length::Fill),
        );

        Container::new(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}
