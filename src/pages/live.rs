use iced::widget::{column, container, row, text, Column};

use crate::state::LiveState;

pub fn live_page<'a>(state: &'a LiveState) -> crate::Element<'a> {
    match state {
        LiveState::Menu => container(text("live page")).into(),
        LiveState::PreGame(game_data) => {
            let friendly_team = game_data
                .ally_team
                .players
                .iter()
                .map(|player| {
                    row![
                        "puuid",
                        player.subject.as_str(),
                        "level",
                        text(player.player_identity.account_level)
                    ]
                    .into()
                })
                .collect();
            let teams = row![Column::with_children(friendly_team)];
            container(teams).into()
        }
        LiveState::InGame => container(text("image page")).into(),
    }
}
