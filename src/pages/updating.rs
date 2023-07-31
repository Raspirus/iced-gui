use flume::{self, Receiver};
use iced::widget::{progress_bar, text, Column, Container, Row, Space, button};
use iced::{
    alignment, executor, Alignment, Application, Command, Element, Length, Subscription, Theme,
};
use iced_aw::{Modal, ICON_FONT, Icon};
use log::error;

use crate::backend::utils::Utils;
use crate::components::{modal_widget::DefaultModal, updating_sub::UpdatingSubscription};
use crate::{Message, Page};

#[derive(Debug, Clone)]
pub struct UpdatingPage {
    progress: f32,
    progress_receiver: Option<Receiver<f32>>,
    show_modal: bool,
}

#[derive(Debug, Clone)]
pub enum UpdatingMessage {
    StartUpdating,
    UpdatingProgress(f32),
    UpdatingError(String),
    ResetUpdating,
    CloseModal,
}

impl Application for UpdatingPage {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            UpdatingPage {
                progress: 0.0,
                progress_receiver: None,
                show_modal: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Raspirus | Updating...")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Updating(update_message) => match update_message {
                UpdatingMessage::StartUpdating => {
                    println!("UPDATE SZARTED");
                    let (progress_sender, progress_receiver) = flume::unbounded();
                    self.progress_receiver = Some(progress_receiver);

                    Command::perform(
                        Utils::update_database(Some(progress_sender)),
                        Message::UpdatingFinished,
                    )
                }
                UpdatingMessage::UpdatingProgress(progress) => {
                    // Update the progress value and trigger a UI update
                    self.progress = progress;
                    Command::none()
                }
                UpdatingMessage::UpdatingError(error) => {
                    error!("Error while updating: {}", error);
                    self.show_modal = true;
                    Command::none()
                }
                UpdatingMessage::ResetUpdating => {
                    self.progress = 0.0;
                    self.progress_receiver = None;
                    Command::none()
                }
                UpdatingMessage::CloseModal => {
                    self.show_modal = false;
                    Command::none()
                }
            },
            _ => Command::none(),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        if let Some(receiver) = &self.progress_receiver {
            // Use the actual_receiver if it exists.
            Subscription::from_recipe(UpdatingSubscription::new(receiver.clone()))
        } else {
            // Return an empty subscription if the receiver is None.
            Subscription::none()
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let updating_title = text("Loading...").size(40);

        let update_title = Row::new()
            .push(Space::with_width(10))
            .push(updating_title)
            .align_items(Alignment::Center);

        let prog_bar = progress_bar(0.0..=100.0, self.progress);

        let start_button = button(
            Row::new()
                .push(text(Icon::PlayFill.to_string()).font(ICON_FONT))
                .push(Space::with_width(5))
                .push(
                    text("START"),
                ),
        )
        .on_press(Message::Updating(UpdatingMessage::StartUpdating))
        .padding(10);

        let cancel_button = button(
            Row::new()
                .push(text(Icon::X.to_string()).font(ICON_FONT))
                .push(Space::with_width(5))
                .push(
                    text("CANCEL"),
                ),
        )
        .on_press(Message::ChangePage(Page::Settings, None))
        .padding(10);

        let btn_row = Row::new()
        .push(cancel_button)
        .push(Space::with_width(20))
        .push(start_button)
        .padding(5)
        .align_items(Alignment::Center);

        let content = Column::new()
            .push(update_title)
            .push(prog_bar)
            .push(Space::with_height(10))
            .push(btn_row)
            .padding(10)
            .align_items(alignment::Horizontal::Center.into());

        let container = Container::new(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20);

        Modal::new(self.show_modal, container, || DefaultModal.into())
            .backdrop(Message::Updating(UpdatingMessage::CloseModal))
            .on_esc(Message::Updating(UpdatingMessage::CloseModal))
            .into()
    }
}
