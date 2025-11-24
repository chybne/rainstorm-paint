mod texture;
mod vertex;

use std::borrow::Cow;
use std::sync::Mutex;

use canvas::Canvas;
use tauri::{async_runtime::block_on, Window};
use wgpu::{self, Device, Queue, RenderPipeline, Surface, SurfaceConfiguration};

use texture::CanvasTexture;
use vertex::Vertex;

pub struct Pipeline {
    surface: Surface<'static>,
    pipeline: Mutex<RenderPipeline>,
    device: Device,
    queue: Queue,
    config: Mutex<SurfaceConfiguration>,
    texture: Mutex<Option<CanvasTexture>>,
}

impl Pipeline {
    pub fn with_window(window: Window) -> Result<Pipeline, Box<dyn std::error::Error>> {
        let size = window.inner_size()?;

        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window)?;
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .expect("Failed to find an appropriate adapter");

        let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits:
                wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            memory_hints: wgpu::MemoryHints::Performance,
            trace: wgpu::Trace::Off,
        }))
        .expect("Failed to create device");

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
                r#"
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
let x = f32(i32(in_vertex_index) - 1);
let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    let srgb_color = vec3<f32>(0.380, 0.596, 0.859);
    let linear_color = pow(srgb_color, vec3<f32>(2.2));
    return vec4<f32>(linear_color, 1.0);
}
"#,
            )),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .unwrap_or(&swapchain_capabilities.formats[0])
            .clone();
        println!(
            " {:?} swapchain format: {:?}",
            swapchain_capabilities.formats, swapchain_format
        );

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(swapchain_format.into())],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Ok(Pipeline {
            surface,
            pipeline: Mutex::new(render_pipeline),
            device,
            queue,
            config: Mutex::new(config),
            texture: Mutex::new(None),
        })
    }

    pub fn attach_canvas(&self, canvas: &Canvas) {
        let config = self.config.lock().unwrap();

        let texture = CanvasTexture::new(
            &self.device,
            &canvas,
            config.width as f32,
            config.width as f32,
        );

        let shader = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Basic Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("./pipeline/temp.wgsl").into()),
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
        *pipeline = render_pipeline;
        let mut ctexture = self.texture.lock().unwrap();
        *ctexture = Some(texture);
    }

    pub fn change_size(&self, width: u32, height: u32, canvas: &Option<Canvas>) {
        let mut config = self.config.lock().unwrap();
        config.width = if width > 0 { width } else { 1 };
        config.height = if height > 0 { height } else { 1 };
        self.surface.configure(&self.device, &config);

        let canvas = if let Some(c) = canvas {
            c
        } else {
            return;
        };

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

        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
            ..Default::default()
        });

        let mut texture = self.texture.lock().unwrap();
        if let Some(t) = &mut *texture {
            let num_vertices = t.vertices.len() as u32;

            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
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
                                r: 0.9,
                                g: 0.9,
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
                rpass.set_vertex_buffer(0, t.vertex_buffer.slice(..));
                rpass.set_bind_group(0, &t.diffuse_bind_group, &[]);
                rpass.set_bind_group(1, &t.uniform_bind_group, &[]);
                rpass.draw(0..num_vertices, 0..1);
            }

            self.queue.submit(Some(encoder.finish()));
            frame.present();
        }
    }
}
