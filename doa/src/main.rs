// Bring in iced for window management.
use iced::widget::{center, svg};
use iced::{Element, Length, Size};

pub fn main() -> iced::Result {
    // Set window sizes
    let x = 1000.0;
    let y = 600.0;

    iced::application(App::default, App::update, App::view)
        .title("Doa - Chat, Better")
        .window_size(Size::new(x, y))
        .run()
}

#[derive(Default)]
struct App;

#[derive(Debug, Clone)]
enum Message {}

impl App {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        let handle = svg::Handle::from_memory(include_bytes!("assets/logo.svg"));
        let logo_size = 125.0;

        let logo = svg(handle)
            .width(Length::Fixed(logo_size))
            .height(Length::Fixed(logo_size));

        center(logo).into()
    }
}
