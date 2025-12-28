use tauri::Manager;

mod appstate;
mod input;
use appstate::AppState;
use canvas::Canvas;
use std::sync::{Arc, Mutex};

use tauri_plugin_canvas::{AppHandleExt, CanvasPluginBuilder};

#[tauri::command]
async fn show_snap_overlay() {
    #[cfg(target_os = "windows")]
    {
        use enigo::{Direction, Enigo, Key, Keyboard, Settings};

        // press win + z using enigo
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        enigo.key(Key::Meta, Direction::Press).ok();
        enigo.key(Key::Z, Direction::Click).ok();
        enigo.key(Key::Meta, Direction::Release).ok();

        // Wait 50 ms
        std::thread::sleep(std::time::Duration::from_millis(50));

        // Press Alt to hide the ugly numbers
        enigo.key(Key::Alt, Direction::Click).ok();
    }
}

#[tauri::command]
fn attach_canvas(width: usize, height: usize, app: tauri::AppHandle, window: tauri::Window) {
    let label = window.label();

    println!("{label} sent this call");
    let canvas = Arc::new(Mutex::new(Canvas::new(width, height)));
    app.attach_canvas_for_window(label, canvas.clone()).ok();
    app.send_redraw_request_for_window(label).ok();
    app.manage(canvas);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            /*
             * might want to write a plugin
             * to directly customize the titlebar on windows
             */
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
            attach_canvas,
            input::process_canvas_input,
            show_snap_overlay,
        ])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
