use iced::{executor, window::icon, Application, Command, Subscription, Theme};
use log::error;
use log::{info, warn};
rust_i18n::i18n!("locales");

/*
This file connects all the interactions between pages or components.
Adding a new page also needs to be added here. Same goes for its messages.
Basically when run, it redirects the user to the home page. When the user then triggers a message
there, it passes back through this file, then to the Messages section, which again redirects the Message
back to the Home page. This is necessary because it allows us to have a centralized pages management.
*/

mod pages;
use pages::{
    clean::CleanPage,
    home::{HomeMessage, HomePage},
    infected::{InfectedMessage, InfectedPage},
    info::InfoPage,
    loading::{LoadingMessage, LoadingPage},
    settings::{SettingsMessage, SettingsPage},
    updating::{UpdatingMessage, UpdatingPage},
};

pub mod backend;
pub mod components;

/// Represents different pages of the application.
/// Will be used to change from one page to another using Messages
#[derive(Debug, Clone, Copy)]
pub enum Page {
    Home,
    Settings,
    Info,
    Loading,
    Clean,
    Infected,
    Updating,
}

#[derive(Debug, Clone)]
pub enum Param {
    String(String),
    Vector(Vec<String>),
}

/// The main structure of the application. It contains an instance of all the pages.
/// Furthermore it also contains a reference to the current page.
/// Having each instance separate, lets us control each page-specific message individually
pub struct Raspirus {
    page: Page,
    home_page: HomePage,
    info_page: InfoPage,
    settings_page: SettingsPage,
    loading_page: LoadingPage,
    clean_page: CleanPage,
    infected_page: InfectedPage,
    updating_page: UpdatingPage,
}

/// Contains the messages used for changing from one page to another.
/// Furthermore it also contains a reference to the various message of each page
/// This is needed because the Message type() of each page must remain the same and therefore needs to
/// reference this Message specifically.
#[derive(Debug, Clone)]
pub enum Message {
    ChangePage(Page, Option<Param>),
    Home(HomeMessage),
    Settings(SettingsMessage),
    Loading(LoadingMessage),
    Infected(InfectedMessage),
    Updating(UpdatingMessage),
    ScanningFinished(Result<Vec<String>, String>),
    UpdatingFinished(Result<String, String>),
}

/// The main implementation of the application
/// The application is not using Iceds subscription system.
impl Application for Raspirus {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    /// Creates a new Raspirus object that contains an instance of each page. This is necessary for the borrowing to function.
    /// It also allows us to change from one page to another trough this page.
    /// This may cause the application to start slowly, as it needs to load all pages first.
    /// If one page fails, the application also fails
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Raspirus {
                page: Page::Home,
                home_page: HomePage::new(()).0,
                info_page: InfoPage::new(()).0,
                settings_page: SettingsPage::new(()).0,
                loading_page: LoadingPage::new(()).0,
                clean_page: CleanPage::new(()).0,
                infected_page: InfectedPage::new(()).0,
                updating_page: UpdatingPage::new(()).0,
            },
            Command::none(),
        )
    }

    /// This sets the title of each page. It doesn't matter what title you specify in the page directly.
    /// That value will be overwritten by the value set here.
    fn title(&self) -> String {
        match self.page {
            Page::Home => String::from("Raspirus | Home"),
            Page::Settings => String::from("Raspirus | Settings"),
            Page::Info => String::from("Raspirus | Info"),
            Page::Loading => String::from("Raspirus | Loading..."),
            Page::Clean => String::from("Raspirus | Clean"),
            Page::Infected => String::from("Raspirus | Virus found!"),
            Page::Updating => String::from("Raspirus | Updating..."),
        }
    }

    /// This collects all Messages from all pages and then redirects them to the various pages to be handled there.
    /// This is the reason why we always import both the page and its Messages.
    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            // The params here is completely optional and is only used to transfer the select scan location
            // to the next page, where the loading starts and the scanner is initialized with the given location
            Message::ChangePage(page, params) => {
                self.page = page;
                if let Some(params) = params {
                    match params {
                        Param::String(scan_path) => {
                            return self
                                .loading_page
                                .update(Message::Loading(LoadingMessage::SetPath(scan_path)));
                        }
                        Param::Vector(scan_result) => {
                            return self.infected_page.update(Message::Infected(
                                InfectedMessage::SetScanResult(scan_result),
                            ));
                        }
                    }
                }
                Command::none()
            }
            Message::Home(home_message) => self.home_page.update(Message::Home(home_message)),
            Message::Settings(settings_message) => self
                .settings_page
                .update(Message::Settings(settings_message)),
            Message::Loading(loading_message) => {
                self.loading_page.update(Message::Loading(loading_message))
            }
            Message::Infected(infected_message) => self
                .infected_page
                .update(Message::Infected(infected_message)),
            Message::Updating(update_message) => {
                self.updating_page.update(Message::Updating(update_message))
            }
            Message::ScanningFinished(Ok(result)) => {
                info!("Scanning successfull: {:?}", result);
                // Assigning to an unused variable to avoid the warning: #[warn(unused_must_use)]
                let _ = self.update(Message::Loading(LoadingMessage::ResetScan));
                if result.is_empty() {
                    self.update(Message::ChangePage(Page::Clean, None))
                } else {
                    self.update(Message::ChangePage(
                        Page::Infected,
                        Some(Param::Vector(result)),
                    ))
                }
            }
            Message::ScanningFinished(Err(error)) => {
                error!("Scanning error: {}", error);
                let _ = self.update(Message::Loading(LoadingMessage::ResetScan));
                self.update(Message::Loading(LoadingMessage::ScanError(error)))
            }
            Message::UpdatingFinished(Ok(result)) => {
                info!("Update successfull: {}", result);
                let _ = self.update(Message::Updating(UpdatingMessage::ResetUpdating));
                self.update(Message::ChangePage(Page::Settings, None))
            }
            Message::UpdatingFinished(Err(error)) => {
                error!("Update error: {}", error);
                let _ = self.update(Message::Updating(UpdatingMessage::ResetUpdating));
                self.update(Message::Updating(UpdatingMessage::UpdatingError(error)))
            }
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        match self.page {
            Page::Loading => self.loading_page.subscription(),
            Page::Updating => self.updating_page.subscription(),
            _ => Subscription::none(),
        }
    }

    /// This function loads the view of the currently set page. This allows us to have the page management
    /// centralized. If you want to add a page, you also need to add its view() function here, else it won't load.
    fn view(&self) -> iced::Element<'_, Self::Message> {
        match self.page {
            Page::Home => self.home_page.view(),
            Page::Settings => self.settings_page.view(),
            Page::Info => self.info_page.view(),
            Page::Loading => self.loading_page.view(),
            Page::Clean => self.clean_page.view(),
            Page::Infected => self.infected_page.view(),
            Page::Updating => self.updating_page.view(),
        }
    }
}

fn main() {
    // Initializes the logger for the entire application. Useful when in dev-mode
    match pretty_env_logger::try_init() {
        Ok(()) => {
            info!("Logger initialized!");
        }
        Err(err) => {
            warn!("Failed initializing logger: {err}");
        }
    }

    let icon = icon::from_file("assets/icons/icon.png");
    let settings = iced::settings::Settings {
        window: iced::window::Settings {
            icon: Some(icon.unwrap()),
            ..Default::default()
        },
        ..Default::default()
    };

    Raspirus::run(settings).expect("Init error");
}
