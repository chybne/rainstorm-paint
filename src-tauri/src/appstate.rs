use canvas::Canvas;

pub struct AppState {
    canvas: Option<Canvas>,
}

impl AppState {
    pub fn set_canvas(&mut self, canvas: Canvas) {
        self.canvas = Some(canvas);
    }

    pub fn canvas(&self) -> &Option<Canvas> {
        &self.canvas
    }

    pub fn canvas_mut(&mut self) -> &mut Option<Canvas> {
        &mut self.canvas
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self { canvas: None }
    }
}
