use iced::widget::{container, text, button, column};
use iced::Element;


#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
}

#[derive(Default)]
pub struct Rainstorm {
    counter: u32,
}

impl Rainstorm {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.counter += 1,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let content  = container("hellow world")
            .padding(10)
            .center(400)
            .style(container::rounded_box);


        let result = column![]
            .push(content)
            .push(button("+").on_press(Message::Increment))
            .push(text(self.counter));

        result.into()
    }
}   

#[cfg(test)]
mod tests {
    // use super::*;
}
