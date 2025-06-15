mod color;

use color::Color;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub async fn new() -> Self {
        Canvas {
            width: 200,
            height: 200,
            pixels: vec![Color::new(1.0, 1.0, 1.0, 1.0); 200 * 200],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
