use reqwest_impersonate::Client;
use serde_derive::Deserialize;
use serde_derive::Serialize;

pub async fn weapon_skins(client: &Client) -> Result<Vec<Weapons>, reqwest_impersonate::Error> {
    Ok(client
        .get("https://valorant-api.com/v1/weapons/skins")
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
    pub data: Vec<Weapons>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weapons {
    pub uuid: String,
    pub display_name: String,
    pub theme_uuid: String,
    pub content_tier_uuid: Option<String>,
    pub display_icon: Option<String>,
    pub wallpaper: Option<String>,
    pub asset_path: String,
    pub chromas: Vec<SkinChroma>,
    pub levels: Vec<SkinLevel>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinChroma {
    pub uuid: String,
    pub display_name: String,
    pub display_icon: Option<String>,
    pub full_render: String,
    pub swatch: Option<String>,
    pub streamed_video: Option<String>,
    pub asset_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinLevel {
    pub uuid: String,
    pub display_name: String,
    pub level_item: Option<String>,
    pub display_icon: Option<String>,
    pub streamed_video: Option<String>,
    pub asset_path: String,
}

#[tokio::test]
async fn test_weaponskins_and_store() {
    let client = crate::create_client().unwrap();
    let auth = crate::auth::full_auth(&client).await;
    let store = crate::val_api::get_store(
        &client,
        &auth.region,
        &auth.player_info.puuid,
        &auth.token,
        &auth.entitlement_token,
    )
    .await
    .unwrap();
    let weapon_skins = weapon_skins(&client).await.unwrap();
    let store_weapons = store
        .skins_panel_layout
        .single_item_store_offers
        .iter()
        .map(|store_item| {
            let weapon = weapon_skins
                .iter()
                .find(|f| store_item.offer_id == f.levels[0].uuid)
                .unwrap()
                .clone();
            (weapon, store_item.clone())
        })
        .collect::<Vec<(Weapons, crate::val_api::SingleItemStoreOffer)>>();

    for (weapon, store_item) in store_weapons {
        println!(
            "{}: {}",
            weapon.display_name, store_item.cost.n85ad13f7_3d1b_5128_9eb2_7cd8ee0b5741
        )
    }
}
