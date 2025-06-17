#[allow(unused_imports)]
use crate::widgets::CanvasWidget;
use canvas;
use iced::widget::{Shader, button, column, row, text};
use iced::{Element, Task};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub enum Message {
    Ui(UiMessage),
    CanvasLoaded(canvas::Canvas),
}

#[derive(Debug, Clone)]
pub(crate) enum UiMessage {
    Increment,
    ChangeScreen,
    LoadCanvas,
}

pub enum Action {
    ChangeScreen,
    Nothing,
    Task(Task<Message>),
}

#[derive(Debug, Default)]
pub struct CanvasScreen {
    counter: u32,
    canvases: Vec<Arc<RwLock<canvas::Canvas>>>,
}

impl CanvasScreen {
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Ui(ui_message) => match ui_message {
                UiMessage::Increment => {
                    self.counter += 1;
                    Action::Nothing
                }
                UiMessage::ChangeScreen => Action::ChangeScreen,
                UiMessage::LoadCanvas => {
                    Action::Task(Task::perform(canvas::Canvas::new(), Message::CanvasLoaded))
                }
            },

            Message::CanvasLoaded(canvas) => {
                self.canvases.push(Arc::new(RwLock::new(canvas)));
                Action::Nothing
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let button_1 = button(text(self.counter)).on_press(UiMessage::Increment);
        let button_2 = button("Click me to change screen").on_press(UiMessage::ChangeScreen);
        let button_3 = button("Click me to Load a canvas").on_press(UiMessage::LoadCanvas);

        let buttons = row![].push(button_1).push(button_2).push(button_3);

        #[allow(unused_mut)]
        let mut content = column![].push(buttons);

        for canvas in &self.canvases {
            let shader = Shader::new(CanvasWidget::new(canvas.clone()))
                .width(500)
                .height(500);
            content = content.push(shader);
        }

        let content: Element<'_, UiMessage> = content.into();

        content.map(Message::Ui)
    }
}
