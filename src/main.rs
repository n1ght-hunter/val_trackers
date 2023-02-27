pub mod auth;
pub mod componects;
pub mod helpers;
pub mod pages;
pub mod state;
pub mod subscriptions;
pub mod theme;
pub mod update;
pub mod view;

use std::sync::Arc;

use iced::{executor, Application, Command, Settings};

use state::GameContent;
pub use state::State;

pub use update::Message;

pub type Element<'a> = iced::Element<'a, Message, iced::Renderer<theme::Theme>>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    helpers::check_riot_client::check_riot_client();
    let valorant_client = api::create_client().unwrap();
    let tokens = api::auth::full_auth(&valorant_client);
    let game_content = state::GameContent::new(&valorant_client);

    let (tokens, game_content) = tokio::join!(tokens, game_content);

    state::State::run(Settings {
        flags: Flags {
            valorant_client,
            tokens: Arc::new(tokens),
            game_content: Arc::new(game_content),
        },
        ..Settings::default()
    })
    .unwrap();
}

#[derive(Default)]
pub struct Flags {
    valorant_client: api::Client,
    tokens: Arc<api::auth::FullAuth>,
    game_content: Arc<GameContent>,
}

impl Application for state::State {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = theme::Theme;

    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        state::State::init(flags)
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
