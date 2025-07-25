mod canvas_pipeline;

use canvas::Canvas;
use canvas_pipeline::CanvasPipeline;

use iced::advanced::{Widget, layout::Node, renderer};
use iced::border::radius;
use iced::widget::{Action, shader};
use iced::{self, Border, Color, Element, Length, Size, wgpu};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct CanvasWidget {
    canvas: Arc<RwLock<Canvas>>,
    canvas_id: u32,
}

impl CanvasWidget {
    pub fn new(canvas: Arc<RwLock<Canvas>>, canvas_id: u32) -> Self {
        let x = Self { canvas, canvas_id };
        x
    }
}

/* Bare bones basics for now */
impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for CanvasWidget
where
    Renderer: renderer::Renderer,
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
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border {
                    radius: radius(30),
                    ..Default::default()
                },
                shadow: Default::default(),
                snap: true,
            },
            Color::BLACK,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<CanvasWidget> for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(value: CanvasWidget) -> Self {
        Self::new(value)
    }
}

#[derive(Default)]
pub struct CanvasState {
    is_painting: bool,
}

impl CanvasState {
    pub fn set_is_painting(&mut self, is_painting: bool) {
        self.is_painting = is_painting;
    }
}

/* temp struct */
#[derive(Debug)]
struct CanvasPosition {
    x: u32,
    y: u32,
}

impl<Message> shader::Program<Message> for CanvasWidget {
    type State = CanvasState;
    type Primitive = CanvasPrimitive;
    fn draw(
        &self,
        _state: &Self::State,
        _cursor: iced::mouse::Cursor,
        _bounds: iced::Rectangle,
    ) -> Self::Primitive {
        CanvasPrimitive::new(self.canvas.clone(), self.canvas_id)
    }
    fn update(
        &self,
        state: &mut Self::State,
        event: &iced::Event,
        bounds: iced::Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> Option<iced::widget::Action<Message>> {
        // println!("{event:?}");
        use iced::Event;
        use iced::mouse;

        let canvas = self.canvas.read().unwrap();
        /* Not the actual canvas position, canvas width and widget with are not linked */
        let canvas_position: Option<CanvasPosition> = match cursor {
            mouse::Cursor::Available(point) => {
                let (mut x, mut y) = (point.x - bounds.x, point.y - bounds.y);

                if bounds.contains(point) {
                    let translated_points =
                        glam::Mat4::from_cols_array_2d(&canvas.inverse_matrix())
                            * glam::Vec4::new(x, y, 0.0, 1.0);
                    x = translated_points.x;
                    y = translated_points.y;
                }

                if x > 0.0 && x < canvas.width() as f32 && y > 0.0 && y < canvas.height() as f32 {
                    Some(CanvasPosition {
                        x: x as u32,
                        y: y as u32,
                    })
                } else {
                    None
                }
            }
            mouse::Cursor::Unavailable => None,
            mouse::Cursor::Levitating(_) => None,
        };
        drop(canvas);

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                // println!("mouse is pressed");

                if let Some(position) = canvas_position {
                    state.set_is_painting(true);
                    let mut canvas_mut = self.canvas.write().unwrap();
                    canvas_mut.draw_pixel(position.x, position.y);
                }
                Some(Action::request_redraw())
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                // println!("mouse is released");
                state.set_is_painting(false);
                Some(Action::request_redraw())
            }
            Event::Mouse(mouse::Event::CursorMoved { position: _ }) if state.is_painting => {
                // println!("mouse moved to {position:?}");
                if let Some(position) = canvas_position {
                    let mut canvas_mut = self.canvas.write().unwrap();
                    canvas_mut.draw_pixel(position.x, position.y);
                }
                Some(Action::request_redraw())
            }
            Event::Window(_) => {
                let mut canvas = self.canvas.write().unwrap();
                canvas.set_bounds(bounds.x, bounds.y, bounds.width, bounds.height);
                Some(Action::request_redraw())
            }
            _ => None,
        }
    }
}

/* Implement a Primitive to render a Canvas Buffer on screen */

#[derive(Debug)]
pub struct CanvasPrimitive {
    canvas: Arc<RwLock<Canvas>>,
    canvas_id: u32,
}

impl CanvasPrimitive {
    fn new(canvas: Arc<RwLock<Canvas>>, canvas_id: u32) -> Self {
        Self { canvas, canvas_id }
    }
}

#[allow(unused_variables)]
impl iced::widget::shader::Primitive for CanvasPrimitive {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        storage: &mut shader::Storage,
        bounds: &iced::Rectangle,
        viewport: &shader::Viewport,
    ) {
        /*
         * still need to replace a canvas pipeline if the size changes
         */
        if !storage.has::<HashMap<u32, CanvasPipeline>>() {
            storage.store::<HashMap<u32, CanvasPipeline>>(HashMap::new());
        }

        let canvases = storage.get_mut::<HashMap<u32, CanvasPipeline>>().unwrap();

        let mut pipeline = canvases
            .entry(self.canvas_id.clone())
            .or_insert_with(|| CanvasPipeline::new(device, queue, format, &self.canvas, bounds));

        if pipeline.bounds != *bounds {
            canvases.insert(
                self.canvas_id,
                CanvasPipeline::new(device, queue, format, &self.canvas, bounds),
            );

            pipeline = canvases
                .get_mut(&self.canvas_id)
                .expect("the canvas should exist");
        }

        pipeline.update(device, queue, format, &self.canvas);
    }
    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        storage: &shader::Storage,
        target: &wgpu::TextureView,
        clip_bounds: &iced::Rectangle<u32>,
    ) {
        let canvases = storage.get::<HashMap<u32, CanvasPipeline>>().unwrap();

        let canvas_pipeline = canvases.get(&self.canvas_id).unwrap();

        canvas_pipeline.render(encoder, target, clip_bounds);
    }
}
