use iced::Command;

use crate::{state::State, Message};

#[derive(Clone, Debug)]
pub enum Event {
    Store(Vec<super::QuickShowStore>, Vec<super::state::Bundle>),
}

pub fn update(state: &mut State, event: Event) -> Command<Message> {
    match event {
        Event::Store(weapons, bundles) => {
            state.store.weapons = weapons;
            state.store.bundles = bundles;
        }
    };
    Command::none()
}
