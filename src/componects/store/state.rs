#[derive(Clone, Debug, Default)]
pub struct State {
    pub weapons: Vec<crate::componects::store::QuickShowStore>,
    pub bundles: Vec<Bundle>,
}

#[derive(Clone, Debug)]
pub struct QuickShowStore {
    pub weapon: api::game_content::weapon_skins::Weapons,
    pub store_item: api::val_api::SingleItemStoreOffer,
    pub background_color: iced::Color,
    pub background_icon: iced::widget::image::Handle,
    pub gun_image: iced::widget::image::Handle,
    pub tier_image: iced::widget::image::Handle,
}

#[derive(Clone, Debug)]
pub struct Bundle {
    pub image: iced::widget::image::Handle,
    pub name: String,
    pub price: i64,
    pub time_remaining: i64,
}
