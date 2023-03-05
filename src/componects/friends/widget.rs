use iced::{
    color,
    mouse::{self, Button},
    Color, Element, Rectangle, Size,
};
use iced_native::{
    layout,
    widget::{tree, Tree},
    Widget,
};

use crate::theme::svg;

pub struct ShowFriends<'a, Message, Renderer> {
    element: Element<'a, Message, Renderer>,
    width: iced::Length,
    height: iced::Length,
}

impl<'a, Message, Renderer> ShowFriends<'a, Message, Renderer> {
    /// Returns a new [`Modal`]
    pub fn new(element: impl Into<Element<'a, Message, Renderer>>) -> Self {
        Self {
            element: element.into(),
            width: iced::Length::Units(200),
            height: iced::Length::Fill,
        }
    }
}

struct State {
    pub open: bool,
    pub open_svg: iced_native::svg::Handle,
    pub close_svg: iced_native::svg::Handle,
}

impl State {
    pub fn new() -> State {
        State { open: false, open_svg: iced_native::svg::Handle::from_memory( "<svg xmlns=\"http://www.w3.org/2000/svg\" fill-rule=\"evenodd\" clip-rule=\"evenodd\" viewBox=\"0 0 24 24\">
        <path d=\"M20 .76 5.63 12 20 23.22l-.62.78L4 12 19.4 0l.6.76z\"/>
        </svg>
        ".as_bytes()),
    close_svg:  iced_native::svg::Handle::from_memory("<svg xmlns=\"http://www.w3.org/2000/svg\" fill-rule=\"evenodd\" clip-rule=\"evenodd\" viewBox=\"0 0 24 24\">
    <path d=\"M4 .76 18.37 12 4 23.22l.62.78L20 12 4.6 0 4 .76z\"/>
  </svg>".as_bytes())
   }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for ShowFriends<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::svg::Renderer,
{
    fn width(&self) -> iced::Length {
        self.width
    }

    fn height(&self) -> iced::Length {
        self.height
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.element)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.element))
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let limits = limits.width(self.width).height(self.height);

        let content = self.element.as_widget().layout(renderer, &limits);
        let size = limits.resolve(content.size());

        layout::Node::with_children(size, vec![content])
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: iced::Event,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        renderer: &Renderer,
        clipboard: &mut dyn iced_native::Clipboard,
        shell: &mut iced_native::Shell<'_, Message>,
    ) -> iced::event::Status {
        if let iced::Event::Mouse(mouse::Event::ButtonReleased(Button::Left)) = event {
            let bounds = layout.bounds();
            let state = tree.state.downcast_mut::<State>();

            let bounds = if state.open {
                Rectangle {
                    width: 25.0,
                    height: 25.0,
                    x: bounds.x - 25.0,
                    y: bounds.y + 10.0,
                }
            } else {
                Rectangle {
                    width: 25.0,
                    height: 25.0,
                    x: bounds.x + bounds.width - 25.0,
                    y: bounds.y + 10.0,
                }
            };
            if bounds.contains(cursor_position) {
                state.open = !state.open;
            }
        }
        self.element.as_widget_mut().on_event(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap(),
            cursor_position,
            renderer,
            clipboard,
            shell,
        );
        iced::event::Status::Ignored
    }

    fn draw(
        &self,
        tree: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced_native::Renderer>::Theme,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        _viewport: &iced::Rectangle,
    ) {
        let bounds = layout.bounds();
        let state = tree.state.downcast_ref::<State>();
        let content_layout = layout.children().next().unwrap();

        let svg_bounds = if state.open {
            Rectangle {
                width: 25.0,
                height: 25.0,
                x: bounds.x - 25.0,
                y: bounds.y + 10.0,
            }
        } else {
            Rectangle {
                width: 25.0,
                height: 25.0,
                x: bounds.x + bounds.width - 25.0,
                y: bounds.y + 10.0,
            }
        };

        let outer_button = Rectangle {
            width: 30.0,
            height: 35.0,
            x: svg_bounds.x - 5.0,
            y: svg_bounds.y - 5.0,
        };

        renderer.fill_quad(
            iced_native::renderer::Quad {
                bounds: outer_button,
                border_radius: [50.0, 0.0, 0.0, 50.0].into(),
                border_width: 2.0,
                border_color: Color::WHITE.into(),
            },
            if outer_button.contains(cursor_position) {
                color!(0, 0, 0, 0.7).into()
            } else {
                Color::TRANSPARENT
            },
        );

        renderer.draw(
            if state.open {
                state.close_svg.clone()
            } else {
                state.open_svg.clone()
            },
            Color::WHITE.into(),
            svg_bounds,
        );
        if state.open {
            self.element.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                content_layout,
                cursor_position,
                &bounds,
            );
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> iced_native::mouse::Interaction {
        let bounds = layout.bounds();
        let state = tree.state.downcast_ref::<State>();

        let svg_bounds = if state.open {
            Rectangle {
                width: 25.0,
                height: 25.0,
                x: bounds.x - 25.0,
                y: bounds.y + 10.0,
            }
        } else {
            Rectangle {
                width: 25.0,
                height: 25.0,
                x: bounds.x + bounds.width - 25.0,
                y: bounds.y + 10.0,
            }
        };

        let outer_button = Rectangle {
            width: 30.0,
            height: 35.0,
            x: svg_bounds.x - 5.0,
            y: svg_bounds.y - 5.0,
        };

        let is_mouse_over = outer_button.contains(cursor_position);

        if is_mouse_over {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message, Renderer> From<ShowFriends<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::svg::Renderer + 'a,
    Message: 'a,
{
    fn from(store_weapon: ShowFriends<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(store_weapon)
    }
}
