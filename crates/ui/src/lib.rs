use iced::widget::{container, text, Column, column, Container};
use iced::overlay::Element;


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

    pub fn view<'a>(&self) -> Column<Message> {
        let content  = container("hellow world")
            .padding(10)
            .center(800)
            .style(container::rounded_box);


        let result = column![].push(content);

        result
    }
}   

#[cfg(test)]
mod tests {
    // use super::*;
}
