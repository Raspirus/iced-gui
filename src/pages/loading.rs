use flume::{self, Receiver};
use iced::widget::{button, progress_bar, scrollable, text, Column, Container, Row, Space};
use iced::{
    alignment, executor, Alignment, Application, Command, Element, Length, Subscription, Theme,
};
use iced_aw::{Icon, Modal, ICON_FONT};
use log::error;
use rust_i18n::t;

use crate::backend::utils::Utils;
use crate::components::modal_widget::DefaultModal;
use crate::components::progress_sub::ProgressSubscription;
use crate::{Message, Page};

#[derive(Debug, Clone)]
pub struct LoadingPage {
    confirmed: bool,
    scan_path: String,
    progress: f32,
    progress_receiver: Option<Receiver<f32>>,
    show_modal: bool,
}

#[derive(Debug, Clone)]
pub enum LoadingMessage {
    StartScanner,
    SetPath(String),
    UpdateProgress(f32),
    ScanError(String),
    ResetScan,
    CloseModal,
}

impl Application for LoadingPage {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            LoadingPage {
                confirmed: false,
                scan_path: String::from(""),
                progress: 0.0,
                progress_receiver: None,
                show_modal: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Raspirus | Loading")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Loading(loading_message) => match loading_message {
                LoadingMessage::StartScanner => {
                    self.confirmed = true;
                    let scan_path = self.scan_path.clone();
                    let (progress_sender, progress_receiver) = flume::unbounded();

                    self.progress_receiver = Some(progress_receiver);

                    Command::perform(
                        Utils::start_scanner(scan_path, Some(progress_sender)),
                        Message::ScanningFinished,
                    )
                }
                LoadingMessage::SetPath(scan_path) => {
                    self.scan_path = scan_path;
                    Command::none()
                }
                LoadingMessage::UpdateProgress(progress) => {
                    // Update the progress value and trigger a UI update
                    self.progress = progress;
                    Command::none()
                }
                LoadingMessage::ScanError(error) => {
                    error!("Error while loading: {}", error);
                    self.show_modal = true;
                    Command::none()
                }
                LoadingMessage::ResetScan => {
                    self.confirmed = false;
                    self.progress = 0.0;
                    self.progress_receiver = None;
                    Command::none()
                }
                LoadingMessage::CloseModal => {
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
            Subscription::from_recipe(ProgressSubscription::new(receiver.clone()))
        } else {
            // Return an empty subscription if the receiver is None.
            Subscription::none()
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let perms_title = text(t!("permissions_title")).size(40);
        let loading_title = text(t!("loading_title")).size(40);

        let perms_text =
            scrollable(text(t!("permissions_text"))).height(Length::Fill);

        let confirm_button = button(
            Row::new()
                .push(text(Icon::Check2.to_string()).font(ICON_FONT))
                .push(Space::with_width(5))
                .push(
                    text(t!("perms_accept")), // Add your custom text here
                ),
        )
        .on_press(Message::Loading(LoadingMessage::StartScanner))
        .padding(10);

        let cancel_button = button(
            Row::new()
                .push(text(Icon::X.to_string()).font(ICON_FONT))
                .push(Space::with_width(5))
                .push(
                    text(t!("perms_decline")), // Add your custom text here
                ),
        )
        .on_press(Message::ChangePage(Page::Home, None))
        .padding(10);

        let btn_row = Row::new()
            .push(cancel_button)
            .push(Space::with_width(20))
            .push(confirm_button)
            .padding(5)
            .align_items(Alignment::Center);

        let perm_title = Row::new()
            .push(Space::with_width(10))
            .push(perms_title)
            .align_items(Alignment::Center);

        let load_title = Row::new()
            .push(Space::with_width(10))
            .push(loading_title)
            .align_items(Alignment::Center);

        let prog_bar = progress_bar(0.0..=100.0, self.progress);

        let mut content = Column::new();

        if self.confirmed {
            content = content
                .push(load_title)
                .push(prog_bar)
                .padding(10)
                .align_items(alignment::Horizontal::Center.into());
        } else {
            content = content
                .push(perm_title)
                .push(perms_text)
                .push(btn_row)
                .padding(10)
                .align_items(alignment::Horizontal::Center.into());
        }

        let container = Container::new(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20);

        Modal::new(self.show_modal, container, || DefaultModal.into())
            .backdrop(Message::Loading(LoadingMessage::CloseModal))
            .on_esc(Message::Loading(LoadingMessage::CloseModal))
            .into()
    }
}
