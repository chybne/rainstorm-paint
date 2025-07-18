use super::Vertex;
use canvas::Canvas;
use iced::wgpu;
use std::sync::{Arc, RwLock};

pub struct CanvasTexture {
    pub size: wgpu::Extent3d,
    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub vertices: [Vertex; 6],
}

impl CanvasTexture {
    pub fn new(device: &wgpu::Device, canvas: &Arc<RwLock<Canvas>>) -> Self {
        let canvas = canvas.read().unwrap();

        let size = wgpu::Extent3d {
            width: canvas.width() as u32,
            height: canvas.height() as u32,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("Canvas rendering"),
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

        Self {
            size,
            texture,
            texture_view,
            sampler,
            vertices,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, canvas: &Arc<RwLock<Canvas>>) {
        let canvas = canvas.read().unwrap();

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
}
