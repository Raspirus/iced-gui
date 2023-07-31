use iced::widget::{button, text, Column, Image, Row, Space};
use iced::{Alignment, Element, Length, Sandbox};
use iced_aw::{Icon, ICON_FONT};
use rust_i18n::t;

use crate::{Message, Page};

/// The struct is empty, as this page doesn't require any page-specific values or Messages
#[derive(Debug, Clone)]
pub struct CleanPage {}

/// # Clean page
/// A page that displays a green check if the app found no viruses
///
/// ## Structure
/// In the top left corner there is a title showing on which page the user currently is.
/// In the center of the page there is an image showing a green tick
/// Right below it there is a button that allows the user to return home
///
/// ## Actions
/// User can return home by clicking a button
impl Sandbox for CleanPage {
    type Message = Message;

    fn new() -> Self {
        CleanPage {}
    }

    fn title(&self) -> String {
        String::from("Raspirus | Clean")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        let title = text(t!("clean_title")).size(50);

        let image = Image::new("assets/images/success_image.png")
            .width(500)
            .height(500);

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

        Column::new()
            .push(title)
            .push(image)
            .push(back_button)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .padding(10)
            .into()
    }
}
