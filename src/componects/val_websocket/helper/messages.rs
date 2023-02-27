mod chat_v4_presences;
mod chat_v5_messages;
mod chat_v5_participants;
mod chat_v6_conversations;
mod service_message;

pub use chat_v4_presences::ChatV4Presences;
pub use chat_v5_messages::ChatV5Messages;
pub use chat_v5_participants::ChatV5Participants;
pub use chat_v6_conversations::ChatV6Conversations;
pub use service_message::ServiceMessage;

use serde_derive::{Deserialize, Serialize};

// pub static EVENTS: &'static [&str] = &[
//     "OnClientMinimize",
//     "OnClientFocus",
//     "OnJsonApiEvent_agent_v1_session",
//     "OnJsonApiEvent_agent_v1_requests",
//     "OnJsonApiEvent_chat_v1_session",
//    asdfasds "OnJsonApiEvent_chat_v4_presences",
//     "OnJsonApiEvent_chat_v4_friends",
//  ASDFASD   "OnJsonApiEvent_chat_v5_messages",
//   asdfadf  "OnJsonApiEvent_chat_v5_participants",
//asdfasd "OnJsonApiEvent_chat_v6_conversations",
//     "OnJsonApiEvent_chat_v6_friendrequests",
// "OnJsonApiEvent_riot-messaging-service_v1_message",
// ];

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Messages {
    ChatV6Conversations(ChatV6Conversations),
    ChatV5Messages(ChatV5Messages),
    ChatV4Presences(ChatV4Presences),
    ChatV5Participants(ChatV5Participants),
    ServiceMessage(ServiceMessage),
    None,
}

