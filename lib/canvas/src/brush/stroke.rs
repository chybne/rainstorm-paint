#[allow(dead_code)]
struct Point {
    x: f32,
    y: f32,
    pressure: f32,
}

pub struct StrokeManager {
    current_stroke: Option<Stroke>,
}

impl StrokeManager {
    pub fn new() -> Self {
        Self {
            current_stroke: None,
        }
    }

    pub fn begin_stroke(&mut self, x: f32, y: f32, pressure: f32) {
        let mut new_stroke = Stroke::new();
        let point = Point { x, y, pressure };
        new_stroke.add_point(point);

        println!("new stroke began at (x: {x}, y: {y})");

        self.current_stroke = Some(new_stroke);
    }

    pub fn continue_stroke(&mut self, x: f32, y: f32, pressure: f32) {
        let Some(stroke) = &mut self.current_stroke else {
            eprintln!("Stroke not initialized");
            return;
        };

        println!("continued stroke: (x: {x}, y: {y}");

        let point = Point { x, y, pressure };
        stroke.add_point(point);
    }

    pub fn end_stroke(&mut self) {
        self.current_stroke = None;
    }
}

/// Representation of a brush stroke, storing all the raw input data that is received
struct Stroke {
    position_data: Vec<Point>,
}

impl Stroke {
    pub fn new() -> Self {
        Self {
            position_data: vec![],
        }
    }
    /// adds a new location point data to the end of the internally stored vec
    pub fn add_point(&mut self, point: Point) {
        self.position_data.push(point);
    }
}
