use foundation::geometry::{Point, Rectangle};
use glam::{self, Vec2, Vec3};

type Bounds = Rectangle;

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
    zoom: f32,
    rotation: f32,
    bounds: Rectangle,
    offset: Point,
}

impl Default for Canvas {
    fn default() -> Self {
        let size = 500 * 500 * 4;
        let pattern = [51, 153, 255, 255];

        let pixels: Vec<u8> = pattern.iter().cycle().take(size).cloned().collect();
        let zoom = 1.0;
        let rotation = 0.0;

        Self {
            width: 500,
            height: 500,
            pixels,
            zoom,
            rotation,
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

        println!("old offset: {offset}");
        let mouse_pos: Vec2 = mouse_pos.into();
        let world_before: Vec2 = (mouse_pos + offset) / self.zoom;
        println!("world before: {world_before}");

        self.zoom *= delta;

        /*
         * TODO define a min/max zoom const
         */
        self.zoom = self.zoom.clamp(0.3, 5.0);

        self.offset = (world_before * self.zoom - mouse_pos).into();

        let offset: Vec2 = self.offset.into();
        println!("new offset: {offset}");
    }

    pub fn set_zoom_temp(&mut self, zoom: f32) {
        self.zoom = zoom;
    }

    pub fn apply_offset(&mut self, dx: f32, dy: f32) {
        self.offset.x -= dx;
        self.offset.y -= dy;

        // 0.9 is a magic but defines the amount of extra space for viewing
        let min_offset = Point {
            x: -0.9 * self.bounds.width,
            y: -0.9 * self.bounds.height,
        };
        let max_offset = Point {
            x: self.width as f32 * self.zoom - self.bounds.width + self.bounds.width * 0.9,
            y: self.height as f32 * self.zoom - self.bounds.height + self.bounds.height * 0.9,
        };

        // let max_offset_y = self.height as f32 * self.zoom - self.bounds.height as f32;

        self.offset.x = self.offset.x.clamp(min_offset.x, max_offset.y);
        self.offset.y = self.offset.y.clamp(min_offset.y, max_offset.y);
        // println!("offset x: {} y: {}", self.offset.x, self.offset.y);
    }

    // rotate from center for now, will change later
    pub fn apply_rotation(&mut self, delta: f32, mouse_pos: Point) {
        self.rotation += delta;
    }

    pub fn inverse_matrix(&self) -> [[f32; 4]; 4] {
        glam::Mat4::from_cols_array_2d(&self.transform_matrix())
            .inverse()
            .to_cols_array_2d()
    }

    pub fn transform_matrix(&self) -> [[f32; 4]; 4] {
        let translation =
            glam::Mat4::from_translation(Vec3::new(-self.offset.x, -self.offset.y, 0.0));
        let scale = glam::Mat4::from_scale(Vec3::new(self.zoom, self.zoom, 1.0));
        let rotation = glam::Mat4::from_rotation_z(self.rotation);
        let pivot_rotate = glam::Mat4::from_translation(Vec3::new(
            self.width as f32 / 2.0,
            self.height as f32 / 2.0,
            0.0,
        ));

        (translation * scale * pivot_rotate * rotation * pivot_rotate.inverse()).to_cols_array_2d()
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
