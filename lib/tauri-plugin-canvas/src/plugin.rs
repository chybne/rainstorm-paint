mod renderer;

use crate::utils::*;
use anyhow::{Error, Result};
use canvas::Canvas;
use renderer::RenderState;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};
use tauri_runtime::UserEvent;
use tauri_runtime_wry::tao::event::Event;
use tauri_runtime_wry::tao::event::WindowEvent as TaoWindowEvent;
use tauri_runtime_wry::tao::event_loop::{EventLoopProxy, EventLoopWindowTarget};
use tauri_runtime_wry::tao::window::WindowId as TaoWindowId;
use tauri_runtime_wry::{Message, Plugin, PluginBuilder};

type CanvasWindowMap = Arc<Mutex<HashMap<String, CanvasWindow>>>;

pub struct Builder {
    app: AppHandle,
}

impl Builder {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }
}

impl<T: UserEvent> PluginBuilder<T> for Builder {
    type Plugin = CanvasRendererPlugin<T>;

    fn build(self, context: tauri_runtime_wry::Context<T>) -> Self::Plugin {
        let window_map: CanvasWindowMap = Arc::new(Mutex::new(HashMap::new()));
        self.app.manage(window_map.clone());
        CanvasRendererPlugin::new(window_map, context)
    }
}

pub struct CanvasRendererPlugin<T: UserEvent> {
    windows: CanvasWindowMap,
    _phantom: PhantomData<T>,
}

impl<T: UserEvent> CanvasRendererPlugin<T> {
    fn new(windows: CanvasWindowMap, _: tauri_runtime_wry::Context<T>) -> Self {
        Self {
            windows,
            _phantom: PhantomData,
        }
    }
}

impl<T: UserEvent> Plugin<T> for CanvasRendererPlugin<T> {
    fn on_event(
        &mut self,
        event: &Event<tauri_runtime_wry::Message<T>>,
        _event_loop: &EventLoopWindowTarget<tauri_runtime_wry::Message<T>>,
        proxy: &EventLoopProxy<tauri_runtime_wry::Message<T>>,
        _control_flow: &mut tauri_runtime_wry::tao::event_loop::ControlFlow,
        context: tauri_runtime_wry::EventLoopIterationContext<'_, T>,
        _web_context: &tauri_runtime_wry::WebContextStore,
    ) -> bool {
        let mut windows = self.windows.lock().unwrap();

        match event {
            Event::WindowEvent {
                window_id, event, ..
            } => {
                if let Some(label) = get_label_from_tao_id(window_id, &context) {
                    if let Some(canvas_win) = windows.get_mut(&label) {
                        /* hackiest of hacks but whateves */
                        canvas_win.tao_id = Some(window_id.clone());
                        match event {
                            TaoWindowEvent::Resized(size) => {
                                if let Some(canvas) = &canvas_win.canvas {
                                    let canvas = canvas.lock().unwrap();
                                    canvas_win.renderer.change_size(
                                        size.width,
                                        size.height,
                                        &canvas,
                                    );
                                    return false;
                                }
                            }
                            _ => {
                                let consumed = false;

                                let win_id = get_id_from_tao_id(window_id, &context);

                                // Request redraw after input events to process accumulated events
                                if let Some(id) = win_id {
                                    proxy
                                        .send_event(Message::Window(
                                            id,
                                            tauri_runtime_wry::WindowMessage::RequestRedraw,
                                        ))
                                        .ok();
                                }

                                // Request a redraw after any input event
                                return consumed;
                            }
                        }
                    }
                }
            }
            Event::RedrawRequested(window_id) => 'a: {
                let Some(label) = get_label_from_tao_id(window_id, &context) else {
                    break 'a;
                };

                let Some(canvas_win) = windows.get_mut(&label) else {
                    break 'a;
                };

                if let Some(canvas) = &canvas_win.canvas {
                    let canvas = canvas.lock().unwrap();
                    canvas_win.renderer.update(&*canvas);
                    canvas_win.renderer.render();
                };
            }
            &_ => {}
        }

        false
    }
}

struct CanvasWindow {
    tao_id: Option<TaoWindowId>,
    canvas: Option<Arc<Mutex<Canvas>>>,
    renderer: RenderState,
}

pub trait AppHandleExt {
    fn start_renderer_for_window(&self, label: &str) -> Result<()>;

    fn attach_canvas_for_window(&self, label: &str, canvas: Arc<Mutex<Canvas>>) -> Result<()>;

    fn send_redraw_request_for_window(&self, label: &str) -> Result<()>;
}
impl AppHandleExt for AppHandle {
    fn start_renderer_for_window(&self, label: &str) -> Result<()> {
        let canvas_windows = self
            .try_state::<CanvasWindowMap>()
            .ok_or(Error::msg("TauriPluginCanvasRenderer is not initialized"))?;

        let window = self
            .get_window(label)
            .ok_or(Error::msg("No window found with the provided label"))?;

        let size = window.inner_size()?;
        let renderer = RenderState::with_window(window, size.width, size.height)?;

        let mut managed_windows = canvas_windows.lock().unwrap();
        managed_windows.insert(
            label.to_string(),
            CanvasWindow {
                canvas: None,
                tao_id: None,
                renderer,
            },
        );

        Ok(())
    }

    fn attach_canvas_for_window(&self, label: &str, canvas: Arc<Mutex<Canvas>>) -> Result<()> {
        let canvas_windows = self
            .try_state::<CanvasWindowMap>()
            .ok_or(Error::msg("TauriPluginCanvasRenderer is not initialized"))?;

        let _ = self
            .get_window(label)
            .ok_or(Error::msg("No window found with the provided label"))?;

        let mut managed_windows = canvas_windows.lock().unwrap();

        let Some(window) = managed_windows.get_mut(label) else {
            return Err(Error::msg("Canvas renderer for window not initialized"));
        };

        window.canvas = Some(canvas.clone());
        let canvas = canvas.lock().unwrap();
        window.renderer.attach_canvas(&canvas);

        Ok(())
    }

    fn send_redraw_request_for_window(&self, label: &str) -> Result<()> {
        let canvas_windows = self
            .try_state::<CanvasWindowMap>()
            .ok_or(Error::msg("TauriPluginCanvasRenderer is not initialized"))?;

        let mut managed_windows = canvas_windows.lock().unwrap();

        let window = managed_windows
            .get_mut(&label.to_string())
            .ok_or(Error::msg("No window with label found"))?;

        if let Some(tid) = window.tao_id {
            self.send_tao_window_event(tid, tauri_runtime_wry::WindowMessage::RequestRedraw)
                .ok();
        }

        Ok(())
    }
}
