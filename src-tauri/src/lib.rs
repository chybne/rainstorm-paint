use tauri::{Manager, RunEvent, WindowEvent};

mod appstate;
mod pipeline;
use appstate::AppState;
use canvas::Canvas;
use pipeline::Pipeline;
use std::sync::Mutex;

#[tauri::command]
fn canvas_pan(dx: f32, dy: f32, state: tauri::State<Mutex<AppState>>) {
    // handle the unwraps my guy >:C
    let mut state = state.lock().unwrap();
    if let Some(c) = state.canvas_mut() {
        c.apply_offset(dx * 2f32, dy * 2f32);
    }
}

#[tauri::command]
fn canvas_zoom(zoom: f32, mouse_x: f32, mouse_y: f32, state: tauri::State<Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    if let Some(c) = state.canvas_mut() {
        c.zoom_relative_to_point(zoom, mouse_x, mouse_y);
    }
}

#[tauri::command]
fn attach_canvas(
    width: usize,
    height: usize,
    state: tauri::State<Mutex<AppState>>,
    pipeline: tauri::State<Pipeline>,
) {
    let mut state = state.lock().unwrap();
    let canvas = Canvas::new(width, height);
    pipeline.attach_canvas(&canvas);
    state.set_canvas(canvas);
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn set_view(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    state: tauri::State<Mutex<AppState>>,
    pipeline: tauri::State<Pipeline>,
) {
    let mut state = state.lock().unwrap();
    let canvas = state.canvas_mut();
    if let Some(c) = canvas {
        c.set_original_offset(x, y);
        pipeline.change_size(width as u32, height as u32, canvas);
    }

    // let mut canvas = state.lock().unwrap();
    // canvas.set_original_offset(x, y);

    // pipeline.change_size(width as u32, height as u32, &canvas);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

            let state = AppState::default();
            app.manage(Mutex::new(state));

            let pipeline = Pipeline::with_window(main_window)?;

            // let canvas = Mutex::new(Canvas::default());
            // pipeline.attach_canvas(&canvas);

            app.manage(pipeline);
            // app.manage(canvas);

            println!("Finished!");

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            set_view,
            attach_canvas,
            canvas_pan,
            canvas_zoom,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| match event {
            RunEvent::WindowEvent {
                label: _,
                event: WindowEvent::Resized(size),
                ..
            } => {
                let pipeline = app_handle.state::<Pipeline>();
                let state = app_handle.state::<Mutex<AppState>>();
                let state = state.lock().unwrap();
                pipeline.change_size(size.width, size.height, state.canvas());

                // let canvas = app_handle.state::<Mutex<Canvas>>();
                // let canvas = canvas.lock().unwrap();
                // pipeline.change_size(size.width, size.height, &canvas);
            }
            RunEvent::MainEventsCleared => {
                let pipeline = app_handle.state::<Pipeline>();
                let state = app_handle.state::<Mutex<AppState>>();
                let state = state.lock().unwrap();
                if let Some(c) = state.canvas() {
                    pipeline.update(c);
                }

                // let canvas = app_handle.state::<Mutex<Canvas>>();
                // pipeline.update(&canvas);
                pipeline.render();
            }
            _ => (),
        });
}
