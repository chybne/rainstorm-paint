use glam::Vec2;

use crate::{Canvas, Color};

#[derive(Debug)]
pub struct StrokePositionalData {
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
}

pub struct StrokeManager {
    current_stroke: Option<Stroke>,
}

/// Really basic for now, but in the future this should handle
/// bounding boxes for strokes and handle undo redo states and such
impl StrokeManager {
    pub fn new() -> Self {
        Self {
            current_stroke: None,
        }
    }

    /// Begin recording positional data for current stroke
    pub fn begin_stroke(&mut self, point: StrokePositionalData, canvas: &mut Canvas) {
        let mut new_stroke = Stroke::new();

        println!("new stroke began at (x: {}, y: {})", point.x, point.y);
        let (x, y) = canvas.translate_screen_to_canvas(point.x, point.y);
        let point = StrokePositionalData { x, y, ..point };

        canvas.draw_pixel(
            x as usize,
            y as usize,
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
        );
        new_stroke.add_point(point);

        self.current_stroke = Some(new_stroke);
    }

    /// Add to the positional data
    pub fn continue_stroke(&mut self, point: StrokePositionalData, canvas: &mut Canvas) {
        let Some(stroke) = &mut self.current_stroke else {
            eprintln!("Stroke not initialized");
            return;
        };

        println!("continued stroke: (x: {}, y: {})", point.x, point.y);

        let (x, y) = canvas.translate_screen_to_canvas(point.x, point.y);
        let point = StrokePositionalData { x, y, ..point };
        let points = stroke.add_point(point);
        println!("points: {points:?}");

        for p in points.into_iter() {
            canvas.draw_pixel(
                p.x as usize,
                p.y as usize,
                Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                },
            );
        }
    }

    pub fn end_stroke(&mut self, point: StrokePositionalData, canvas: &mut Canvas) {
        println!("ended stroke: (x: {}, y: {})", point.x, point.y);

        let (x, y) = canvas.translate_screen_to_canvas(point.x, point.y);

        canvas.draw_pixel(
            x as usize,
            y as usize,
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
        );

        self.current_stroke = None;
    }
}

/// Representation of a brush stroke, storing all the raw input data that is received
struct Stroke {
    position_data: Vec<StrokePositionalData>,
    accumulated_distance: f32,
    spacing: f32,
}

impl Stroke {
    pub fn new() -> Self {
        Self {
            position_data: vec![],
            accumulated_distance: 0.0,
            spacing: 1.0,
        }
    }
    /// adds a new location point data to the end of the internally stored vec
    pub fn add_point(&mut self, point: StrokePositionalData) -> &[StrokePositionalData] {
        let last_point = self.position_data.last();
        let last_index = self.position_data.len();
        let Some(last_point) = last_point else {
            self.position_data.push(point);
            return &self.position_data[..];
        };
        let from = Vec2::new(last_point.x, last_point.y);
        let to = Vec2::new(point.x, point.y);
        let distance = from.distance(to);

        self.accumulated_distance += distance;

        if self.accumulated_distance < self.spacing {
            return &[];
        }

        let steps = (self.accumulated_distance / self.spacing).floor() as usize;

        for i in 0..steps {
            let from = from.lerp(to, i as f32 / steps as f32);

            let positional_data = StrokePositionalData {
                x: from.x,
                y: from.y,
                pressure: point.pressure,
            };
            self.position_data.push(positional_data);
            self.accumulated_distance -= self.spacing;
        }

        return &self.position_data[last_index..];
    }
}
