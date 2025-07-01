use crate::widgets::CanvasWidget;
use canvas::{self, Canvas};
use iced::widget::{Shader, button, column, row, slider};
use iced::{Element, Task};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub enum Message {
    Ui(UiMessage),
    CanvasLoaded(canvas::Canvas),
}

#[derive(Debug, Clone)]
pub(crate) enum UiMessage {
    ChangedZoom(f32),
    ChangeScreen,
    LoadCanvas,
}

pub enum Action {
    ChangeScreen,
    Nothing,
    Task(Task<Message>),
}

#[derive(Debug)]
pub struct CanvasScreen {
    zoom: f32,
    canvases: Vec<Arc<RwLock<canvas::Canvas>>>,
}

impl Default for CanvasScreen {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            canvases: vec![],
        }
    }
}

impl CanvasScreen {
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Ui(ui_message) => match ui_message {
                UiMessage::ChangedZoom(zoom) => {
                    self.zoom = zoom;
                    for canvas in self.canvases.iter() {
                        canvas.write().unwrap().scale_matrix(self.zoom);
                    }
                    Action::Nothing
                }
                UiMessage::ChangeScreen => Action::ChangeScreen,
                UiMessage::LoadCanvas => {
                    Action::Task(Task::perform(Canvas::new(), Message::CanvasLoaded))
                }
            },

            Message::CanvasLoaded(canvas) => {
                self.canvases.push(Arc::new(RwLock::new(canvas)));
                Action::Nothing
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let slider = slider(0.3..=5.0, self.zoom, UiMessage::ChangedZoom)
            .step(0.05)
            .width(100);
        let button_2 = button("Click me to change screen").on_press(UiMessage::ChangeScreen);
        let button_3 = button("Click me to Load a canvas").on_press(UiMessage::LoadCanvas);

        let buttons = row![].push(slider).push(button_2).push(button_3);

        let mut content = column![].push(buttons);

        for (i, canvas) in self.canvases.iter().enumerate() {
            let shader = Shader::new(CanvasWidget::new(canvas.clone(), i as u32))
                .width(800)
                .height(500);
            content = content.push(shader);
        }

        let content: Element<'_, UiMessage> = content.into();

        content.map(Message::Ui)
    }
}
