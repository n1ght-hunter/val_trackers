use reqwest_impersonate::Client;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

pub async fn get_store(
    client: &Client,
    region: &str,
    puuid: &str,
    token: &str,
    entitlements: &str,
) -> Result<Store, reqwest_impersonate::Error> {
    client
        .get(format!(
            "https://pd.{}.a.pvp.net/store/v2/storefront/{}",
            region, puuid
        ))
        .header("X-Riot-Entitlements-JWT", entitlements)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    #[serde(rename = "FeaturedBundle")]
    pub featured_bundle: FeaturedBundle,
    #[serde(rename = "SkinsPanelLayout")]
    pub skins_panel_layout: SkinsPanelLayout,
    #[serde(rename = "UpgradeCurrencyStore")]
    pub upgrade_currency_store: UpgradeCurrencyStore,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeaturedBundle {
    #[serde(rename = "Bundle")]
    pub bundle: Bundle,
    #[serde(rename = "Bundles")]
    pub bundles: Vec<Bundle2>,
    #[serde(rename = "BundleRemainingDurationInSeconds")]
    pub bundle_remaining_duration_in_seconds: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "DataAssetID")]
    pub data_asset_id: String,
    #[serde(rename = "CurrencyID")]
    pub currency_id: String,
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "ItemOffers")]
    pub item_offers: Value,
    #[serde(rename = "TotalBaseCost")]
    pub total_base_cost: Value,
    #[serde(rename = "TotalDiscountedCost")]
    pub total_discounted_cost: Value,
    #[serde(rename = "TotalDiscountPercent")]
    pub total_discount_percent: f32,
    #[serde(rename = "DurationRemainingInSeconds")]
    pub duration_remaining_in_seconds: i64,
    #[serde(rename = "WholesaleOnly")]
    pub wholesale_only: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "Item")]
    pub item: Item2,
    #[serde(rename = "BasePrice")]
    pub base_price: i64,
    #[serde(rename = "CurrencyID")]
    pub currency_id: String,
    #[serde(rename = "DiscountPercent")]
    pub discount_percent: f32,
    #[serde(rename = "DiscountedPrice")]
    pub discounted_price: i64,
    #[serde(rename = "IsPromoItem")]
    pub is_promo_item: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item2 {
    #[serde(rename = "ItemTypeID")]
    pub item_type_id: String,
    #[serde(rename = "ItemID")]
    pub item_id: String,
    #[serde(rename = "Amount")]
    pub amount: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle2 {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "DataAssetID")]
    pub data_asset_id: String,
    #[serde(rename = "CurrencyID")]
    pub currency_id: String,
    #[serde(rename = "Items")]
    pub items: Vec<Item3>,
    #[serde(rename = "ItemOffers")]
    pub item_offers: Vec<ItemOffer>,
    #[serde(rename = "TotalBaseCost")]
    pub total_base_cost: TotalBaseCost,
    #[serde(rename = "TotalDiscountedCost")]
    pub total_discounted_cost: TotalDiscountedCost,
    #[serde(rename = "TotalDiscountPercent")]
    pub total_discount_percent: f64,
    #[serde(rename = "DurationRemainingInSeconds")]
    pub duration_remaining_in_seconds: i64,
    #[serde(rename = "WholesaleOnly")]
    pub wholesale_only: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item3 {
    #[serde(rename = "Item")]
    pub item: Item4,
    #[serde(rename = "BasePrice")]
    pub base_price: i64,
    #[serde(rename = "CurrencyID")]
    pub currency_id: String,
    #[serde(rename = "DiscountPercent")]
    pub discount_percent: f32,
    #[serde(rename = "DiscountedPrice")]
    pub discounted_price: i64,
    #[serde(rename = "IsPromoItem")]
    pub is_promo_item: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item4 {
    #[serde(rename = "ItemTypeID")]
    pub item_type_id: String,
    #[serde(rename = "ItemID")]
    pub item_id: String,
    #[serde(rename = "Amount")]
    pub amount: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemOffer {
    #[serde(rename = "BundleItemOfferID")]
    pub bundle_item_offer_id: String,
    #[serde(rename = "Offer")]
    pub offer: Offer,
    #[serde(rename = "DiscountPercent")]
    pub discount_percent: f32,
    #[serde(rename = "DiscountedCost")]
    pub discounted_cost: DiscountedCost,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    #[serde(rename = "OfferID")]
    pub offer_id: String,
    #[serde(rename = "IsDirectPurchase")]
    pub is_direct_purchase: bool,
    #[serde(rename = "StartDate")]
    pub start_date: String,
    #[serde(rename = "Cost")]
    pub cost: Cost,
    #[serde(rename = "Rewards")]
    pub rewards: Vec<Reward>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cost {
    #[serde(rename = "85ad13f7-3d1b-5128-9eb2-7cd8ee0b5741")]
    pub n85ad13f7_3d1b_5128_9eb2_7cd8ee0b5741: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reward {
    #[serde(rename = "ItemTypeID")]
    pub item_type_id: String,
    #[serde(rename = "ItemID")]
    pub item_id: String,
    #[serde(rename = "Quantity")]
    pub quantity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountedCost {
    #[serde(rename = "85ad13f7-3d1b-5128-9eb2-7cd8ee0b5741")]
    pub n85ad13f7_3d1b_5128_9eb2_7cd8ee0b5741: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalBaseCost {
    #[serde(rename = "85ad13f7-3d1b-5128-9eb2-7cd8ee0b5741")]
    pub n85ad13f7_3d1b_5128_9eb2_7cd8ee0b5741: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalDiscountedCost {
    #[serde(rename = "85ad13f7-3d1b-5128-9eb2-7cd8ee0b5741")]
    pub n85ad13f7_3d1b_5128_9eb2_7cd8ee0b5741: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinsPanelLayout {
    #[serde(rename = "SingleItemOffers")]
    pub single_item_offers: Vec<String>,
    #[serde(rename = "SingleItemStoreOffers")]
    pub single_item_store_offers: Vec<SingleItemStoreOffer>,
    #[serde(rename = "SingleItemOffersRemainingDurationInSeconds")]
    pub single_item_offers_remaining_duration_in_seconds: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleItemStoreOffer {
    #[serde(rename = "OfferID")]
    pub offer_id: String,
    #[serde(rename = "IsDirectPurchase")]
    pub is_direct_purchase: bool,
    #[serde(rename = "StartDate")]
    pub start_date: String,
    #[serde(rename = "Cost")]
    pub cost: Cost2,
    #[serde(rename = "Rewards")]
    pub rewards: Vec<Reward2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cost2 {
    #[serde(rename = "85ad13f7-3d1b-5128-9eb2-7cd8ee0b5741")]
    pub n85ad13f7_3d1b_5128_9eb2_7cd8ee0b5741: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reward2 {
    #[serde(rename = "ItemTypeID")]
    pub item_type_id: String,
    #[serde(rename = "ItemID")]
    pub item_id: String,
    #[serde(rename = "Quantity")]
    pub quantity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeCurrencyStore {
    #[serde(rename = "UpgradeCurrencyOffers")]
    pub upgrade_currency_offers: Vec<UpgradeCurrencyOffer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeCurrencyOffer {
    #[serde(rename = "OfferID")]
    pub offer_id: String,
    #[serde(rename = "StorefrontItemID")]
    pub storefront_item_id: String,
    #[serde(rename = "Offer")]
    pub offer: Offer2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Offer2 {
    #[serde(rename = "OfferID")]
    pub offer_id: String,
    #[serde(rename = "IsDirectPurchase")]
    pub is_direct_purchase: bool,
    #[serde(rename = "StartDate")]
    pub start_date: String,
    #[serde(rename = "Cost")]
    pub cost: Cost3,
    #[serde(rename = "Rewards")]
    pub rewards: Vec<Reward3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cost3 {
    #[serde(rename = "85ad13f7-3d1b-5128-9eb2-7cd8ee0b5741")]
    pub n85ad13f7_3d1b_5128_9eb2_7cd8ee0b5741: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reward3 {
    #[serde(rename = "ItemTypeID")]
    pub item_type_id: String,
    #[serde(rename = "ItemID")]
    pub item_id: String,
    #[serde(rename = "Quantity")]
    pub quantity: i64,
}

#[tokio::test]
async fn test_store() {
    let client = crate::create_client().unwrap();
    let auth = crate::auth::full_auth(&client).await;
    let store = crate::val_api::store::get_store(
        &client,
        &auth.region,
        &auth.player_info.puuid,
        &auth.token,
        &auth.entitlement_token,
    )
    .await
    .unwrap();
    println!("{:?}", store);
}
