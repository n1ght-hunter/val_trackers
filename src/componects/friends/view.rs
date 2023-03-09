use iced::widget;

use crate::{helpers::componet_trait::View, state::State, theme, Element};

use super::Friends;

impl View for Friends {
    fn view(state: &State) -> Element {
        let mut online_friends = state
            .friends
            .online_friends
            .iter()
            .map(|x| x.game_name.clone())
            .collect::<Vec<String>>();

        online_friends.sort();

        let mut online_friends = online_friends
            .into_iter()
            .map(|f| {
                widget::container(widget::text(f))
                    .width(iced::Length::Fill)
                    .center_x()
                    .center_y()
                    .padding([3, 0])
                    .style(theme::container::Container::Friends)
                    .into()
            })
            .collect::<Vec<Element>>();

        let mut offline_friends = state
            .friends
            .offline_friends
            .iter()
            .map(|x| x.game_name.clone())
            .collect::<Vec<String>>();

        offline_friends.sort();

        let mut offline_friends = offline_friends
            .into_iter()
            .map(|f| {
                widget::container(widget::text(f))
                    .width(iced::Length::Fill)
                    .center_x()
                    .center_y()
                    .padding([3, 0])
                    .style(theme::container::Container::Friends)
                    .into()
            })
            .collect::<Vec<Element>>();
        let mut friends_list = Vec::new();
        friends_list.push(
            widget::container(widget::text("Online friends"))
                .width(iced::Length::Fill)
                .center_x()
                .center_y()
                .padding([3, 0])
                .style(theme::container::Container::FriendsHeaders)
                .into(),
        );
        friends_list.append(&mut online_friends);

        friends_list.push(
            widget::container(widget::text("offline friends"))
                .width(iced::Length::Fill)
                .center_x()
                .center_y()
                .padding([3, 0])
                .style(theme::container::Container::FriendsHeaders)
                .into(),
        );
        friends_list.append(&mut offline_friends);

        let all = widget::scrollable(widget::Column::with_children(friends_list));

        super::widget::ShowFriends::new(all).into()
    }
}
