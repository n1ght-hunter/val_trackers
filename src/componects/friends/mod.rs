pub mod state;
mod subscription;
mod update;
mod view;
pub mod widget;

use self::state::{OnlineFriends, AllFriends};

#[derive(Debug, Clone, Default)]
pub struct Friends {
    pub online_friends: Vec<OnlineFriends>,
    pub offline_friends: Vec<AllFriends>,
    pub all_friends: Vec<AllFriends>,
}