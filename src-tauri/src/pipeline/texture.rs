use super::vertex::Vertex;

use canvas::Canvas;
use std::sync::Mutex;
use wgpu;
use wgpu::util::DeviceExt;

#[allow(unused)]
pub struct CanvasTexture {
    pub size: wgpu::Extent3d,
    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub vertices: [Vertex; 6],
    pub diffuse_bind_group: wgpu::BindGroup,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub vertex_buffer: wgpu::Buffer,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
}

impl CanvasTexture {
    pub fn new(device: &wgpu::Device, canvas: &Mutex<Canvas>, w: f32, h: f32) -> Self {
        let canvas = canvas.lock().unwrap();

        let size = wgpu::Extent3d {
            width: canvas.width() as u32,
            height: canvas.height() as u32,
            ..Default::default()
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Canvas Rendering"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb, // might change to non srgb
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let width: f32 = canvas.width() as f32;
        let height: f32 = canvas.height() as f32;

        let vertices = [
            Vertex {
                // Top Right
                position: [width, 0.0], // 200, 0.0
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                // Top Left
                position: [0.0, 0.0], // 0.0, 0.0
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                // Bottom Left
                position: [0.0, height], // 0.0, 200
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                // Top Right
                position: [width, 0.0], // 200, 0.0
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                // Bottom Left
                position: [0.0, height], // 0.0, 200
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                // Bottom Right
                position: [width, height], // 200, 200
                tex_coords: [1.0, 1.0],
            },
        ];

        /* bind group stuff */
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
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("diffuse bind group"),
        });

        let ortho = glam::Mat4::orthographic_lh(0.0, w, h, 0.0, 0.0, 1.0);
        let trans_mat = glam::Mat4::IDENTITY;

        let uniforms = Uniforms {
            projection: ortho.to_cols_array_2d(),
            transformation: trans_mat.to_cols_array_2d(),
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

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            size,
            texture,
            texture_view,
            sampler,
            vertices,
            diffuse_bind_group,
            texture_bind_group_layout,
            vertex_buffer,
            uniform_buffer,
            uniform_bind_group,
            uniform_bind_group_layout,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, canvas: &Mutex<Canvas>) {
        let canvas = canvas.lock().unwrap();

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &canvas.pixels().as_slice(),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * canvas.width() as u32),
                rows_per_image: Some(canvas.height() as u32),
            },
            self.size,
        );
    }

    pub fn update_uniform(
        &mut self,
        queue: &wgpu::Queue,
        width: f32,
        height: f32,
        canvas: &Canvas,
    ) {
        let ortho = glam::Mat4::orthographic_lh(0.0, width, height, 0.0, 0.0, 1.0);
        let uniforms = Uniforms {
            projection: ortho.to_cols_array_2d(),
            transformation: canvas.transform_matrix(),
        };

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }
}

/* for later use */
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    projection: [[f32; 4]; 4],
    transformation: [[f32; 4]; 4],
}
