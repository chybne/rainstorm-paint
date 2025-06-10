#[allow(unused_imports)]
use crate::widgets::Canvas;
use crate::widgets::canvas_widget;
use canvas;
use iced::widget::{Shader, button, column, row, text};
use iced::{Element, Task};

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
    canvases: Vec<canvas::Canvas>,
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
                self.canvases.push(canvas);
                Action::Nothing
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let button_1 = button(text(self.counter)).on_press(UiMessage::Increment);
        let button_2 = button("Click me to change screen").on_press(UiMessage::ChangeScreen);
        let button_3 = button("Click me to Load a canvas").on_press(UiMessage::LoadCanvas);

        let shader: Shader<UiMessage, canvas_widget::Canvas> =
            iced::widget::shader::Shader::new(Canvas::new());

        let buttons = row![].push(button_1).push(button_2).push(button_3);

        #[allow(unused_mut)]
        let mut content = column![].push(buttons).push(shader);

        for _canvas in &self.canvases {

            // content = content.push(Canvas::new());
        }

        let content: Element<'_, UiMessage> = content.into();

        content.map(Message::Ui)
    }
}
