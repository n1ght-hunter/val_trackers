#![windows_subsystem = "windows"]


pub mod auth;
pub mod componects;
pub mod helpers;
pub mod pages;
pub mod state;
pub mod subscriptions;
pub mod theme;
pub mod update;
pub mod view;


use iced::{executor, Application, Command, Settings};

pub use state::State;

pub use update::Message;

pub type Element<'a> = iced::Element<'a, Message, iced::Renderer<theme::Theme>>;

fn main() {
    dotenv::dotenv().ok();

    state::State::run(Settings::default()).unwrap();
}

impl Application for state::State {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = theme::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        state::State::init()
    }

    fn title(&self) -> String {
        String::from("Video Player")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        subscriptions::subscription(self)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        update::update(self, message)
    }

    fn view(&self) -> Element {
        view::view(self)
    }
}
