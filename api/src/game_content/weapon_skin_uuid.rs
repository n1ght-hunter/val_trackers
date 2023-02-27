use reqwest_impersonate::Client;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

pub async fn weapon_skin_uuid(client: &Client, uuid: &String) -> Result<WeaponSkinLevel, reqwest_impersonate::Error>  {
    client
        .get(format!(
            "https://valorant-api.com/v1/weapons/skinlevels/{}",
            uuid
        ))
        .send()
        .await?
        .json()
        .await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponSkinLevel {
    pub status: i64,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub uuid: String,
    pub display_name: String,
    pub level_item: Value,
    pub display_icon: String,
    pub streamed_video: Option<String>,
    pub asset_path: String,
}
