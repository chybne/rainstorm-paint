pub mod canvas_input;

use canvas::Canvas;
use canvas_input::CanvasInput;
use std::sync::{Arc, Mutex};
use tauri::Window;
use tauri_plugin_canvas::AppHandleExt;

#[tauri::command]
pub fn process_canvas_input(
    input: CanvasInput,
    canvas: tauri::State<Arc<Mutex<Canvas>>>,
    app: tauri::AppHandle,
    window: Window,
) {
    let mut canvas = canvas.lock().unwrap();

    println!("Received {input} input");

    match input {
        CanvasInput::ZoomCanvas { zoom } => handle_zoom(zoom, &mut canvas),
        CanvasInput::PanCanvas { offset_x, offset_y } => {
            handle_pan(offset_x, offset_y, &mut canvas)
        }
        _ => {}
    }

    app.send_redraw_request_for_window(window.label()).ok();
}

fn handle_zoom(zoom: f32, canvas: &mut Canvas) {
    canvas.set_zoom(zoom);
}

fn handle_pan(offset_x: f32, offset_y: f32, canvas: &mut Canvas) {
    canvas.set_offset(offset_x, offset_y);
}
