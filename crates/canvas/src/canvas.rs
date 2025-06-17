#[allow(dead_code)]
#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl Default for Canvas {
    fn default() -> Self {
        let size = 200 * 200 * 4;
        let pattern = [51, 153, 255, 255];

        let pixels: Vec<u8> = pattern.iter().cycle().take(size).cloned().collect();

        Self {
            width: 200,
            height: 200,
            pixels,
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
}

#[cfg(test)]
mod tests {
    // use super::*;
}
