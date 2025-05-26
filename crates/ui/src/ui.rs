use canvas_screen::CanvasScreen;
use iced::{self, Element};

mod canvas_screen;

pub type Result = iced::Result;

#[derive(Debug, Clone, Copy)]
enum Message {
    CanvasScreen(canvas_screen::Message),
}

#[derive(Debug)]
enum Screen {
    CanvasScreen(CanvasScreen),
    NOtcanva(CanvasScreen),
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
                    canvas_screen.update(message)
                } else {
                    ()
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match &self.screen {
            Screen::CanvasScreen(canvas_screen) => canvas_screen.view().map(Message::CanvasScreen),
            Screen::NOtcanva(canvas_screen) => canvas_screen.view().map(Message::CanvasScreen),
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
