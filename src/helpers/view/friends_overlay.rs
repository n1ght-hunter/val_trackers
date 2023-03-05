use iced::Element;
use iced_native::{layout, widget::Tree, Widget};

pub struct FriendsOverlay<'a, Message, Renderer> {
    element: Element<'a, Message, Renderer>,
    friends: Element<'a, Message, Renderer>,
    width: iced::Length,
    height: iced::Length,
}

impl<'a, Message, Renderer> FriendsOverlay<'a, Message, Renderer> {
    /// Returns a new [`Modal`]
    pub fn new(
        element: impl Into<Element<'a, Message, Renderer>>,
        friends: impl Into<Element<'a, Message, Renderer>>,
    ) -> Self {
        Self {
            element: element.into(),
            friends: friends.into(),
            width: iced::Length::Fill,
            height: iced::Length::Fill,
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for FriendsOverlay<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::svg::Renderer,
{
    fn width(&self) -> iced::Length {
        self.width
    }

    fn height(&self) -> iced::Length {
        self.height
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.element), Tree::new(&self.friends)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&vec![&self.element, &self.friends])
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let limits = limits.width(self.width).height(self.height);

        let content = self.element.as_widget().layout(renderer, &limits);
        let size = limits.resolve(content.size());

        let mut friends = self.friends.as_widget().layout(renderer, &limits);

        let friedns_size = limits.resolve(friends.size());

        friends.align(iced::Alignment::End, iced::Alignment::Fill, friedns_size);

        layout::Node::with_children(size, vec![content, friends])
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
        let mut children_layout = layout.children();
        self.element.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            children_layout.next().unwrap(),
            cursor_position,
            renderer,
            clipboard,
            shell,
        );
        self.friends.as_widget_mut().on_event(
            &mut tree.children[1],
            event,
            children_layout.next().unwrap(),
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
        viewport: &iced::Rectangle,
    ) {
        let bounds = layout.bounds();
        let mut children_layout = layout.children();
        self.element.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            children_layout.next().unwrap(),
            cursor_position,
            viewport,
        );

        let friend_layout = children_layout.next().unwrap();

        renderer.with_layer(bounds, |renderer| {
            self.friends.as_widget().draw(
                &tree.children[1],
                renderer,
                theme,
                style,
                friend_layout,
                cursor_position,
                viewport,
            );
        });
    }
}

impl<'a, Message, Renderer> From<FriendsOverlay<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::svg::Renderer + 'a,
    Message: 'a,
{
    fn from(store_weapon: FriendsOverlay<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(store_weapon)
    }
}
