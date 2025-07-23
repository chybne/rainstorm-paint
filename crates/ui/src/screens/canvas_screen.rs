use crate::widgets::CanvasWidget;
use canvas::{self, Canvas};
use iced::Element;
use iced::widget::{Shader, button, column, mouse_area, row, slider};
use input::{GestureEvent, InputEvent, TabletEvent};
use std::sync::{Arc, RwLock};

use crate::{Action, Message, Screen};

#[derive(Debug, Clone)]
pub enum ScreenMessage {
    ChangedZoom(f32),
    CanvasFocused(bool),
    ChangeScreen,
    LoadCanvas,
}

#[derive(Debug)]
pub struct CanvasScreen {
    zoom: f32,
    mouse_pos: canvas::Point,
    is_canvas_focused: bool,
    canvases: Vec<Arc<RwLock<canvas::Canvas>>>,
}

impl Default for CanvasScreen {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            mouse_pos: canvas::Point::default(),
            is_canvas_focused: false,
            canvases: vec![],
        }
    }
}

impl Screen for CanvasScreen {
    fn update(&mut self, message: Message) -> Action {
        if let Message::CanvasScreen(message) = message {
            match message {
                ScreenMessage::CanvasFocused(focus) => {
                    self.is_canvas_focused = focus;
                    println!("is canvas focused? {focus:?}");
                    return Action::None;
                }

                ScreenMessage::ChangedZoom(zoom) => {
                    self.zoom = zoom;
                    for canvas in self.canvases.iter() {
                        canvas.write().unwrap().set_zoom_temp(zoom);
                    }
                    return Action::None;
                }

                ScreenMessage::ChangeScreen => {
                    return Action::ChangeScreen(Box::new(crate::HomeScreen::default()));
                }

                ScreenMessage::LoadCanvas => {
                    self.canvases.push(Arc::new(RwLock::new(Canvas::new())));
                    return Action::None;
                }
            }
        }

        if let Message::Input(InputEvent::Gesture(input)) = message {
            return self.handle_gesture_input(input);
        }

        if let Message::Input(InputEvent::Tablet(input)) = message {
            return self.handle_tablet_input(input);
        }

        Action::None
    }

    fn view(&self) -> Element<Message> {
        let slider = slider(0.3..=5.0, self.zoom, ScreenMessage::ChangedZoom)
            .step(0.05)
            .width(100);
        let button_2 = button("Click me to change screen").on_press(ScreenMessage::ChangeScreen);
        let button_3 = button("Click me to Load a canvas").on_press(ScreenMessage::LoadCanvas);

        let buttons = row![].push(slider).push(button_2).push(button_3);

        let mut content = column![].push(buttons);

        for (i, canvas) in self.canvases.iter().enumerate() {
            let shader = Shader::new(CanvasWidget::new(canvas.clone(), i as u32))
                .width(800)
                .height(500);

            let shader = mouse_area(shader)
                .on_enter(ScreenMessage::CanvasFocused(true))
                .on_exit(ScreenMessage::CanvasFocused(false));
            content = content.push(shader);
        }

        let content: Element<'_, ScreenMessage> = content.into();

        content.map(Message::CanvasScreen)
    }
}

impl CanvasScreen {
    fn handle_tablet_input(&mut self, input: TabletEvent) -> Action {
        match input {
            TabletEvent::TabletPoint { x, y, pressure } => {
                let point = canvas::Point {
                    x: x as f32,
                    y: y as f32,
                };

                self.mouse_pos = point;

                for canvas in self.canvases.iter() {
                    if let Ok(canvas) = canvas.read() {
                        println!(
                            "is {:?} in {:?} and pressure {:?}",
                            point,
                            canvas.is_within_bounds(point),
                            pressure
                        );

                        if let Some(pos) = canvas.convert_to_canvas_coords(point) {
                            println!("canvas coords {pos:?}");
                        }
                    }
                }
                Action::None
            }
            TabletEvent::TabletMoved { x, y } => {
                let point = canvas::Point {
                    x: x as f32,
                    y: y as f32,
                };
                self.mouse_pos = point;

                Action::None
            }
            _ => Action::None,
        }
    }

    fn handle_gesture_input(&mut self, input: GestureEvent) -> Action {
        match input {
            GestureEvent::MagnifyGesture { scale } => {
                if !self.is_canvas_focused {
                    return Action::None;
                }
                // i know these are some magic number
                // will change later
                let delta = 1.0 + scale as f32 * 0.1;

                for canvas in self.canvases.iter() {
                    let mut canvas = canvas.write().unwrap();
                    canvas.zoom_relative_to_point(delta, self.mouse_pos);
                    self.zoom = canvas.zoom();
                }

                Action::None
            }
            GestureEvent::PanGesture { dx, dy } => {
                self.canvases
                    .iter()
                    .filter_map(|lock| lock.write().ok())
                    .for_each(|mut canvas| {
                        canvas.apply_offset(dx as f32, dy as f32);
                    });
                Action::None
            }
        }
    }
}
