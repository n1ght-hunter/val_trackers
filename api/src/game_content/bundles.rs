use reqwest_impersonate::Client;
use serde_derive::Deserialize;
use serde_derive::Serialize;

pub async fn bundles(client: &Client) -> Result<Vec<Bundle>, reqwest_impersonate::Error> {
    Ok(client
        .get("https://valorant-api.com/v1/bundles")
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
    pub data: Vec<Bundle>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    pub uuid: String,
    pub display_name: String,
    pub display_name_sub_text: Option<String>,
    pub description: String,
    pub extra_description: Option<String>,
    pub promo_description: Option<String>,
    pub use_additional_context: bool,
    pub display_icon: String,
    pub display_icon2: String,
    pub vertical_promo_image: Option<String>,
    pub asset_path: String,
}

#[tokio::test]
async fn test_bundles() {
    let client = crate::create_client().unwrap();
    let bundles = bundles(&client).await.unwrap();

    for bundle in bundles {
        println!("{}: {}", bundle.display_name, bundle.description)
    }
}
