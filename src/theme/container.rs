use iced::{color, widget::container, Color};

use super::Theme;

/*
 * Container
 */
#[derive(Clone, Copy, Default)]
pub enum Container {
    #[default]
    Transparent,
    Box,
    Friends,
    FriendsHeaders,
    Custom(fn(&Theme) -> container::Appearance),
}

impl From<fn(&Theme) -> container::Appearance> for Container {
    fn from(f: fn(&Theme) -> container::Appearance) -> Self {
        Self::Custom(f)
    }
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Transparent => Default::default(),
            Container::Box => container::Appearance {
                text_color: None,
                background: self.background.into(),
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::BLACK,
            },
            Container::Friends => container::Appearance {
                text_color: None,
                background: color!(0, 0, 0, 0.4).into(),
                border_radius: 1.0,
                border_width: 0.0,
                border_color: Color::BLACK,
            },
            Container::FriendsHeaders => container::Appearance {
                text_color: None,
                background: color!(255, 255, 200, 0.4).into(),
                border_radius: 1.0,
                border_width: 0.0,
                border_color: Color::BLACK,
            },
            Container::Custom(f) => f(self),
        }
    }
}
