use std::sync::Arc;

use crate::{
    componects::{
        self,
        friends::{self, state::Friends},
    },
    subscriptions::lockfile::LockFile,
    view::Pages,
    Flags, Message,
};

#[derive(Clone, Debug, Default)]
pub struct State {
    pub input_text: String,
    pub current_user: CurrentUser,
    pub page: Pages,
    pub home_page: HomePages,
    pub lock_file: LockFile,
    pub client: reqwest::Client,
    pub non_secure_client: reqwest::Client,
    pub websocket_status: String,
    pub live_state: LiveState,

    pub friends: friends::State,

    pub tokens: Arc<api::auth::FullAuth>,
    pub valorant_client: api::Client,
    pub store: componects::store::State,
    pub game_content: Arc<GameContent>,
}

impl State {
    pub fn init(flags: Flags) -> (State, iced::Command<Message>) {
        let Flags {
            valorant_client,
            tokens,
            game_content,
        } = flags;
        let client = reqwest::Client::new();
        let non_secure_client = reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        (
            State {
                client: client,
                non_secure_client,
                tokens,
                valorant_client,
                websocket_status: "Disconected".to_string(),
                game_content,
                ..Default::default()
            },
            iced::Command::none(),
        )
    }
}

#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub name: String,
    pub tag: String,
    pub pid: Option<String>,
}

impl Default for CurrentUser {
    fn default() -> Self {
        Self {
            name: "NightHunter".to_string(),
            tag: "000".to_string(),
            pid: None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum HomePages {
    #[default]
    Competitive,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PreGameData {
    pub pre_game_match_id: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum LiveState {
    #[default]
    Menu,
    PreGame(PreGameData),
    InGame,
}

#[derive(Clone, Debug, Default)]
pub struct GameContent {
    pub content_tiers: Vec<api::game_content::content_tiers::ContentTiers>,
    pub themes: Vec<api::game_content::theme::Theme>,
    pub weapon_skins: Vec<api::game_content::weapon_skins::Weapons>,
    pub bundles: Vec<api::game_content::bundles::Bundle>,
}

impl GameContent {
    pub async fn new(client: &api::Client) -> GameContent {
        let weapon_skins = api::game_content::weapon_skins::weapon_skins(&client);
        let themes = api::game_content::theme::get_theme(&client);
        let tiers = api::game_content::content_tiers::content_tiers(&client);
        let bundles = api::game_content::bundles::bundles(&client);
        let (weapon_skins, themes, tiers, bundles) =
            tokio::join!(weapon_skins, themes, tiers, bundles);
        let bundles = bundles.unwrap();
        GameContent {
            weapon_skins: weapon_skins.unwrap(),
            themes: themes.unwrap(),
            content_tiers: tiers.unwrap(),
            bundles: bundles,
        }
    }
}
