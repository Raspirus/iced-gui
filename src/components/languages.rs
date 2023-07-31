use iced::widget::Image;

// Define a struct to hold language information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Italian,
    German,
}

impl Language {
    pub const ALL: [Language; 3] = [Language::English, Language::Italian, Language::German];

    pub fn icon(&self) -> Image {
        let flag_size = 16;

        match self {
            Language::English => Image::new("assets/images/flags/united-kingdom-flag-icon-64.png")
                .height(flag_size)
                .width(flag_size),
            Language::Italian => Image::new("assets/images/flags/italy-flag-icon-64.png")
                .height(flag_size)
                .width(flag_size),
            Language::German => Image::new("assets/images/flags/germany-flag-icon-64.png")
                .height(flag_size)
                .width(flag_size),
        }
    }

    pub fn key(&self) -> &str {
        match self {
            Language::English => "en",
            Language::Italian => "it",
            Language::German => "de",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::English => "English",
                Language::German => "German",
                Language::Italian => "Italian",
            }
        )
    }
}
