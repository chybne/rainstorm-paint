use iced::widget::button;
use iced::{Background, Border, Color, Element};

#[derive(Clone, Debug, Copy)]
pub enum Message {
    ChangeColor,
}

#[derive(Debug)]
pub struct HomeScreen {
    color: Color,
}

impl Default for HomeScreen {
    fn default() -> Self {
        Self {
            color: Color::from_rgba(0f32, 0f32, 0f32, 1f32),
        }
    }
}

impl HomeScreen {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeColor => {
                self.color.r = (self.color.r + 0.1) % 1.0;
                self.color.g = (self.color.g + 0.05) % 1.0;
                self.color.b = (self.color.b + 0.02) % 1.0;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let button = button("Hi, Im a button")
            .style(|_theme, _status| button::Style {
                background: Some(Background::Color(self.color)),
                border: Border {
                    color: Color::WHITE,
                    width: 1.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .on_press(Message::ChangeColor);

        button.into()
    }
}
