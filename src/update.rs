use iced::Command;


use crate::{
    componects::{friends, val_websocket, store},
    state::{HomePages, LiveState},
    subscriptions::lockfile::File,
    view::Pages,
    State,
};

#[derive(Clone, Debug)]
pub enum Recieved {
    // Match(Match),
    // Matchs(Matchs),
    // Profile(Profile),
}

#[derive(Clone, Debug)]
pub enum Message {
    Match(String),
    Matchs,
    Profile,
    UpdateUser(String),
    Receiver(Recieved),
    SetPage(Pages),
    Friends(friends::state::Friends),
    Store(store::Event),
    SetHomePage(HomePages),
    WebSocketEvent(val_websocket::Event),
    File(Option<String>),
    SetLiveState(LiveState),
}

pub fn update(state: &mut State, message: Message) -> iced::Command<Message> {
    match message {
        Message::Match(_id) => {
            // return Command::perform(get_match(id, state.client.clone()), |x| {
            //     Message::Receiver(Recieved::Match(x.unwrap()))
            // });
        }
        Message::Matchs => {
            // return Command::perform(
            //     get_matchs(
            //         state.current_user.name.clone(),
            //         state.current_user.tag.clone(),
            //         state.client.clone(),
            //     ),
            //     |x| Message::Receiver(Recieved::Matchs(x.unwrap())),
            // );
        }
        Message::Profile => {
            // return Command::perform(
            //     get_profile(
            //         state.current_user.name.clone(),
            //         state.current_user.tag.clone(),
            //         state.client.clone(),
            //     ),
            //     |x| Message::Receiver(Recieved::Profile(x.unwrap())),
            // );
        }
        Message::Receiver(item) => match item {
            // Recieved::Match(m) => state.match_info = Some(m),
            // Recieved::Matchs(matchs) => state.matchs = Some(matchs),
            // Recieved::Profile(profile) => state.profile = Some(profile),
        },
        Message::UpdateUser(message) => {
            state.input_text = message;
        }
        Message::SetPage(page) => state.page = page,
        Message::SetHomePage(page) => state.home_page = page,
        Message::WebSocketEvent(event) => return val_websocket::update(state, event),
        Message::File(f) => {
            if let Some(f) = f {
                let lock_vec = f.split(":").collect::<Vec<&str>>();

                state.lock_file.set_file(Some(File {
                    name: lock_vec[0].to_string(),
                    pid: lock_vec[1].to_string(),
                    port: lock_vec[2].to_string(),
                    password: lock_vec[3].to_string(),
                    protocol: lock_vec[4].to_string(),
                }));
            } else {
                state.lock_file.set_file(None);
            }
        }
        Message::SetLiveState(new_state) => state.live_state = new_state,
        Message::Friends(friends) => state.friends = friends,
        Message::Store(x) => return store::update(state, x),
    }
    Command::none()
}
