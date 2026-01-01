pub mod canvas_input;

use canvas::{brush::stroke::StrokeManager, Canvas};
use canvas_input::CanvasInput;
use std::sync::{Arc, Mutex};
use tauri::Window;
use tauri_plugin_canvas::AppHandleExt;

#[tauri::command]
pub fn process_canvas_input(
    input: CanvasInput,
    canvas: tauri::State<Arc<Mutex<Canvas>>>,
    stroke_manager: tauri::State<Mutex<StrokeManager>>,
    app: tauri::AppHandle,
    window: Window,
) {
    let mut canvas = canvas.lock().unwrap();
    let mut stroke_manager = stroke_manager.lock().unwrap();
    // println!("Received {input} input");

    match input {
        CanvasInput::ZoomCanvas { zoom } => handle_zoom(zoom, &mut canvas),
        CanvasInput::PanCanvas { offset_x, offset_y } => {
            handle_pan(offset_x, offset_y, &mut canvas)
        }
        CanvasInput::BeginStroke(event) => {
            stroke_manager.begin_stroke(event.into(), &mut canvas);
        }
        CanvasInput::ContinueStroke(event) => {
            stroke_manager.continue_stroke(event.into(), &mut canvas);
        }
        CanvasInput::EndStroke(event) => {
            stroke_manager.end_stroke(event.into(), &mut canvas);
        }
    }

    app.send_redraw_request_for_window(window.label()).ok();
}

fn handle_zoom(zoom: f32, canvas: &mut Canvas) {
    canvas.set_zoom(zoom);
}

fn handle_pan(offset_x: f32, offset_y: f32, canvas: &mut Canvas) {
    canvas.set_offset(offset_x, offset_y);
}
