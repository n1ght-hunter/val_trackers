
use api::current_game::Player;
use serde_derive::{Deserialize, Serialize};



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceMessage {
    pub data: ServiceMessageData,
    pub event_type: String,
    pub uri: String,
}

impl ServiceMessage {
    pub fn pre_match(self) -> Option<String> {
        if let Some(payload) = self.data.payload  {
            if payload.loop_state == "PREGAME" {
                return Some(payload.loop_state_metadata)
            }
        } 
        None
    }

    pub fn core_game(self) -> Option<String> {
        if let Some(payload) = self.data.payload  {
            if payload.loop_state == "INGAME" {
                return Some(payload.loop_state_metadata)
            }
        } 
        None
    }

    pub fn menu(&self) -> bool {
        if let Some(payload) = &self.data.payload  {
            if payload.loop_state == "MENUS" {
                return true;
            }
        } 
        false
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceMessageData {
    pub ack_required: bool,
    pub id: String,
    #[serde(deserialize_with = "maybe_extra_data")]
    pub payload: Option<Payload>,
    pub resource: String,
    pub service: String,
    pub timestamp: i64,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub subject: String,
    pub cxn_state: String,
    #[serde(rename = "clientID")]
    pub client_id: String,
    pub client_version: String,
    pub loop_state: String,
    pub loop_state_metadata: String,
    pub version: i64,
    pub last_heartbeat_time: String,
    pub expired_time: String,
    pub heartbeat_interval_millis: i64,
    pub playtime_notification: String,
    pub playtime_minutes: i64,
    pub is_restricted: bool,
    pub userinfo_valid_time: String,
    pub restriction_type: String,
    pub client_platform_info: ClientPlatformInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientPlatformInfo {
    pub platform_type: String,
    #[serde(rename = "platformOS")]
    pub platform_os: String,
    #[serde(rename = "platformOSVersion")]
    pub platform_osversion: String,
    pub platform_chipset: String,
}


fn maybe_extra_data<'de, D>(deserializer: D) -> Result<Option<Payload>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde::Deserialize;
    let s =  String::deserialize(deserializer)?;
    if s.len() > 0 {
        Ok(Some(serde_json::from_str(&s).map_err(|x|Error::custom(format!("can't deserialise playload, {:?}", x)))?))
    } else {
        Ok(None)
    }
}