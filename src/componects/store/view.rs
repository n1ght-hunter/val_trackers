use iced::widget::{self, text};

use crate::{
    helpers::componet_trait::View,
    theme::{self},
    Element, State,
};

use super::Store;

impl View for Store {
    fn view(state: &State) -> Element {
        let child: Vec<Element> = state
            .store
            .weapons
            .iter()
            .map(|store_offer| super::store_weapon::StoreWeapon::new(store_offer).into())
            .collect();

        let title = widget::container(widget::text("Store"))
            .width(iced::Length::Fill)
            .center_x()
            .center_y()
            .padding([3, 0])
            .style(theme::container::Container::FriendsHeaders);

        let bundle: Element = if state.store.bundles.len() > 0 {
            super::bundle::Bundle::new(&state.store.bundles[0]).into()
        } else {
            text("loading bundle").into()
        };

        widget::column![title, bundle, widget::Row::with_children(child)]
            .width(iced::Length::Units(320 * 4))
            .into()
    }
}
