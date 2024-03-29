use iced::subscription;

use reqwest::Client;
use serde_derive::{Deserialize, Serialize};

use crate::{
    helpers::componet_trait::Subscription,
    subscriptions::lockfile::{File, LockFile},
    Message,
};

use super::{
    state::{AllFriends, OnlineFriends},
    Friends,
};

enum State {
    Start(LockFile, Client),
    Continue(LockFile, Client),
}

impl Subscription for Friends {
    type Params = ();

    fn subscription(state: &crate::State, params: Self::Params) -> iced::Subscription<Message> {
        subscription::unfold(
            "friends_list",
            State::Start(state.lock_file.clone(), state.non_secure_client.clone()),
            |state| async move {
                match state {
                    State::Start(lockfile, client) => {
                        let file = lockfile.get_file().await;
                        if let Some(file) = file {
                            let all_friends =
                                get_friends(&file, &client).await.unwrap_or_else(|err| {
                                    println!("{}", err);
                                    Vec::new()
                                });
                            let online_friends = get_online_friends(&all_friends, &file, &client)
                                .await
                                .unwrap_or_else(|err| {
                                    println!("{}", err);
                                    Vec::new()
                                });

                            let offline_friends = all_friends
                                .clone()
                                .into_iter()
                                .filter(|f| {
                                    online_friends.iter().any(|x| x.game_name != f.game_name)
                                })
                                .collect();

                            println!("onlinefreinds {:?}", online_friends);

                            return (
                                Some(Friends {
                                    online_friends,
                                    offline_friends,
                                    all_friends,
                                }),
                                State::Continue(lockfile, client),
                            );
                        }
                        (None, State::Continue(lockfile, client))
                    }
                    State::Continue(lockfile, client) => {
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        let file = lockfile.get_file().await;
                        if let Some(file) = file {
                            let all_friends =
                                get_friends(&file, &client).await.unwrap_or_else(|err| {
                                    println!("{}", err);
                                    Vec::new()
                                });
                            let online_friends = get_online_friends(&all_friends, &file, &client)
                                .await
                                .unwrap_or_else(|err| {
                                    println!("{}", err);
                                    Vec::new()
                                });

                            let offline_friends = all_friends
                                .clone()
                                .into_iter()
                                .filter(|all_friends| {
                                    !online_friends
                                        .iter()
                                        .any(|x| x.game_name == all_friends.game_name)
                                })
                                .collect();

                            return (
                                Some(Friends {
                                    online_friends,
                                    offline_friends,
                                    all_friends,
                                }),
                                State::Continue(lockfile, client),
                            );
                        }
                        (None, State::Continue(lockfile, client))
                    }
                }
            },
        )
        .map(Message::Friends)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendsSerialize {
    pub friends: Vec<AllFriends>,
}

async fn get_friends(lockfile: &File, client: &Client) -> Result<Vec<AllFriends>, reqwest::Error> {
    Ok(client
        .get(format!(
            "https://127.0.0.1:{}/chat/v4/friends",
            &lockfile.port
        ))
        .basic_auth("riot", Some(&lockfile.password))
        .send()
        .await?
        .json::<FriendsSerialize>()
        .await?
        .friends)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnlineFriendsSerialize {
    pub presences: Vec<OnlineFriends>,
}

async fn get_online_friends(
    all_friends: &Vec<AllFriends>,
    lockfile: &File,
    client: &Client,
) -> Result<Vec<OnlineFriends>, reqwest::Error> {
    let presences = client
        .get(format!(
            "https://127.0.0.1:{}/chat/v4/presences",
            &lockfile.port
        ))
        .basic_auth("riot", Some(&lockfile.password))
        .send()
        .await?
        .json::<OnlineFriendsSerialize>()
        .await?
        .presences;

    let online_friends = presences
        .into_iter()
        .filter(|x| all_friends.iter().any(|f| f.puuid == x.puuid))
        .collect();

    Ok(online_friends)
}
