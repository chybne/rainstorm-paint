pub mod canvas_input;

use canvas::{brush::stroke::StrokeManager, Canvas};
use canvas_input::CanvasInput;
use std::sync::{Arc, Mutex};

use crate::appstate::AppState;

#[tauri::command]
pub fn process_canvas_input(input: CanvasInput, canvas: tauri::State<Arc<Mutex<Canvas>>>) {
    let mut canvas = canvas.lock().unwrap();

    println!("Received {input} input");

    match input {
        CanvasInput::ZoomCanvas { zoom } => handle_zoom(zoom, &mut canvas),
        CanvasInput::PanCanvas { offset_x, offset_y } => {
            handle_pan(offset_x, offset_y, &mut canvas)
        }
        CanvasInput::BeginStroke {
            mouse_x,
            mouse_y,
            pressure,
        } => {}
    }
}

fn handle_zoom(zoom: f32, canvas: &mut Canvas) {
    canvas.set_zoom(zoom);
}

fn handle_pan(offset_x: f32, offset_y: f32, canvas: &mut Canvas) {
    canvas.set_offset(offset_x, offset_y);
}

fn handle_begin_stroke(
    mouse_x: f32,
    mouse_y: f32,
    pressure: f32,
    stroke_manager: &mut StrokeManager,
) {
    stroke_manager.begin_stroke(mouse_x, mouse_y, pressure);
}
