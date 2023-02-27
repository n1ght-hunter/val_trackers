use std::borrow::Cow;

use api::{auth::FullAuth, game_content};
use chrono::{Duration, Utc};
use iced::{color, subscription, widget::image, Color};

use crate::{componects, state::GameContent, Message};

// UTC Time 1:00 am

enum StoreWait {
    Start(api::Client, FullAuth, GameContent),
    Wait(api::Client, FullAuth, GameContent),
}

pub fn subscription(
    client: &api::Client,
    auth: &FullAuth,
    game_content: &GameContent,
) -> iced::Subscription<Message> {
    subscription::unfold(
        "friends_list",
        StoreWait::Start(client.clone(), auth.clone(), game_content.clone()),
        |state| async move {
            match state {
                StoreWait::Start(client, auth, game_content) => {
                    let store_weapons = get_store(&client, &auth, &game_content).await;
                    (
                        Some(store_weapons),
                        StoreWait::Wait(client, auth, game_content),
                    )
                }
                StoreWait::Wait(client, auth, game_content) => {
                    tokio::time::sleep(duration_till_store_update()).await;
                    let store_weapons = get_store(&client, &auth, &game_content).await;

                    (
                        Some(store_weapons),
                        StoreWait::Wait(client, auth, game_content),
                    )
                }
            }
        },
    )
    .map(|(x, y)| Message::Store(componects::store::Event::Store(x, y)))
}

pub fn duration_till_store_update() -> std::time::Duration {
    let now = Utc::now();
    (now + Duration::days(1))
        .date_naive()
        .and_hms_opt(1, 0, 0)
        .unwrap()
        .signed_duration_since(now.naive_utc())
        .to_std()
        .unwrap()
}

pub async fn get_store(
    client: &api::Client,
    auth: &FullAuth,
    game_content: &GameContent,
) -> (Vec<super::QuickShowStore>, Vec<super::state::Bundle>) {
    let store = api::val_api::get_store(
        &client,
        "ap",
        &auth.player_info.puuid,
        &auth.token,
        &auth.entitlement_token,
    )
    .await
    .unwrap();

    let handles = store
        .skins_panel_layout
        .single_item_store_offers
        .iter()
        .map(|store_item| async {
            let weapon = game_content
                .weapon_skins
                .iter()
                .find(|f| store_item.offer_id == f.levels[0].uuid)
                .unwrap()
                .clone();
            let tier = game_content
                .content_tiers
                .iter()
                .find(|tier| &tier.uuid == weapon.content_tier_uuid.as_ref().unwrap())
                .unwrap();

            let gun_image = get_image(weapon.levels[0].display_icon.clone().unwrap(), client);
            let tier_image = get_image(tier.display_icon.clone(), client);
            let background_icon = get_image(
                game_content
                    .themes
                    .iter()
                    .find(|theme| theme.uuid == weapon.theme_uuid)
                    .unwrap()
                    .display_icon
                    .clone()
                    .unwrap(),
                client,
            );

            let (gun_image, tier_image, background_icon) =
                tokio::join!(gun_image, tier_image, background_icon);

            let background_color = hex_to_color(&tier.highlight_color);

            super::QuickShowStore {
                weapon,
                store_item: store_item.clone(),
                background_icon,
                background_color,
                gun_image,
                tier_image,
            }
        });

    let mut output_weapons = Vec::with_capacity(handles.len());
    for handle in handles {
        output_weapons.push(handle.await);
    }
    let handles = store.featured_bundle.bundles.iter().map(|bundle| async {
        let bundle_content = game_content
            .bundles
            .iter()
            .find(|bundle_content| bundle_content.uuid == bundle.data_asset_id)
            .unwrap();

        let image = get_image(bundle_content.display_icon.clone(), client).await;

        super::state::Bundle {
            image,
            name: bundle_content.display_name.clone(),
            price: bundle
                .total_discounted_cost
                .n85ad13f7_3d1b_5128_9eb2_7cd8ee0b5741,
            time_remaining: bundle.duration_remaining_in_seconds,
        }
    });
    let mut output_bundles = Vec::with_capacity(handles.len());
    for handle in handles {
        output_bundles.push(handle.await);
    }

    (output_weapons, output_bundles)
}

async fn get_image(url: String, client: &api::Client) -> image::Handle {
    let bytes = &client.get(url).send().await.unwrap().bytes().await.unwrap();
    image::Handle::from_memory(bytes.to_vec())
}

fn hex_to_color(hex: &str) -> Color {
    let (hex, hex_opacity) = hex.split_at(hex.len() - 2);
    let hex_opacity = u32::from_str_radix(hex_opacity, 16).unwrap() as f32 / 255.0;
    let hex = u32::from_str_radix(hex, 16).unwrap();
    let r = (hex & 0xff0000) >> 16;
    let g = (hex & 0xff00) >> 8;
    let b = hex & 0xff;
    Color::from_rgba8(r as u8, g as u8, b as u8, hex_opacity)
}
