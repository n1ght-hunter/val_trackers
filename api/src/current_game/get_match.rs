use std::sync::Arc;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use reqwest_impersonate::Client;

use crate::auth::FullAuth;

pub async fn get_match(
    client: &Client,
    full_auth: Arc<FullAuth>,
    match_id: String,
) -> Result<Match, reqwest_impersonate::Error> {
    client
        .get(format!(
            "https://glz-{}-1.{}.a.pvp.net/core-game/v1/matches/{}",
            full_auth.region, full_auth.region, match_id
        ))
        .header("X-Riot-Entitlements-JWT", &full_auth.entitlement_token)
        .header("Content-Type", "application/json")
        .bearer_auth(full_auth.token.clone())
        .send()
        .await?
        .json()
        .await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    #[serde(rename = "MatchID")]
    pub match_id: String,
    #[serde(rename = "Version")]
    pub version: i64,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "MapID")]
    pub map_id: String,
    #[serde(rename = "ModeID")]
    pub mode_id: String,
    #[serde(rename = "ProvisioningFlow")]
    pub provisioning_flow: String,
    #[serde(rename = "GamePodID")]
    pub game_pod_id: String,
    #[serde(rename = "AllMUCName")]
    pub all_mucname: String,
    #[serde(rename = "TeamMUCName")]
    pub team_mucname: String,
    #[serde(rename = "TeamVoiceID")]
    pub team_voice_id: String,
    #[serde(rename = "IsReconnectable")]
    pub is_reconnectable: bool,
    #[serde(rename = "ConnectionDetails")]
    pub connection_details: ConnectionDetails,
    #[serde(rename = "PostGameDetails")]
    pub post_game_details: Value,
    #[serde(rename = "Players")]
    pub players: Vec<Player>,
    #[serde(rename = "MatchmakingData")]
    pub matchmaking_data: Option<MatchmakingData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionDetails {
    #[serde(rename = "GameServerHosts")]
    pub game_server_hosts: Vec<String>,
    #[serde(rename = "GameServerHost")]
    pub game_server_host: String,
    #[serde(rename = "GameServerPort")]
    pub game_server_port: i64,
    #[serde(rename = "GameServerObfuscatedIP")]
    pub game_server_obfuscated_ip: i64,
    #[serde(rename = "GameClientHash")]
    pub game_client_hash: i64,
    #[serde(rename = "PlayerKey")]
    pub player_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    #[serde(rename = "Subject")]
    pub subject: String,
    #[serde(rename = "TeamID")]
    pub team_id: String,
    #[serde(rename = "CharacterID")]
    pub character_id: String,
    #[serde(rename = "PlayerIdentity")]
    pub player_identity: PlayerIdentity,
    #[serde(rename = "SeasonalBadgeInfo")]
    pub seasonal_badge_info: SeasonalBadgeInfo,
    #[serde(rename = "IsCoach")]
    pub is_coach: bool,
    #[serde(rename = "IsAssociated")]
    pub is_associated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerIdentity {
    #[serde(rename = "Subject")]
    pub subject: String,
    #[serde(rename = "PlayerCardID")]
    pub player_card_id: String,
    #[serde(rename = "PlayerTitleID")]
    pub player_title_id: String,
    #[serde(rename = "AccountLevel")]
    pub account_level: i64,
    #[serde(rename = "PreferredLevelBorderID")]
    pub preferred_level_border_id: String,
    #[serde(rename = "Incognito")]
    pub incognito: bool,
    #[serde(rename = "HideAccountLevel")]
    pub hide_account_level: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeasonalBadgeInfo {
    #[serde(rename = "SeasonID")]
    pub season_id: String,
    #[serde(rename = "NumberOfWins")]
    pub number_of_wins: i64,
    #[serde(rename = "WinsByTier")]
    pub wins_by_tier: Value,
    #[serde(rename = "Rank")]
    pub rank: i64,
    #[serde(rename = "LeaderboardRank")]
    pub leaderboard_rank: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingData {
    #[serde(rename = "QueueID")]
    pub queue_id: String,
    #[serde(rename = "IsRanked")]
    pub is_ranked: bool,
}
