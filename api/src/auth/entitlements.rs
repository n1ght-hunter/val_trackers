use reqwest_impersonate::Client;
use serde_derive::Deserialize;
use serde_derive::Serialize;

pub async fn get_entitlements(client: &Client, token: String) -> Result<Root, reqwest_impersonate::Error> {
    client
        .post("https://entitlements.auth.riotgames.com/api/token/v1")
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub entitlements_token: String,
}
