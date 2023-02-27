use iced::{color, Color, ContentFit, Element, Rectangle, Size, Vector};
use iced_native::{layout, renderer::Quad, Widget};

pub struct Bundle<'a> {
    bundle: &'a super::state::Bundle,
    width: iced::Length,
    height: iced::Length,
}

impl<'a> Bundle<'a> {
    /// Returns a new [`Modal`]
    pub fn new(bundle: &'a super::state::Bundle) -> Self {
        Self {
            bundle,
            width: iced::Length::Fill,
            height: iced::Length::Fill,
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Bundle<'a>
where
    Renderer: iced_native::Renderer
        + iced_native::image::Renderer<Handle = iced_native::image::Handle>
        + iced_native::text::Renderer,
{
    fn width(&self) -> iced::Length {
        self.width
    }

    fn height(&self) -> iced::Length {
        self.height
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let limits = limits.width(self.width).height(self.height);

        let size = limits.resolve(Size::ZERO);

        layout::Node::new(size)
    }

    fn draw(
        &self,
        _state: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        _theme: &<Renderer as iced_native::Renderer>::Theme,
        _style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        _cursor_position: iced::Point,
        _viewport: &iced::Rectangle,
    ) {
        let bounds = layout.bounds();
        let Size { width, height } = renderer.dimensions(&self.bundle.image);
        let image_size = Size::new(width as f32, height as f32);
        let adjusted_fit = ContentFit::ScaleDown.fit(
            image_size,
            Rectangle {
                x: bounds.x + 2.0,
                y: bounds.y + 2.0,
                width: bounds.width - 4.0,
                height: bounds.height - 4.0,
            }
            .size(),
        );

        let offset = Vector::new(
            (bounds.width - adjusted_fit.width) / 2.0,
            (bounds.height - adjusted_fit.height) / 2.0,
        );

        let drawing_bounds = Rectangle {
            width: adjusted_fit.width,
            height: adjusted_fit.height,
            ..bounds
        };

        let actual_bounds = drawing_bounds + offset;

        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x: actual_bounds.x - 2.0,
                    y: actual_bounds.y - 2.0,
                    width: actual_bounds.width + 4.0,
                    height: actual_bounds.height + 4.0,
                },
                border_radius: 0.0.into(),
                border_width: 2.0,
                border_color: color!(0xbbb8ba),
            },
            Color::TRANSPARENT,
        );

        renderer.draw(self.bundle.image.clone(), actual_bounds);
    }
}

impl<'a, Message, Renderer> From<Bundle<'a>> for Element<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer
        + iced_native::image::Renderer<Handle = iced_native::image::Handle>
        + iced_native::text::Renderer,
{
    fn from(store_weapon: Bundle<'a>) -> Element<'a, Message, Renderer> {
        Element::new(store_weapon)
    }
}
