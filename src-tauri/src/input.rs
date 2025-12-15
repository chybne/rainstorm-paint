pub mod canvas_input;

use canvas::Canvas;
use canvas_input::CanvasInput;
use std::sync::Mutex;

use crate::appstate::AppState;

#[tauri::command]
pub fn process_canvas_input(input: CanvasInput, appstate: tauri::State<Mutex<AppState>>) {
    let mut state = appstate.lock().unwrap_or_else(|poison| poison.into_inner());
    let Some(canvas) = state.canvas_mut() else {
        eprintln!("Received {:?} input, but no canvas was provided", input);
        return;
    };

    println!("Received {input} input");

    match input {
        CanvasInput::ZoomCanvas { zoom } => handle_zoom(zoom, canvas),
        CanvasInput::PanCanvas { offset_x, offset_y } => handle_pan(offset_x, offset_y, canvas),
        CanvasInput::StartStroke {
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
