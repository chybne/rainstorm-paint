use tauri::{Manager, RunEvent, WindowEvent};

mod pipeline;
use pipeline::Pipeline;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

            let pipeline = Pipeline::with_window(main_window)?;
            app.manage(pipeline);

            println!("Finished!");

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| match event {
            RunEvent::WindowEvent {
                label: _,
                event: WindowEvent::Resized(size),
                ..
            } => {
                let pipeline = app_handle.state::<Pipeline>();
                pipeline.change_size(size.width, size.height);
            }
            RunEvent::MainEventsCleared => {
                let pipeline = app_handle.state::<Pipeline>();
                pipeline.render();
            }
            _ => (),
        });
}
