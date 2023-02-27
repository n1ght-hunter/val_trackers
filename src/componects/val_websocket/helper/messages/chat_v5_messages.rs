
use serde_derive::{Deserialize, Serialize};



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatV5Messages {
    pub data: MessagesData,
    pub event_type: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagesData {
    pub messages: Vec<Message>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub body: String,
    pub cid: String,
    #[serde(rename = "game_name")]
    pub game_name: String,
    #[serde(rename = "game_tag")]
    pub game_tag: String,
    pub id: String,
    pub mid: String,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    pub read: bool,
    pub region: String,
    pub time: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

