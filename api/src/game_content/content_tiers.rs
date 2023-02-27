use reqwest_impersonate::Client;
use serde_derive::Deserialize;
use serde_derive::Serialize;

pub async fn content_tiers(
    client: &Client,
) -> Result<Vec<ContentTiers>, reqwest_impersonate::Error> {
    Ok(client
        .get("https://valorant-api.com/v1/contenttiers")
        .send()
        .await?
        .json::<Root>()
        .await?
        .data)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Root {
    pub status: i64,
    pub data: Vec<ContentTiers>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentTiers {
    pub uuid: String,
    pub display_name: String,
    pub dev_name: String,
    pub rank: i64,
    pub juice_value: i64,
    pub juice_cost: i64,
    pub highlight_color: String,
    pub display_icon: String,
    pub asset_path: String,
}

#[tokio::test]
async fn test_weaponskins_and_store() {
    let client = crate::create_client().unwrap();
    let tiers = content_tiers(&client).await.unwrap();

    for tier in tiers {
        println!("{}: {}", tier.display_name, tier.highlight_color)
    }
}
