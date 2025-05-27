use iced::widget::{button, text, column};
use iced::Element;
use crate::widgets::Canvas;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    ChangeScreen
}

pub enum Action {
    ChangeScreen,
    Nothing,
}

#[derive(Debug, Default)]
pub struct CanvasScreen {
    counter: u32,
}


impl CanvasScreen {
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Increment => {self.counter += 1; Action::Nothing},
            Message::ChangeScreen => Action::ChangeScreen
        }
    }

    pub fn view(&self) -> Element<Message> {
        let canvas = Canvas::new();

        let button_1 = button(text(self.counter)).on_press(Message::Increment);
        let button_2 =  button("Click me to change screen").on_press(Message::ChangeScreen);
    
        let content = column![]
            .push(button_1)
            .push(button_2)
            .push(canvas);

        content.into()

    }


}