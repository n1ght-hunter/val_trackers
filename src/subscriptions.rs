pub mod lockfile;

use crate::{
    componects::{friends, store, val_websocket},
    state::State,
    Message,
};

pub fn subscription(state: &State) -> iced::Subscription<Message> {
    iced::Subscription::batch([
        lockfile::get_lockfile(),
        val_websocket::subscription(&state.lock_file),
        friends::subscription(&state.lock_file, &state.non_secure_client),
        store::subscription::subscription(
            &state.valorant_client,
            &state.tokens,
            &state.game_content,
        ),
    ])
}
