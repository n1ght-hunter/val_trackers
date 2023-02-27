
use serde_derive::{Deserialize, Serialize};



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatV6Conversations {
    pub data: ConversationsData,
    pub event_type: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsData {
    pub conversations: Vec<Conversation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conversation {
    pub cid: String,
    #[serde(rename = "direct_messages")]
    pub direct_messages: bool,
    #[serde(rename = "global_readership")]
    pub global_readership: bool,
    #[serde(rename = "message_history")]
    pub message_history: bool,
    pub mid: String,
    pub muted: bool,
    pub muted_restriction: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub ui_state: UiState,
    #[serde(rename = "unread_count")]
    pub unread_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiState {
    pub changed_since_hidden: bool,
    pub hidden: bool,
}