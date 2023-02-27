use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

pub async fn get_riot_install_locations() -> Vec<Games> {
    let file = tokio::fs::read(format!(
        "{}/Riot Games/RiotClientInstalls.json",
        std::env::var("PROGRAMDATA").unwrap()
    ))
    .await
    .unwrap();
    let data = serde_json::from_slice::<Root>(file.as_slice()).unwrap();
    let mut output = vec![Games::RiotClient(data.rc_live)];

    data.associated_client.into_iter().for_each(|(game, _)| {
        if game.contains("VALORANT") {
            output.push(Games::Valorant(format!("{}/VALORANT.exe", game)))
        } else if game.contains("League of Legends") {
            output.push(Games::LOL(format!("{}/LeagueClient.exe", game)))
        } else {
            output.push(Games::Other(game))
        }
    });
    output
}

#[derive(Debug, Clone)]
pub enum Games {
    Valorant(String),
    LOL(String),
    RiotClient(String),
    Other(String),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "associated_client")]
    pub associated_client: HashMap<String, String>,
    pub patchlines: Patchlines,
    #[serde(rename = "rc_default")]
    pub rc_default: String,
    #[serde(rename = "rc_live")]
    pub rc_live: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patchlines {
    #[serde(rename = "KeystoneFoundationLiveWin")]
    pub keystone_foundation_live_win: String,
}

#[tokio::test]
async fn test_riot_file() {
    let files = get_riot_install_locations().await;
    for game in files {
        println!("{:?}", game)
    }
}
