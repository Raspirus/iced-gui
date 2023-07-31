use iced::{widget::text, Element, Length};
use iced_aw::Card;
use iced_lazy::{self, Component};

pub struct DefaultModal;
pub struct HomeModal;

impl<Message> Component<Message, iced::Renderer> for DefaultModal {
    type State = ();
    type Event = ();

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<Message> {
        None
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, iced::Renderer> {
        Card::new(
            text("Unexpected Error").width(Length::Fill),
            text("This was unexpected! Please report it on GitHub"),
        )
        .max_width(300.0)
        .into()
    }
}

impl<'a, Message> From<DefaultModal> for Element<'a, Message, iced::Renderer>
where
    Message: 'a,
{
    fn from(my_component: DefaultModal) -> Self {
        iced_lazy::component(my_component)
    }
}

// HOME PAGE
impl<Message> Component<Message, iced::Renderer> for HomeModal {
    type State = ();
    type Event = ();

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<Message> {
        None
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, iced::Renderer> {
        Card::new(
            text("Error").width(Length::Fill),
            text("No scan location selected"),
        )
        .max_width(300.0)
        .into()
    }
}

impl<'a, Message> From<HomeModal> for Element<'a, Message, iced::Renderer>
where
    Message: 'a,
{
    fn from(my_component: HomeModal) -> Self {
        iced_lazy::component(my_component)
    }
}
