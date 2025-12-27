mod texture;
mod vertex;

use std::sync::Mutex;

use anyhow::Result;
use canvas::Canvas;
use tauri::{async_runtime::block_on, Window};
use wgpu::{
    Device, Queue, RenderPipeline, RequestAdapterOptions, Surface, SurfaceConfiguration,
    SurfaceTarget,
};

use crate::plugin::renderer::texture::CanvasTexture;

use vertex::Vertex;

/*
 * might want to move this to the canvas crate
*/

pub(crate) struct RenderState {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: Mutex<SurfaceConfiguration>,

    pipeline: Mutex<Option<RenderPipeline>>,
    texture: Mutex<Option<CanvasTexture>>,
}

impl RenderState {
    pub(crate) fn with_window(
        window: impl Into<SurfaceTarget<'static>>,
        width: u32,
        height: u32,
    ) -> Result<RenderState> {
        /* We can have a wgpu context that stores the instance
         * so we can have different settings such as using metal
         * or vulcan etc
         */
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window)?;

        /* TODO: remove the expect statements */
        let adapter = block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .expect("Failed to find an appropiate adapter");

        let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            memory_hints: wgpu::MemoryHints::Performance,
            trace: wgpu::Trace::Off,
        }))
        .expect("Failed to create Device");

        let surface_capabilities = surface.get_capabilities(&adapter);
        let texture_format = surface_capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .unwrap()
            .clone();
        println!(
            "{:?} texture format: {:?}",
            surface_capabilities.formats, texture_format
        );

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: texture_format,
            width,
            height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config: Mutex::new(config),
            pipeline: Mutex::new(None),
            texture: Mutex::new(None),
        })
    }

    pub fn attach_canvas(&self, canvas: &Canvas) {
        let config = self.config.lock().unwrap_or_else(|p| p.into_inner());

        let texture = CanvasTexture::new(
            &self.device,
            &canvas,
            config.width as f32,
            config.height as f32,
        );

        let shader = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Basic Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("./renderer/temp.wgsl").into()),
            });

        let render_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline layout"),
                    bind_group_layouts: &[
                        &texture.texture_bind_group_layout,
                        &texture.uniform_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });
        let render_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("thiis Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[Vertex::desc()],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw, // 2.
                    cull_mode: Some(wgpu::Face::Back),

                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            });

        let mut pipeline = self.pipeline.lock().unwrap();
        *pipeline = Some(render_pipeline);
        let mut ctexture = self.texture.lock().unwrap();
        *ctexture = Some(texture);
    }

    pub fn change_size(&self, width: u32, height: u32, canvas: &Canvas) {
        let mut config = self.config.lock().unwrap();
        config.width = if width > 0 { width } else { 1 };
        config.height = if height > 0 { height } else { 1 };
        self.surface.configure(&self.device, &config);

        let mut texture = self.texture.lock().unwrap();
        if let Some(c) = &mut *texture {
            c.update_uniform(&self.queue, width as f32, height as f32, canvas);
        }
    }

    pub fn update(&self, canvas: &Canvas) {
        let mut texture = self.texture.lock().unwrap();

        if let Some(c) = &mut *texture {
            c.update(&self.queue, canvas);
        }
    }

    pub fn render(&self) {
        let pipeline = self.pipeline.lock().unwrap();

        let Some(pipeline) = &*pipeline else {
            return;
        };

        let texture = self.texture.lock().unwrap();
        let Some(texture) = &*texture else {
            return;
        };

        let frame = self
            .surface
            .get_current_texture()
            .expect("Failure to acquire next swap chain texture");
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
            ..Default::default()
        });

        let num_vertices = texture.vertices.len() as u32;

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder :3"),
            });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Test View"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    depth_slice: None,
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.8,
                            g: 0.8,
                            b: 0.9,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.set_pipeline(&pipeline);
            rpass.set_vertex_buffer(0, texture.vertex_buffer.slice(..));
            rpass.set_bind_group(0, &texture.diffuse_bind_group, &[]);
            rpass.set_bind_group(1, &texture.uniform_bind_group, &[]);
            rpass.draw(0..num_vertices, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }
}
