use iced::{widget::scrollable, Color};

use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum Scrollable {
    #[default]
    Default,
}

impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, style: &Self::Style) -> scrollable::Scrollbar {
        match style {
            Scrollable::Default => scrollable::Scrollbar {
                background: self.background.into(),
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                scroller: scrollable::Scroller {
                    color: self.light_blue,
                    border_radius: 2.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> scrollable::Scrollbar {
        self.active(style)
    }
}
