use iced::widget::{column, container, row, text, Column, self};

use crate::{state::LiveState, State};

pub fn live_page<'a>(state: &'a State) -> crate::Element<'a> {
    let player_id = |puuid: &str| -> crate::Element {
        let name = state.id_to_name.get(puuid);
        if let Some((name, tag)) = name  {
            text(format!("{}:{}", name, tag)).into()
        } else {
            text(puuid).into()
        }
    };

    match &state.live_state {
        LiveState::Menu => container(text("live page")).into(),
        LiveState::PreGame(game_data) => {
            let friendly_team = game_data
                .ally_team
                .players
                .iter()
                .map(|player| {
                    row![
                        "Player ",
                        player_id(&player.subject),
                        " level ",
                        text(player.player_identity.account_level)
                    ]
                    .into()
                })
                .collect();
            let teams = row![Column::with_children(friendly_team)];
            container(widget::column!["PreamGame", teams]).into()
        }
        LiveState::InGame(match_data) => {
            let (friendly_team, enemy_team) = match_data.players.iter().fold(
                (Column::new(), Column::new()),
                |(friendly_team, enemy_team), player| {
                    if player.team_id == "Blue" {
                        let team = friendly_team.push(row![
                            "puuid ",
                            player_id(&player.subject),
                            " level ",
                            text(player.player_identity.account_level)
                        ]);
                        return (team, enemy_team);
                    } else if player.team_id == "Red" {
                        let team = enemy_team.push(row![
                            "puuid ",
                            player_id(&player.subject),
                            " level ",
                            text(player.player_identity.account_level)
                        ]);
                        return (friendly_team, team);
                    }

                    (friendly_team, enemy_team)
                },
            );
            let teams = row![friendly_team, enemy_team];
            container(widget::column!["Ingame",teams]).into()
        }
    }
}
