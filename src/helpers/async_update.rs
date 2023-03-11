use api::pregame::PreMatch;
use iced::Command;

use crate::{view::Pages, Message, State};

#[derive(Debug, Clone)]
pub enum AsyncUpdate {
    PreMatch(PreMatch),
    Match(api::current_game::Match),
}

pub fn async_update(state: &mut State, event: AsyncUpdate) -> Command<Message> {
    match event {
        AsyncUpdate::PreMatch(prematch) => {
            state.live_state = crate::state::LiveState::PreGame(prematch);
        }
        AsyncUpdate::Match(match_data) => {
            state.live_state = crate::state::LiveState::InGame(match_data);
        }
    }
    Command::none()
}
