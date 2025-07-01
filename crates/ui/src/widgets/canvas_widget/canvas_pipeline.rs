mod canvas_texture;
mod vertex;

use canvas::Canvas;
use vertex::Vertex;

use bytemuck;
use glam;
use iced::widget::shader::wgpu::{self, util::DeviceExt};
use std::sync::{Arc, RwLock};

pub struct CanvasPipeline {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    diffuse_bind_group: wgpu::BindGroup,
    uniform_bind_group: wgpu::BindGroup,
    texture: canvas_texture::CanvasTexture,
    /* hacky solution might think of better way */
    pub bounds: iced::Rectangle,
}

impl CanvasPipeline {
    pub fn new(
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        canvas: &Arc<RwLock<Canvas>>,
        bounds: &iced::Rectangle,
    ) -> Self {
        let texture = canvas_texture::CanvasTexture::new(device, canvas);
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("Texture bind group layout"),
            });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: Some("diffuse bind group"),
        });

        /* Aspect Ratio and Tranformation */
        let ortho = glam::Mat4::orthographic_lh(0.0, bounds.width, bounds.height, 0.0, 0.0, 1.0);
        let trans_mat = canvas.read().unwrap().trans_matrix();

        println!("{trans_mat:?}");

        let uniforms = Uniforms {
            projection: ortho.to_cols_array_2d(),
            transformation: trans_mat,
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniform buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("uniform buffer layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        /* Render Pipeline stuff */
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("basicShader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./temp.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("thiis Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
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
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&texture.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let bounds = bounds.clone();

        Self {
            pipeline: render_pipeline,
            diffuse_bind_group,
            uniform_bind_group,
            vertex_buffer,
            uniform_buffer,
            texture,
            bounds,
        }
    }

    pub fn update(
        &mut self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _format: wgpu::TextureFormat,
        canvas: &Arc<RwLock<Canvas>>,
    ) {
        self.texture.update(queue, canvas);

        let canvas = canvas.write().unwrap();
        let offset = std::mem::size_of::<[[f32; 4]; 4]>();
        queue.write_buffer(
            &self.uniform_buffer,
            offset as u64,
            bytemuck::bytes_of(&canvas.trans_matrix()),
        );
    }

    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        clip_bounds: &iced::Rectangle<u32>,
    ) {
        let num_vertices = self.texture.vertices.len() as u32;

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

            pass.set_viewport(
                clip_bounds.x as f32,
                clip_bounds.y as f32,
                clip_bounds.width as f32,
                clip_bounds.height as f32,
                0.0,
                1.0,
            );
            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            pass.set_bind_group(1, &self.uniform_bind_group, &[]);
            pass.draw(0..num_vertices, 0..1);
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    projection: [[f32; 4]; 4],
    transformation: [[f32; 4]; 4],
}
