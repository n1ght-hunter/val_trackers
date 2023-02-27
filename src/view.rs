use iced::widget::{self, *};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::{componects, pages::live::live_page, state::State, Element, Message};

#[derive(Clone, Debug, EnumIter, IntoStaticStr, Default)]
pub enum Pages {
    Home,
    Live,
    #[default]
    Store,
}

pub fn view(state: &State) -> Element {
    let display_user = row![text(format!(
        "current user is:{}#{}",
        state.current_user.name, state.current_user.tag
    ))];

    let menu = Row::with_children(
        Pages::iter()
            .map(|p| {
                let t: &'static str = p.clone().into();
                button(text(t)).on_press(Message::SetPage(p)).into()
            })
            .collect::<Vec<Element>>(),
    );

    let page: Element = match state.page {
        Pages::Home => {
            container(widget::column![display_user, menu])
                .center_x()
                .center_y()
                .into()
            // if let Some(profile) = &state.profile {
            //     if let Some(mat) = &state.matchs {
            //         home_page(profile, mat, &state.home_page)
            //     } else {
            //         col![].into()
            //     }
            // } else {
            //     col![].into()
            // }
        }
        Pages::Live => container(widget::column![
            widget::column![display_user, menu],
            live_page(&state.live_state)
        ])
        .center_x()
        .center_y()
        .into(),

        Pages::Store => container(widget::column![menu, componects::store::view(state),])
            .center_x()
            .center_y()
            .into(),
    };

    row![page, componects::friends::view(state)].into()
}
