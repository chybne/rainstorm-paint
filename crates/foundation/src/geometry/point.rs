use glam::Vec2;
use std::ops::Sub;

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
