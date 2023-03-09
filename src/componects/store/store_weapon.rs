use iced::{alignment, color, Color, ContentFit, Element, Rectangle, Size, Vector};
use iced_native::{layout, renderer::Quad, Widget};

use super::state::QuickShowStore;

pub struct StoreWeapon<'a> {
    store_offer: &'a QuickShowStore,
    width: iced::Length,
    height: iced::Length,
}

impl<'a> StoreWeapon<'a> {
    /// Returns a new [`Modal`]
    pub fn new(store_offer: &'a QuickShowStore) -> Self {
        Self {
            store_offer,
            width: iced::Length::Units(320),
            height: iced::Length::Units(160),
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for StoreWeapon<'a>
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
        let background_color = Quad {
            bounds: bounds,
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        };
        renderer.fill_quad(background_color, self.store_offer.background_color);

        let Size { width, height } = renderer.dimensions(&self.store_offer.background_icon);
        let image_size = Size::new(width as f32, height as f32);
        let adjusted_fit = ContentFit::Cover.fit(image_size, bounds.size());

        let offset = Vector::new(
            (bounds.width - adjusted_fit.width) / 2.0,
            (bounds.height - adjusted_fit.height) / 2.0,
        );

        let drawing_bounds = Rectangle {
            width: adjusted_fit.width,
            height: adjusted_fit.height,
            ..bounds
        };

        renderer.with_layer(bounds, |renderer: &mut Renderer| {
            renderer.draw(
                self.store_offer.background_icon.clone(),
                drawing_bounds + offset,
            );
        });

        let Size { width, height } = renderer.dimensions(&self.store_offer.gun_image);

        let image_size = Size::new(width as f32, height as f32);
        let adjusted_fit = ContentFit::Contain.fit(image_size, bounds.size());

        let adjusted_fit = Size::new(adjusted_fit.width * 0.9, adjusted_fit.height * 0.9);

        let offset = Vector::new(
            (bounds.width - adjusted_fit.width).max(0.0) / 2.0,
            (bounds.height - adjusted_fit.height).max(0.0) / 2.0,
        );

        let drawing_bounds = Rectangle {
            width: adjusted_fit.width,
            height: adjusted_fit.height,
            ..bounds
        };

        renderer.with_layer(bounds, |renderer: &mut Renderer| {
            let background_color = Quad {
                bounds: bounds,
                border_radius: 0.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            };

            renderer.fill_quad(background_color, color!(0, 0, 0, 0.5));

            renderer.draw(self.store_offer.gun_image.clone(), drawing_bounds + offset);

            let gun_name = iced_native::text::Text {
                content: &self.store_offer.weapon.display_name,
                bounds: Rectangle {
                    y: bounds.y + bounds.height,
                    ..bounds
                },
                size: f32::from(renderer.default_size()),
                color: Color::WHITE,
                font: Default::default(),
                horizontal_alignment: alignment::Horizontal::Left,
                vertical_alignment: alignment::Vertical::Bottom,
            };

            renderer.fill_text(gun_name);

            let price = iced_native::text::Text {
                content: &self
                    .store_offer
                    .store_item
                    .cost
                    .n85ad13f7_3d1b_5128_9eb2_7cd8ee0b5741
                    .to_string(),
                bounds: Rectangle {
                    x: bounds.x + bounds.width,
                    ..bounds
                },
                size: f32::from(renderer.default_size()),
                color: Color::WHITE,
                font: Default::default(),
                horizontal_alignment: alignment::Horizontal::Right,
                vertical_alignment: alignment::Vertical::Top,
            };

            renderer.fill_text(price);
        });
    }
}

impl<'a, Message, Renderer> From<StoreWeapon<'a>> for Element<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer
        + iced_native::image::Renderer<Handle = iced_native::image::Handle>
        + iced_native::text::Renderer,
{
    fn from(store_weapon: StoreWeapon<'a>) -> Element<'a, Message, Renderer> {
        Element::new(store_weapon)
    }
}
