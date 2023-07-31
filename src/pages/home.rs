use iced::widget::{button, pick_list, text, Column, Container, Row, Space};
use iced::{alignment, Alignment, Element, Length, Sandbox};
use iced_aw::{Icon, Modal, ICON_FONT};
use log::debug;
use rfd::FileDialog;
use rust_i18n::t;

use crate::backend::utils::{UsbDevice, Utils};
use crate::components::languages::Language;
use crate::components::modal_widget::HomeModal;
use crate::{Message, Page, Param};

/*
TODO:
- Change the language buttons to an actual selectable list
*/

pub struct HomePage {
    is_raspberry_pi: bool,
    selected_language: Language,
    drive_path: Option<String>,
    drives_list: Option<Vec<UsbDevice>>,
    show_modal: bool,
}

#[derive(Debug, Clone)]
pub enum HomeMessage {
    LanguageSelected(Language),
    RefreshPressed,
    DriveSelected(String),
    FolderSelected,
    ShowModal,
    CloseModal,
}

impl Sandbox for HomePage {
    type Message = Message;

    fn new() -> Self {
        let usbs = match Utils::list_usb_drives() {
            Ok(res) => Some(res),
            Err(_) => None,
        };

        let arch = std::env::consts::ARCH;

        HomePage {
            is_raspberry_pi: arch == "arm",
            selected_language: Language::English,
            drive_path: None,
            drives_list: usbs,
            show_modal: false,
        }
    }

    fn title(&self) -> String {
        String::from("Raspirus | Main")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Home(home_message) => match home_message {
                HomeMessage::DriveSelected(path) => {
                    debug!("User selected drive: {}", path);
                    self.drive_path = Some(path);
                }
                HomeMessage::FolderSelected => {
                    if let Some(folder_path) =
                        FileDialog::new().add_filter("Folders", &[""]).pick_folder()
                    {
                        let f_path: String = folder_path.into_os_string().into_string().unwrap();
                        self.drive_path = Some(f_path);
                    }
                }
                HomeMessage::RefreshPressed => {
                    debug!("Refreshing HomePage");
                    let usbs = match Utils::list_usb_drives() {
                        Ok(res) => Some(res),
                        Err(_) => None,
                    };
                    self.drives_list = usbs;
                }
                HomeMessage::LanguageSelected(language) => {
                    self.selected_language = language;
                    rust_i18n::set_locale(language.key());
                }
                HomeMessage::ShowModal => {
                    self.show_modal = true;
                }
                HomeMessage::CloseModal => {
                    self.show_modal = false;
                }
            },
            _ => {}
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let titel_label = text("RASPIRUS")
            .size(150)
            .horizontal_alignment(alignment::Horizontal::Center);
        let settings_btn = button(
            Row::new()
                .push(text(Icon::Gear.to_string()).font(ICON_FONT))
                .push(Space::with_width(5))
                .push(text(t!("settings"))),
        )
        .on_press(Message::ChangePage(Page::Settings, None));
        let info_btn = button(text(t!("info"))).on_press(Message::ChangePage(Page::Info, None));
        let start_btn = button(text(t!("start"))).on_press(match self.drive_path.clone() {
            Some(path) => Message::ChangePage(Page::Loading, Some(Param::String(path))),
            None => Message::Home(HomeMessage::ShowModal),
        });
        let refresh_btn = button(text(Icon::ArrowClockwise).font(ICON_FONT))
            .on_press(Message::Home(HomeMessage::RefreshPressed));
        let folder_btn = button(text(Icon::Folder.to_string()).font(ICON_FONT))
            .on_press(Message::Home(HomeMessage::FolderSelected));

        let language_picker = Column::new()
            .push(
                button(
                    Row::new()
                        .push(Language::English.icon())
                        .push(Space::with_width(3))
                        .push(text(Language::English.to_string()))
                        .align_items(alignment::Vertical::Center.into()),
                )
                .on_press(Message::Home(HomeMessage::LanguageSelected(
                    Language::English,
                ))),
            )
            .push(
                button(
                    Row::new()
                        .push(Language::Italian.icon())
                        .push(Space::with_width(3))
                        .push(text(Language::Italian.to_string()))
                        .align_items(alignment::Vertical::Center.into()),
                )
                .on_press(Message::Home(HomeMessage::LanguageSelected(
                    Language::Italian,
                ))),
            )
            .push(
                button(
                    Row::new()
                        .push(Language::German.icon())
                        .push(Space::with_width(3))
                        .push(text(Language::German.to_string()))
                        .align_items(alignment::Vertical::Center.into()),
                )
                .on_press(Message::Home(HomeMessage::LanguageSelected(
                    Language::German,
                ))),
            )
            .spacing(5)
            .align_items(Alignment::Start)
            .padding(5);

        let drive_picker = pick_list(
            self.drives_list
                .as_ref()
                .map(|devices| {
                    devices
                        .iter()
                        .map(|device| device.name.clone())
                        .collect::<Vec<_>>()
                })
                .map(|names| names.iter().map(|name| name.clone()).collect::<Vec<_>>())
                .unwrap_or_else(Vec::new),
            self.drive_path.as_ref().map(|device| device.clone()),
            |selected: String| {
                if let Some(device) = self
                    .drives_list
                    .as_ref()
                    .and_then(|devices| devices.iter().find(|device| device.name == selected))
                {
                    Message::Home(HomeMessage::DriveSelected(device.path.clone()))
                } else {
                    // Handle the case when no device is found for the selected name
                    unimplemented!("Handle the case when the selected device is not found.")
                }
            },
        );

        let top_row = Row::new()
            .push(language_picker)
            .push(Space::with_width(Length::Fill))
            .push(settings_btn)
            .padding(10);

        let picker_row;
        if self.is_raspberry_pi {
            picker_row = Row::new()
                .push(drive_picker)
                .push(Space::with_width(5))
                .push(refresh_btn)
                .padding(10);
        } else {
            picker_row = Row::new()
                .push(drive_picker)
                .push(Space::with_width(5))
                .push(folder_btn)
                .push(Space::with_width(2))
                .push(refresh_btn)
                .padding(10);
        }

        let btn_row = Row::new()
            .push(info_btn)
            .push(Space::with_width(10))
            .push(start_btn)
            .padding(5);

        let central_col = Column::new()
            .push(titel_label)
            .push(picker_row)
            .push(btn_row)
            .align_items(Alignment::Center);

        let content = Column::new()
            .push(top_row)
            .push(Space::with_height(Length::FillPortion(2)))
            .push(central_col)
            .push(Space::with_height(Length::FillPortion(2)))
            .align_items(Alignment::Center);

        let container = Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        Modal::new(self.show_modal, container, || HomeModal.into())
            .backdrop(Message::Home(HomeMessage::CloseModal))
            .on_esc(Message::Home(HomeMessage::CloseModal))
            .into()
    }
}
