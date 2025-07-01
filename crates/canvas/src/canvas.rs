use glam;

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
    zoom: glam::Mat4,
    trans_matrix: glam::Mat4,
    inverse_matrix: glam::Mat4,
}

impl Default for Canvas {
    fn default() -> Self {
        let size = 500 * 500 * 4;
        let pattern = [51, 153, 255, 255];

        let pixels: Vec<u8> = pattern.iter().cycle().take(size).cloned().collect();
        let trans_matrix = glam::Mat4::IDENTITY;
        let zoom = glam::Mat4::IDENTITY;
        let inverse_matrix = glam::Mat4::IDENTITY;

        Self {
            width: 500,
            height: 500,
            pixels,
            trans_matrix,
            inverse_matrix,
            zoom,
        }
    }
}

impl Canvas {
    pub async fn new() -> Self {
        Self::default()
    }

    pub fn width(&self) -> usize {
        self.width
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

    pub fn trans_matrix(&self) -> [[f32; 4]; 4] {
        self.trans_matrix.to_cols_array_2d()
    }

    pub fn inverse_matrix(&self) -> [[f32; 4]; 4] {
        self.inverse_matrix.to_cols_array_2d()
    }

    pub fn scale_matrix(&mut self, zoom_factor: f32) {
        if zoom_factor <= 0.0 {
            return;
        }
        self.zoom = glam::Mat4::from_scale(glam::vec3(zoom_factor, zoom_factor, 1.0));

        self.transform_matrix();
    }

    fn transform_matrix(&mut self) {
        self.trans_matrix = self.zoom;
        self.inverse_matrix = self.trans_matrix.inverse();
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
