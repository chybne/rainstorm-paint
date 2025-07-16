use crate::{Action, Message, Screen};
use iced::widget::{button, row};
use iced::{Background, Border, Color, Element};

#[derive(Clone, Debug, Copy)]
pub enum ScreenMessage {
    ChangeColor,
    ChangeScreen,
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

impl Screen for HomeScreen {
    fn update(&mut self, message: Message) -> Action {
        if let Message::HomeScreen(message) = message {
            match message {
                ScreenMessage::ChangeColor => {
                    self.color.r = (self.color.r + 0.1) % 1.0;
                    self.color.g = (self.color.g + 0.05) % 1.0;
                    self.color.b = (self.color.b + 0.02) % 1.0;
                    Action::None
                }
                ScreenMessage::ChangeScreen => {
                    Action::ChangeScreen(Box::new(crate::CanvasScreen::default()))
                }
            }
        } else {
            Action::None
        }
    }

    fn view(&self) -> Element<Message> {
        let button1 = button("Hi, Im a button")
            .style(|_theme, _status| button::Style {
                background: Some(Background::Color(self.color)),
                border: Border {
                    color: Color::WHITE,
                    width: 1.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .on_press(ScreenMessage::ChangeColor);

        let button2 = button("CLick me to go to canvas").on_press(ScreenMessage::ChangeScreen);

        let content: Element<ScreenMessage> = row![button1, button2].into();

        content.map(Message::HomeScreen)
    }
}
