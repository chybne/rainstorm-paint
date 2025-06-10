use iced::advanced::{Widget, layout::Node, renderer};
use iced::border::radius;
use iced::widget::shader::{self, wgpu};
use iced::{self, Border, Color, Element, Length, Size};
use iced_wgpu::wgpu::RenderPipeline;

#[derive(Debug, Copy, Clone)]
pub struct Canvas {}

impl Canvas {
    pub fn new() -> Self {
        Self {}
    }
}

/* Bare bones basics for now */
impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Canvas
where
    Renderer: renderer::Renderer + iced_core::text::Renderer,
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
            },
            Color::BLACK,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Canvas> for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + iced_core::text::Renderer,
{
    fn from(value: Canvas) -> Self {
        Self::new(value)
    }
}

impl<Message> shader::Program<Message> for Canvas {
    type State = ();
    type Primitive = CanvasPrimitive;
    fn draw(
        &self,
        _state: &Self::State,
        _cursor: iced_core::mouse::Cursor,
        bounds: iced::Rectangle,
    ) -> Self::Primitive {
        CanvasPrimitive::new(bounds)
    }
}

/* Implement a Primitive to render a Canvas Buffer on screen */

#[allow(dead_code)]
#[derive(Debug)]
pub struct CanvasPrimitive {
    bounds: iced::Rectangle,
}

impl CanvasPrimitive {
    fn new(bounds: iced::Rectangle) -> Self {
        Self { bounds }
    }
}

#[allow(unused_variables)]
impl iced::widget::shader::Primitive for CanvasPrimitive {
    fn prepare(
        &self,
        device: &iced_wgpu::wgpu::Device,
        queue: &iced_wgpu::wgpu::Queue,
        format: iced_wgpu::wgpu::TextureFormat,
        storage: &mut iced_wgpu::primitive::Storage,
        bounds: &iced::Rectangle,
        viewport: &iced_wgpu::graphics::Viewport,
    ) {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("basicShader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./temp.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        if !storage.has::<RenderPipeline>() {
            storage.store(render_pipeline)
        }
    }
    fn render(
        &self,
        encoder: &mut iced_wgpu::wgpu::CommandEncoder,
        storage: &iced_wgpu::primitive::Storage,
        target: &iced_wgpu::wgpu::TextureView,
        clip_bounds: &iced::Rectangle<u32>,
    ) {
        // println!("{clip_bounds:?}");

        let pipeline = storage.get::<RenderPipeline>().unwrap();

        {
            #[allow(unused_mut)]
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Test View"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_scissor_rect(
                clip_bounds.x,
                clip_bounds.y,
                clip_bounds.width,
                clip_bounds.height,
            );
            pass.set_pipeline(pipeline);
            pass.draw(0..3, 0..1);
        }
    }
}
