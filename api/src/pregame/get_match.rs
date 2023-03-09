use std::sync::Arc;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use reqwest_impersonate::Client;

use crate::auth::FullAuth;

pub async fn get_match(client: &Client, full_auth: Arc<FullAuth>, pre_match_id: String) -> Result<PreMatch, reqwest_impersonate::Error> {
    client
        .get(format!("https://glz-{}-1.{}.a.pvp.net/pregame/v1/matches/{}", full_auth.region, full_auth.region, pre_match_id))
        .header("Content-Type", "application/json")
        .bearer_auth(full_auth.token.clone())
        .send()
        .await?
        .json()
        .await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreMatch {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Version")]
    pub version: i64,
    #[serde(rename = "Teams")]
    pub teams: Vec<Team>,
    #[serde(rename = "AllyTeam")]
    pub ally_team: AllyTeam,
    #[serde(rename = "EnemyTeam")]
    pub enemy_team: Option<AllyTeam>,
    #[serde(rename = "ObserverSubjects")]
    pub observer_subjects: Vec<Value>,
    #[serde(rename = "MatchCoaches")]
    pub match_coaches: Vec<Value>,
    #[serde(rename = "EnemyTeamSize")]
    pub enemy_team_size: i64,
    #[serde(rename = "EnemyTeamLockCount")]
    pub enemy_team_lock_count: i64,
    #[serde(rename = "PregameState")]
    pub pregame_state: String,
    #[serde(rename = "LastUpdated")]
    pub last_updated: String,
    #[serde(rename = "MapID")]
    pub map_id: String,
    #[serde(rename = "MapSelectPool")]
    pub map_select_pool: Vec<Value>,
    #[serde(rename = "BannedMapIDs")]
    pub banned_map_ids: Vec<Value>,
    #[serde(rename = "CastedVotes")]
    pub casted_votes: CastedVotes,
    #[serde(rename = "MapSelectSteps")]
    pub map_select_steps: Vec<Value>,
    #[serde(rename = "MapSelectStep")]
    pub map_select_step: i64,
    #[serde(rename = "Team1")]
    pub team1: String,
    #[serde(rename = "GamePodID")]
    pub game_pod_id: String,
    #[serde(rename = "Mode")]
    pub mode: String,
    #[serde(rename = "VoiceSessionID")]
    pub voice_session_id: String,
    #[serde(rename = "MUCName")]
    pub mucname: String,
    #[serde(rename = "QueueID")]
    pub queue_id: String,
    #[serde(rename = "ProvisioningFlowID")]
    pub provisioning_flow_id: String,
    #[serde(rename = "IsRanked")]
    pub is_ranked: bool,
    #[serde(rename = "PhaseTimeRemainingNS")]
    pub phase_time_remaining_ns: i64,
    #[serde(rename = "StepTimeRemainingNS")]
    pub step_time_remaining_ns: i64,
    #[serde(rename = "altModesFlagADA")]
    pub alt_modes_flag_ada: bool,
    #[serde(rename = "TournamentMetadata")]
    pub tournament_metadata: Value,
    #[serde(rename = "RosterMetadata")]
    pub roster_metadata: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    #[serde(rename = "TeamID")]
    pub team_id: String,
    #[serde(rename = "Players")]
    pub players: Vec<Player>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    #[serde(rename = "Subject")]
    pub subject: String,
    #[serde(rename = "CharacterID")]
    pub character_id: String,
    #[serde(rename = "CharacterSelectionState")]
    pub character_selection_state: String,
    #[serde(rename = "PregamePlayerState")]
    pub pregame_player_state: String,
    #[serde(rename = "CompetitiveTier")]
    pub competitive_tier: i64,
    #[serde(rename = "PlayerIdentity")]
    pub player_identity: PlayerIdentity,
    #[serde(rename = "SeasonalBadgeInfo")]
    pub seasonal_badge_info: SeasonalBadgeInfo,
    #[serde(rename = "IsCaptain")]
    pub is_captain: bool,
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
pub struct AllyTeam {
    #[serde(rename = "TeamID")]
    pub team_id: String,
    #[serde(rename = "Players")]
    pub players: Vec<Player2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player2 {
    #[serde(rename = "Subject")]
    pub subject: String,
    #[serde(rename = "CharacterID")]
    pub character_id: String,
    #[serde(rename = "CharacterSelectionState")]
    pub character_selection_state: String,
    #[serde(rename = "PregamePlayerState")]
    pub pregame_player_state: String,
    #[serde(rename = "CompetitiveTier")]
    pub competitive_tier: i64,
    #[serde(rename = "PlayerIdentity")]
    pub player_identity: PlayerIdentity2,
    #[serde(rename = "SeasonalBadgeInfo")]
    pub seasonal_badge_info: SeasonalBadgeInfo2,
    #[serde(rename = "IsCaptain")]
    pub is_captain: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerIdentity2 {
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
pub struct SeasonalBadgeInfo2 {
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
pub struct CastedVotes {
}
