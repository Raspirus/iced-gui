use iced::widget::{text, Column, Container, Row, Space};
use iced::{alignment, Element};
use iced_aw::{Icon, ICON_FONT};

use crate::Message;

pub struct InfoComp {
    title: String,
    value: String,
    icon: Icon,
}

impl InfoComp {
    pub fn new(title: String, value: String, icon: Icon) -> Self {
        Self { title, value, icon }
    }

    pub fn view(&self) -> Element<Message> {
        let icon = text(self.icon.clone().to_string())
            .font(ICON_FONT)
            .size(64)
            .height(72)
            .height(72)
            .vertical_alignment(alignment::Vertical::Center);

        let title = text(&self.title).size(20);
        let value = text(&self.value).size(14);

        let text_col = Column::new()
            .push(title)
            .push(Space::with_height(5))
            .push(value);

        let content = Row::new()
            .push(icon)
            .push(Space::with_width(10))
            .push(text_col)
            .align_items(alignment::Alignment::Center);

        Container::new(content).padding(10).into()
    }
}
