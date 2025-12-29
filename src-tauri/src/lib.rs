use tauri::{AppHandle, Manager, Window};

mod appstate;
mod event_handler;
use appstate::AppState;
use canvas::Canvas;
use input::{InputEvent, InputEventSink, InputSendError};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};
use tauri::async_runtime::Sender;

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

#[tauri::command]
fn set_view(offset_x: f32, offset_y: f32, app: AppHandle, window: tauri::Window) {
    let canvas = app.try_state::<Arc<Mutex<Canvas>>>();

    let Some(canvas) = canvas else {
        return;
    };
    let mut canvas = canvas.lock().unwrap();

    canvas.set_offset(offset_x, offset_y);
    app.send_redraw_request_for_window(window.label()).ok();
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

            let window = app
                .get_window("main")
                .expect("there should be a main window");

            #[cfg(target_os = "windows")]
            {
                window.set_decorations(false)?;
            }

            start_getting_input(window);

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
            event_handler::process_canvas_input,
            show_snap_overlay,
            set_view,
        ])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}

fn start_getting_input(window: Window) {
    let (send, mut recv) = tauri::async_runtime::channel::<InputEvent>(64);

    let send = InputEventSender(send);

    if let Err(e) = input::start_tracking_gestures(window, Box::new(send)) {
        println!("Error initialize input sender {e}");
    }

    tauri::async_runtime::spawn(async move {
        while let Some(event) = recv.recv().await {
            println!("{event:?}");
        }
    });
}

#[derive(Debug)]
struct InputEventSender(Sender<InputEvent>);
impl Deref for InputEventSender {
    type Target = Sender<InputEvent>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl InputEventSink for InputEventSender {
    fn try_send(&self, event: InputEvent) -> Result<(), InputSendError> {
        match self.0.try_send(event) {
            Ok(()) => Ok(()),
            Err(_) => Err(InputSendError::FailedSend),
        }
    }
}
