mod auth_cookies;
mod auth_request;
mod entitlements;

pub use auth_cookies::*;
pub use auth_request::*;
pub use entitlements::*;
use reqwest_impersonate::Client;

use crate::val_api::{PlayerInfoSmall, get_player_info};

#[derive(Debug, Clone, Default)]
pub struct FullAuth {
    pub token: String,
    pub id_token: String,
    pub player_info: PlayerInfoSmall,
    pub entitlement_token: String,
    pub region: String,
}

pub async fn full_auth(client: &Client) -> FullAuth {
    auth_cookies(&client).await.unwrap();
    let username = std::env::var("USER_NAME").unwrap();
    let password = std::env::var("USER_PASSWORD").unwrap();
    let auth_request = auth_request(
        &client,
        (username, password),
    )
    .await
    .unwrap();

    let token = auth_request.access_token;

    let player_info = get_player_info(&client, token.clone()).await.unwrap();

    let entitlement_token = get_entitlements(&client, token.clone())
        .await
        .unwrap()
        .entitlements_token;

    FullAuth {
        token,
        id_token: auth_request.id_token,
        player_info,
        entitlement_token,
        region: "ap".to_string(),
    }
}
