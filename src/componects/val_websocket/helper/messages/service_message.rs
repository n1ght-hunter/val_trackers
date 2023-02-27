
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
        if self.data.service == "pregame" {
            let string_parts = self.data.resource.split("/").collect::<Vec<&str>>();

            Some(string_parts[string_parts.len() - 1].to_string())
        } else {
            None
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceMessageData {
    pub ack_required: bool,
    pub id: String,
    pub payload: String,
    pub resource: String,
    pub service: String,
    pub timestamp: i64,
    pub version: String,
}
