use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;
mod plugin;
mod utils;

pub use error::{Error, Result};
pub use plugin::{AppHandleExt, Builder as CanvasPluginBuilder};

#[cfg(desktop)]
use desktop::Canvas;
#[cfg(mobile)]
use mobile::Canvas;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the canvas APIs.
pub trait CanvasExt<R: Runtime> {
    fn canvas(&self) -> &Canvas<R>;
}

impl<R: Runtime, T: Manager<R>> crate::CanvasExt<R> for T {
    fn canvas(&self) -> &Canvas<R> {
        self.state::<Canvas<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("canvas")
        .invoke_handler(tauri::generate_handler![commands::ping])
        .setup(|app, api| {
            #[cfg(mobile)]
            let canvas = mobile::init(app, api)?;
            #[cfg(desktop)]
            let canvas = desktop::init(app, api)?;
            app.manage(canvas);
            Ok(())
        })
        .build()
}
