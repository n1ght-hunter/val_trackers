pub mod lockfile;

use crate::{
    componects::{friends, store, val_websocket},
    helpers::componet_trait::Subscription,
    state::State,
    Message,
};

pub fn subscription(state: &State) -> iced::Subscription<Message> {
    iced::Subscription::batch([
        lockfile::get_lockfile(),
        val_websocket::subscription(&state.lock_file),
        friends::Friends::subscription(&state, ()),
        store::Store::subscription(&state, ()),
    ])
}
