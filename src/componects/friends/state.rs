

use serde::Deserializer;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllFriends {
    pub active_platform: Option<String>,
    pub display_group: String,
    #[serde(rename = "game_name")]
    pub game_name: String,
    #[serde(rename = "game_tag")]
    pub game_tag: String,
    pub group: String,
    #[serde(rename = "last_online_ts")]
    pub last_online_ts: Option<i64>,
    pub name: String,
    pub note: String,
    pub pid: String,
    pub puuid: String,
    pub region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnlineFriends {
    pub actor: Value,
    pub basic: String,
    pub details: Value,
    #[serde(rename = "game_name")]
    pub game_name: String,
    #[serde(rename = "game_tag")]
    pub game_tag: String,
    pub location: Value,
    pub msg: Value,
    pub name: String,
    pub patchline: Option<String>,
    pub pid: String,
    pub platform: Option<String>,
    #[serde(deserialize_with = "from_base64")]
    pub private: Private,
    pub private_jwt: Value,
    pub product: String,
    pub puuid: String,
    pub region: String,
    pub resource: String,
    pub state: String,
    pub summary: String,
    pub time: i64,
}

#[derive(Debug, Clone, Default)]
pub struct Friends {
    pub online_friends: Vec<OnlineFriends>,
    pub offline_friends: Vec<AllFriends>,
    pub all_friends: Vec<AllFriends>,
}

impl Friends {
    pub fn new() -> Self {
        Self {
            online_friends: Vec::new(),
            offline_friends: Vec::new(),
            all_friends: Vec::new(),
        }
    }
}

fn from_base64<'de, D>(deserializer: D) -> Result<Private, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    use serde::Deserialize;
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
        .map(|bytes| serde_json::from_slice::<Private>(&bytes))
        .and_then(|opt| opt.map_err(|_|Error::custom("failed to deserialize private friends")))
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Private {
    pub is_valid: bool,
    pub session_loop_state: String,
    pub party_owner_session_loop_state: String,
    pub custom_game_name: String,
    pub custom_game_team: String,
    pub party_owner_match_map: String,
    pub party_owner_match_current_team: String,
    pub party_owner_match_score_ally_team: i64,
    pub party_owner_match_score_enemy_team: i64,
    pub party_owner_provisioning_flow: String,
    pub provisioning_flow: String,
    pub match_map: String,
    pub party_id: String,
    pub is_party_owner: bool,
    pub party_state: String,
    pub party_accessibility: String,
    pub max_party_size: i64,
    pub queue_id: String,
    #[serde(rename = "partyLFM")]
    pub party_lfm: bool,
    pub party_client_version: String,
    pub party_size: i64,
    pub tournament_id: String,
    pub roster_id: String,
    pub party_version: i64,
    pub queue_entry_time: String,
    pub player_card_id: String,
    pub player_title_id: String,
    pub preferred_level_border_id: String,
    pub account_level: i64,
    pub competitive_tier: i64,
    pub leaderboard_position: i64,
    pub is_idle: bool,
}
