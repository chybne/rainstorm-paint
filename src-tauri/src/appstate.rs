use canvas::brush::stroke::StrokeManager;

/// The current state of the application, right now it only holds the canvas
pub struct AppState {
    _stroke_manager: StrokeManager,
}

impl AppState {}

impl Default for AppState {
    fn default() -> Self {
        Self {
            _stroke_manager: StrokeManager::new(),
        }
    }
}
