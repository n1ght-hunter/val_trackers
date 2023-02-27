use iced::widget::{container, text, row, column};

use crate::state::LiveState;

pub fn live_page<'a>(state: &LiveState) -> crate::Element<'a> {
    match state {
        LiveState::Menu => {
    container(text("live page")).into()

        }
        LiveState::PreGame(_game_data) => {
            let friendly_team = column![];
            let enemy_team = column![];
            let teams = row![friendly_team, enemy_team];
            container(teams).into()

        }
        LiveState::InGame => {
    container(text("live page")).into()

        }
    }
}
