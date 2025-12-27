use tauri::{Manager, RunEvent, Window, WindowEvent};

mod appstate;
mod input;
mod pipeline;
use appstate::AppState;
use canvas::Canvas;
use pipeline::Pipeline;
use std::sync::{Arc, Mutex};

use tauri_plugin_canvas::{AppHandleExt, CanvasPluginBuilder};

#[tauri::command]
fn attach_canvas(width: usize, height: usize, app: tauri::AppHandle, window: tauri::Window) {
    let label = window.label();

    println!("{label} sent this call");
    let canvas = Arc::new(Mutex::new(Canvas::new(width, height)));
    app.attach_canvas_for_window(label, canvas.clone()).ok();
    app.manage(canvas);
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
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            #[cfg(target_os = "windows")]
            {
                let window = app
                    .get_window("main")
                    .expect("there should be a main window");

                window.set_decorations(false)?;
            }

            app.wry_plugin(CanvasPluginBuilder::new(app.handle().to_owned()));

            app.handle().start_renderer_for_window("main").ok();

            let state = AppState::default();
            app.manage(Mutex::new(state));

            // let pipeline = Pipeline::with_window(main_window)?;

            // let canvas = Mutex::new(Canvas::default());
            // pipeline.attach_canvas(&canvas);

            // app.manage(pipeline);
            // app.manage(canvas);

            println!("Finished!");

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            set_view,
            attach_canvas,
            input::process_canvas_input,
        ])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
