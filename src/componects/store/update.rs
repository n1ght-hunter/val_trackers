use iced::Command;

use crate::{helpers::componet_trait::Update, state::State, Message};

use super::{Store, state::{QuickShowStore, Bundle}};

#[derive(Clone, Debug)]
pub enum Event {
    Store(Vec<QuickShowStore>, Vec<Bundle>),
}

impl Update for Store {
    type Message = Event;

    fn update(state: &mut State, event: Self::Message) -> Command<Message> {
        match event {
            Event::Store(weapons, bundles) => {
                state.store.weapons = weapons;
                state.store.bundles = bundles;
            }
        };
        Command::none()
    }
}
