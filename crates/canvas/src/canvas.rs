use std::ops::Sub;

use glam::{self, Vec2, Vec3};

#[derive(Default, Debug, Clone)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<Point> for Vec2 {
    fn from(value: Point) -> Self {
        Vec2::new(value.x, value.y)
    }
}

impl From<Vec2> for Point {
    fn from(value: Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl Bounds {
    pub fn contains(&self, point: Point) -> bool {
        self.x <= point.x
            && point.x < self.x + self.width
            && self.y <= point.y
            && point.y < self.y + self.height
    }
}

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
    zoom: f32,
    bounds: Bounds,
    offset: Point,
}

impl Default for Canvas {
    fn default() -> Self {
        let size = 500 * 500 * 4;
        let pattern = [51, 153, 255, 255];

        let pixels: Vec<u8> = pattern.iter().cycle().take(size).cloned().collect();
        let zoom = 1.0;

        Self {
            width: 500,
            height: 500,
            pixels,
            zoom,
            bounds: Bounds::default(),
            offset: Point::default(),
        }
    }
}

impl Canvas {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixels(&self) -> &Vec<u8> {
        &self.pixels
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32) {
        let position = ((y * self.width as u32 + x) * 4) as usize;

        for i in 0..3 {
            self.pixels[position + i] = 0;
        }
    }

    pub fn zoom_relative_to_point(&mut self, delta: f32, mouse_pos: Point) {
        let offset: Vec2 = self.offset.into();
        let mouse_pos: Vec2 = mouse_pos.into();
        let world_before: Vec2 = (mouse_pos - offset) / self.zoom();

        self.zoom *= delta;
        /*
         * TODO define a min/max zoom const
         */
        self.zoom = self.zoom.clamp(0.3, 5.0);

        self.offset = (mouse_pos - world_before * self.zoom()).into();
    }

    pub fn set_zoom_temp(&mut self, zoom: f32) {
        self.zoom = zoom;
    }

    pub fn apply_offset(&mut self, dx: f32, dy: f32) {
        self.offset.x += dx;
        self.offset.y += dy;
    }

    pub fn inverse_matrix(&self) -> [[f32; 4]; 4] {
        glam::Mat4::from_cols_array_2d(&self.transform_matrix())
            .inverse()
            .to_cols_array_2d()
    }

    pub fn transform_matrix(&self) -> [[f32; 4]; 4] {
        let translation =
            glam::Mat4::from_translation(Vec3::new(self.offset.x, self.offset.y, 0.0));
        let scale = glam::Mat4::from_scale(Vec3::new(self.zoom, self.zoom, 1.0));

        (translation * scale).to_cols_array_2d()
    }

    pub fn is_within_bounds(&self, point: Point) -> bool {
        self.bounds.contains(point)
    }

    pub fn set_bounds(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.bounds = Bounds {
            x,
            y,
            width,
            height,
        };
    }

    pub fn convert_to_canvas_coords(&self, point: Point) -> Option<Point> {
        if !self.bounds.contains(point) {
            return None;
        }

        Some(Point {
            x: point.x - self.bounds.x,
            y: point.y - self.bounds.y,
        })
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
