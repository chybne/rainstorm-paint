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
fn attach_canvas(width: usize, height: usize, state: tauri::State<Mutex<AppState>>) {
    println!("dsdfjalkdkf");
    // let mut state = state.lock().unwrap();
    // let canvas = Canvas::new(width, height);
    // pipeline.attach_canvas(&canvas);
    // state.set_canvas(canvas);
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
        .setup(|app| {
            app.wry_plugin(CanvasPluginBuilder::new(app.handle().to_owned()));

            let canvas = Arc::new(Mutex::new(Canvas::default()));

            app.handle()
                .start_renderer_for_window("main", canvas.clone())
                .ok();

            app.manage(canvas);

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
    // .run(|app_handle, event| match event {
    //     RunEvent::WindowEvent {
    //         label: _,
    //         event: WindowEvent::Resized(size),
    //         ..
    //     } => {
    //         println!("resized");
    //         // let pipeline = app_handle.state::<Pipeline>();
    //         // let state = app_handle.state::<Mutex<AppState>>();
    //         // let state = state.lock().unwrap();
    //         // pipeline.change_size(size.width, size.height, state.canvas());

    //         // let canvas = app_handle.state::<Mutex<Canvas>>();
    //         // let canvas = canvas.lock().unwrap();
    //         // pipeline.change_size(size.width, size.height, &canvas);
    //     }
    //     RunEvent::MainEventsCleared => {
    //         println!("hi");
    //         // let pipeline = app_handle.state::<Pipeline>();
    //         // let state = app_handle.state::<Mutex<AppState>>();
    //         // let state = state.lock().unwrap();
    //         // if let Some(c) = state.canvas() {
    //         //     pipeline.update(c);
    //         // }

    //         // // let canvas = app_handle.state::<Mutex<Canvas>>();
    //         // // pipeline.update(&canvas);
    //         // pipeline.render();
    //     }
    //     _ => (),
    // });
}
