use screens::canvas_screen::{self, CanvasScreen};
use screens::home_screen::{self, HomeScreen};
use iced::{self, Element};

mod screens;
mod widgets;

pub type Result = iced::Result;

#[derive(Debug, Clone, Copy)]
enum Message {
    CanvasScreen(canvas_screen::Message),
    HomeScreen(home_screen::Message)
}

#[derive(Debug)]
enum Screen {
    CanvasScreen(CanvasScreen),
    HomeScreen(HomeScreen),
}
impl Default for Screen {
    fn default() -> Self {
        Screen::CanvasScreen(CanvasScreen::default())
    }
}

// Rainstorm struct keeps track with the state of the program
#[derive(Default)]
pub struct Rainstorm {
    screen: Screen,
}

impl Rainstorm {
    fn update(&mut self, message: Message) {
        match message {
            Message::CanvasScreen(message) => {
                if let Screen::CanvasScreen(canvas_screen) = &mut self.screen {
                    let action = canvas_screen.update(message);

                    match action {
                        canvas_screen::Action::Nothing => (),
                        canvas_screen::Action::ChangeScreen => {self.screen = Screen::HomeScreen(HomeScreen::default());}
                    }
                } else {
                    ()
                }
            },
            Message::HomeScreen(message) => {
                if let Screen::HomeScreen(screen) = &mut self.screen {
                    screen.update(message);
                } else {
                    ()
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match &self.screen {
            Screen::CanvasScreen(screen) => screen.view().map(Message::CanvasScreen),
            Screen::HomeScreen(screen) => screen.view().map(Message::HomeScreen),
        }
    }

    pub fn run() -> Result {
        iced::application("my first app", Rainstorm::update, Rainstorm::view).run()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
