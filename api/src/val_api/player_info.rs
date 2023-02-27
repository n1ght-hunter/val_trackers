use reqwest_impersonate::Client;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

pub async fn get_player_info(client: &Client, token: String) -> Result<PlayerInfoSmall, reqwest_impersonate::Error> {
    client
        .get("https://auth.riotgames.com/userinfo")
        .bearer_auth(token)
        .send()
        .await?
        // .text()
        .json::<PlayerInfoSmall>()
        .await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfoSmall {
    #[serde(rename = "sub")]
    pub puuid: String,
    pub jti: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfo {
    pub country: String,
    #[serde(rename = "sub")]
    pub puuid: String,
    #[serde(rename = "email_verified")]
    pub email_verified: bool,
    #[serde(rename = "player_plocale")]
    pub player_plocale: Value,
    #[serde(rename = "country_at")]
    pub country_at: Value,
    pub pw: Pw,
    #[serde(rename = "phone_number_verified")]
    pub phone_number_verified: bool,
    #[serde(rename = "account_verified")]
    pub account_verified: bool,
    pub ppid: Value,
    #[serde(rename = "federated_identity_providers")]
    pub federated_identity_providers: Vec<String>,
    #[serde(rename = "player_locale")]
    pub player_locale: Value,
    pub acct: Acct,
    pub age: i64,
    pub jti: String,
    pub affinity: Affinity,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pw {
    #[serde(rename = "cng_at")]
    pub cng_at: i64,
    pub reset: bool,
    #[serde(rename = "must_reset")]
    pub must_reset: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Acct {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub state: String,
    pub adm: bool,
    #[serde(rename = "game_name")]
    pub game_name: String,
    #[serde(rename = "tag_line")]
    pub tag_line: String,
    #[serde(rename = "created_at")]
    pub created_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Affinity {
    pub pp: String,
}
