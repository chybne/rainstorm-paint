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

    fn build(self, _: tauri_runtime_wry::Context<T>) -> Self::Plugin {
        let window_map: CanvasWindowMap = Arc::new(Mutex::new(HashMap::new()));
        self.app.manage(window_map.clone());
        CanvasRendererPlugin::new(window_map)
    }
}

pub struct CanvasRendererPlugin<T: UserEvent> {
    windows: CanvasWindowMap,
    _phantom: PhantomData<T>,
}

impl<T: UserEvent> CanvasRendererPlugin<T> {
    fn new(windows: CanvasWindowMap) -> Self {
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
        _event_loop: &tauri_runtime_wry::tao::event_loop::EventLoopWindowTarget<
            tauri_runtime_wry::Message<T>,
        >,
        proxy: &tauri_runtime_wry::tao::event_loop::EventLoopProxy<tauri_runtime_wry::Message<T>>,
        _control_flow: &mut tauri_runtime_wry::tao::event_loop::ControlFlow,
        context: tauri_runtime_wry::EventLoopIterationContext<'_, T>,
        _web_context: &tauri_runtime_wry::WebContextStore,
    ) -> bool {
        match event {
            Event::WindowEvent {
                window_id, event, ..
            } => {
                if let Some(label) = get_label_from_tao_id(window_id, &context) {
                    let mut windows = self.windows.lock().unwrap();
                    if let Some(canvas_win) = windows.get_mut(&label) {
                        match event {
                            TaoWindowEvent::Resized(size) => {
                                let canvas = canvas_win.canvas.lock().unwrap();
                                canvas_win
                                    .renderer
                                    .change_size(size.width, size.height, &canvas);
                                return false;
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
            Event::RedrawRequested(window_id) => {
                if let Some(label) = get_label_from_tao_id(window_id, &context) {
                    let mut windows = self.windows.lock().unwrap();
                    if let Some(canvas_win) = windows.get_mut(&label) {
                        let canvas = canvas_win.canvas.lock().unwrap();
                        canvas_win.renderer.update(&*canvas);

                        canvas_win.renderer.render();

                        let win_id = get_id_from_tao_id(window_id, &context);

                        if let Some(id) = win_id {
                            proxy
                                .send_event(Message::Window(
                                    id,
                                    tauri_runtime_wry::WindowMessage::RequestRedraw,
                                ))
                                .ok();
                        }
                    }
                }
            }

            Event::MainEventsCleared => {}
            &_ => {}
        }

        false
    }
}

struct CanvasWindow {
    canvas: Arc<Mutex<Canvas>>,
    renderer: RenderState,
}

pub trait AppHandleExt {
    fn start_renderer_for_window(&self, label: &str, canvas: Arc<Mutex<Canvas>>) -> Result<()>;
}
impl AppHandleExt for AppHandle {
    fn start_renderer_for_window(&self, label: &str, canvas: Arc<Mutex<Canvas>>) -> Result<()> {
        let canvas_windows = self
            .try_state::<CanvasWindowMap>()
            .ok_or(Error::msg("TauriPluginCanvasRenderer is not initialized"))?;

        let window = self
            .get_window(label)
            .ok_or(Error::msg("No window found with the provided label"))?;

        let renderer = RenderState::with_window(window)?;
        let ccanvas = canvas.lock().unwrap();
        renderer.attach_canvas(&ccanvas);
        drop(ccanvas);

        let mut managed_windows = canvas_windows.lock().unwrap();
        managed_windows.insert(label.to_string(), CanvasWindow { canvas, renderer });

        Ok(())
    }
}
