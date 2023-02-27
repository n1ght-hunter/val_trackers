use reqwest_impersonate::Client;
use serde_derive::Deserialize;
use serde_derive::Serialize;

pub async fn get_theme(client: &Client) -> Result<Vec<Theme>, reqwest_impersonate::Error> {
    Ok(client
        .get("https://valorant-api.com/v1/themes")
        .send()
        .await?
        .json::<Root>()
        .await?
        .data)
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub status: i64,
    pub data: Vec<Theme>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub uuid: String,
    pub display_name: String,
    pub display_icon: Option<String>,
    pub store_featured_image: Option<String>,
    pub asset_path: String,
}
