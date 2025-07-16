use iced::futures::{Stream, sink::SinkExt};
use iced::wgpu::rwh::RawWindowHandle;
use iced::{self, Element, Subscription, Task, stream};
use input::{self, InputEvent};
use screens::canvas_screen::{self, CanvasScreen};
use screens::home_screen::{self, HomeScreen};

use tokio::sync::mpsc::{self, Sender};

mod screens;
mod widgets;

pub type Result = iced::Result;

#[derive(Debug)]
pub enum Message {
    CanvasScreen(canvas_screen::ScreenMessage),
    HomeScreen(home_screen::ScreenMessage),
    InitializeInput(iced::window::Id),
    InputSend(Sender<InputEvent>),
    HandleLoaded,
}

// Rainstorm struct keeps track with the state of the program
pub struct Rainstorm {
    screen: Box<dyn Screen>,
    id: Option<iced::window::Id>,
}

impl Default for Rainstorm {
    fn default() -> Self {
        Self {
            screen: Box::new(CanvasScreen::default()),
            id: None,
        }
    }
}

pub trait Screen {
    fn update(&mut self, message: Message) -> Action;
    fn view(&self) -> iced::Element<Message>;
}

pub enum Action {
    None,
    Task(Task<Message>),
    ChangeScreen(Box<dyn Screen>),
}

impl Rainstorm {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InitializeInput(id) => {
                self.id = Some(id);
                Task::none()
            }

            Message::InputSend(send) => {
                let id = self.id.expect("should be initialized");
                let window = iced::window::run_with_handle(id, move |handle| {
                    match handle.as_raw() {
                        RawWindowHandle::AppKit(handle) => {
                            let ns_view = handle.ns_view.as_ptr();
                            input::start_tracking_gestures(ns_view, send);
                        }
                        _ => unreachable!("unknown handle {handle:?} for platform"),
                    }
                    Message::HandleLoaded
                });

                window
            }

            Message::HandleLoaded => Task::none(),

            _ => match self.screen.update(message) {
                Action::ChangeScreen(screen) => {
                    self.screen = screen;
                    Task::none()
                }
                Action::Task(task) => task,
                Action::None => Task::none(),
            },
        }
    }

    fn view(&self) -> Element<Message> {
        self.screen.view()
    }

    fn input_stream() -> impl Stream<Item = Message> {
        stream::channel(100, async move |mut output| {
            let (send, mut recv) = mpsc::channel::<InputEvent>(10);

            output
                .send(Message::InputSend(send))
                .await
                .expect("should work XD");

            loop {
                let input = recv.recv().await;

                let Some(input) = input else {
                    break;
                };

                println!("{input:?}");
            }
        })
    }

    fn subscription(&self) -> Subscription<Message> {
        if let Some(_) = &self.id {
            let input_sub = Subscription::run(Rainstorm::input_stream);
            Subscription::batch(vec![input_sub])
        } else {
            Subscription::none()
        }
    }

    pub fn run() -> Result {
        #[cfg(not(target_os = "macos"))]
        {
            iced::application(Rainstorm::default, Rainstorm::update, Rainstorm::view)
        }

        #[cfg(target_os = "macos")]
        iced::application(
            || {
                (
                    Rainstorm::default(),
                    iced::window::get_latest().map(|id| Message::InitializeInput(id.unwrap())),
                )
            },
            Rainstorm::update,
            Rainstorm::view,
        )
        .title("Rainstorm Paint")
        .subscription(Rainstorm::subscription)
        .run()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
