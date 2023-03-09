use iced::widget::{container, text, row, column, Column};

use crate::state::LiveState;

pub fn live_page<'a>(state: &LiveState) -> crate::Element<'a> {
    match state {
        LiveState::Menu => {
    container(text("live page")).into()

        }
        LiveState::PreGame(game_data) => {

            let friendly_team = game_data.ally_team.players.iter().map(|player| text("ba").into()).collect();
            let enemy_team = column![];
            let teams = row![Column::with_children(friendly_team), enemy_team];
            container(teams).into()

        }
        LiveState::InGame => {
    container(text("live page")).into()

        }
    }
}
