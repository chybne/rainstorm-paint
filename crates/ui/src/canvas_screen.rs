use iced::widget::{button, text};
use iced::Element;

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
        button(text(self.counter)).on_press(Message::Increment).into()
    }


}