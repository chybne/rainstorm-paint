use iced::widget::{button, text, column};
use iced::Element;
use crate::widgets::Canvas;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
}

#[derive(Debug, Default)]
pub struct CanvasScreen {
    counter: u32,
}


impl CanvasScreen {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.counter += 1,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let canvas = Canvas::new();

        let button = button(text(self.counter)).on_press(Message::Increment);

        let content = column![]
            .push(button)
            .push(canvas);

        content.into()

    }


}