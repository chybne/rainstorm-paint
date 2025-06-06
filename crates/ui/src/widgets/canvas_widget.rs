use iced::advanced::{Widget, layout::Node, renderer};
use iced::{self, Border, Color, Element, Length, Size};
use iced::border::radius;

#[derive(Debug, Copy, Clone)]
pub struct Canvas {
    
}

impl Canvas {
    pub fn new() -> Self {

        Self {
        }
    }
}

/* Bare bones basics for now */
impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Canvas
    where
        Renderer: renderer::Renderer
    {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
            &self,
            _tree: &mut iced::advanced::widget::Tree,
            _renderer: &Renderer,
            _limits: &iced::advanced::layout::Limits,
        ) -> Node {
            Node::new(Size::new(30.0, 30.0))
    }

    fn draw(
            &self,
            _tree: &iced::advanced::widget::Tree,
            renderer: &mut Renderer,
            _theme: &Theme,
            _style: &iced::advanced::renderer::Style,
            layout: iced::advanced::Layout<'_>,
            _cursor: iced::advanced::mouse::Cursor,
            _viewport: &iced::Rectangle,
        ) {


        renderer.fill_quad(renderer::Quad {
            bounds: layout.bounds(),
            border: Border {
                radius : radius(30),
                ..Default::default()
            },
            shadow: Default::default()
        }, Color::BLACK);

        
    }

}

impl<'a, Message, Theme, Renderer> From<Canvas> for Element<'a, Message, Theme, Renderer>
    where
        Renderer: renderer::Renderer,
{
    fn from(value: Canvas) -> Self {
        Self::new(value)
    }
}
