use iced::widget::{Container, container};

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Default)]
pub struct Rainstorm {}

impl Rainstorm {
    pub fn update(&mut self, _message: Message) {}

    pub fn view(&self) -> Container<Message> {
        container("hellow world")
            .padding(10)
            .center(800)
            .style(container::rounded_box)
            .into()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
