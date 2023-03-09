use api::pregame::PreMatch;
use iced::Command;

use crate::{view::Pages, Message, State};

#[derive(Debug, Clone)]
pub enum AsyncUpdate {
    PreMatch(PreMatch),
}

pub fn async_update(state: &mut State, event: AsyncUpdate) -> Command<Message> {
    match event {
        AsyncUpdate::PreMatch(prematch) => {
            state.page = Pages::Live(crate::state::LiveState::PreGame(prematch))
        }
    }
    Command::none()
}
