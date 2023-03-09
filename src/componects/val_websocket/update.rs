use iced::Command;

use crate::{
    helpers::async_update::{async_update, AsyncUpdate},
    state::{LiveState, PreGameData, State},
    Message,
};

use super::{helper::messages, Event};

pub fn update(state: &mut State, event: Event) -> Command<Message> {
    match event {
        Event::Connected => {
            state.websocket_status = "Connected".to_string();
            println!("Connected");
        }
        Event::Disconnected => {
            state.websocket_status = "Disconnected".to_string();
            println!("Disconnected");
        }
        Event::MessageReceived(message) => match message {
            messages::Messages::ChatV6Conversations(_) => (),
            messages::Messages::ChatV5Messages(_) => {}
            messages::Messages::ChatV4Presences(_pressence) => {}
            messages::Messages::ChatV5Participants(_) => (),
            messages::Messages::None => (),
            messages::Messages::ServiceMessage(message) => {
                if state.live_state == LiveState::Menu {
                    if let Some(id) = message.pre_match() {
                        let client = state.valorant_client.clone();
                        let full_auth = state.tokens.clone();
                        return Command::perform(
                            async move {
                                let prematch = api::pregame::get_match(&client, full_auth, id)
                                    .await
                                    .unwrap();

                                AsyncUpdate::PreMatch(prematch)
                            },
                            Message::AsyncUpdate,
                        );
                    }
                }
            }
        },
        Event::ErrSendingEvents => {
            state.websocket_status = "ErrSendingEvents".to_string();
            println!("ErrSendingEvents");
        }
        Event::SendEvents => {
            state.websocket_status = "SendEvents".to_string();
            println!("SendEvents");
        }
        Event::ValOpen => {
            state.websocket_status = "ValOpen".to_string();
            println!("ValOpen");
        }
        Event::WaitingForVal => {
            state.websocket_status = "WaitingForVal".to_string();
            println!("WaitingForVal");
        }
    }
    Command::none()
}
