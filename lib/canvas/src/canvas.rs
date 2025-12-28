use glam::{self, Vec2, Vec3};

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
    zoom: f32,
    rotation: f32,
    offset: Point,
}

impl Default for Canvas {
    fn default() -> Self {
        const WIDTH: usize = 500;
        const HEIGHT: usize = 500;

        let size = 500 * 500 * 4;
        let pattern = [97, 152, 219, 255];

        let pixels: Vec<u8> = pattern.iter().cycle().take(size).cloned().collect();
        let zoom = 1.0;
        let rotation = 0.0;
        let offset = Point { x: 0.0, y: 0.0 };

        Self {
            width: WIDTH,
            height: HEIGHT,
            pixels,
            zoom,
            rotation,
            offset,
        }
    }
}

impl Canvas {
    /// Creates a new canvas with specified width and height
    pub fn new(width: usize, height: usize) -> Self {
        let pattern = [97, 152, 219, 255];
        let pixels: Vec<u8> = pattern
            .iter()
            .cycle()
            .take(width * height * 4)
            .cloned()
            .collect();
        let zoom = 1.0;
        let rotation = 0.0;
        let offset = Point { x: 0.0, y: 0.0 };

        Self {
            width,
            height,
            pixels,
            zoom,
            rotation,
            offset,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn pixels(&self) -> &Vec<u8> {
        &self.pixels
    }

    pub fn apply_offset(&mut self, dx: f32, dy: f32) {
        self.offset.x += dx;
        self.offset.y += dy;
    }

    pub fn set_offset(&mut self, offset_x: f32, offset_y: f32) {
        self.offset.x = offset_x;
        self.offset.y = offset_y;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }

    /// creates a tranformation matrix for use to upload to the gpu
    pub fn transform_matrix(&self) -> [[f32; 4]; 4] {
        let translation =
            glam::Mat4::from_translation(Vec3::new(-self.offset.x, -self.offset.y, 0.0));
        let scale = glam::Mat4::from_scale(Vec3::new(self.zoom, self.zoom, 1.0));
        let rotation = glam::Mat4::from_rotation_z(self.rotation);
        let pivot = glam::Mat4::from_translation(Vec3::new(
            self.width as f32 / 2.0,
            self.height as f32 / 2.0,
            0.0,
        ));

        (translation * scale * pivot * rotation * pivot.inverse()).to_cols_array_2d()
    }

    #[deprecated = "moved logic to frontend"]
    pub fn zoom_relative_to_point(&mut self, zoom: f32, mouse_x: f32, mouse_y: f32) {
        let offset: Vec2 = Vec2::new(self.offset.x, self.offset.y);

        let mouse_pos: Vec2 = Vec2::new(mouse_x, mouse_y);
        let world_before: Vec2 = (mouse_pos + offset) / self.zoom;

        /* zoom should be clamped in the front end */
        self.zoom = zoom;

        let new_coords = world_before * self.zoom - mouse_pos;
        self.offset = Point {
            x: new_coords.x,
            y: new_coords.y,
        };
    }
}

/* point type to store canvas location on  */
pub struct Point<U = f32> {
    pub x: U,
    pub y: U,
}
