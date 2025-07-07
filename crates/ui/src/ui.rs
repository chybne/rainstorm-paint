use iced::futures::{Stream, sink::SinkExt};
use iced::wgpu::rwh::RawWindowHandle;
use iced::{self, Element, Subscription, Task, stream};
use input::gestures::{self, GestureEvent};
use input::tablet::{self, TabletEvent};
use screens::canvas_screen::{self, CanvasScreen};
use screens::home_screen::{self, HomeScreen};

use tokio::sync::mpsc::{self, Sender};

mod screens;
mod widgets;

pub type Result = iced::Result;

#[derive(Debug)]
enum Message {
    CanvasScreen(canvas_screen::Message),
    HomeScreen(home_screen::Message),
    InitializeInput(iced::window::Id),
    GestureSend(Sender<GestureEvent>),
    HandleLoaded,
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
    id: Option<iced::window::Id>,
}

impl Rainstorm {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CanvasScreen(message) => {
                let Screen::CanvasScreen(canvas_screen) = &mut self.screen else {
                    return Task::none();
                };
                let action = canvas_screen.update(message);

                match action {
                    canvas_screen::Action::Nothing => Task::none(),
                    canvas_screen::Action::ChangeScreen => {
                        self.screen = Screen::HomeScreen(HomeScreen::default());
                        Task::none()
                    }
                    canvas_screen::Action::Task(task) => task.map(Message::CanvasScreen),
                }
            }
            Message::HomeScreen(message) => {
                if let Screen::HomeScreen(screen) = &mut self.screen {
                    screen.update(message);
                    Task::none()
                } else {
                    Task::none()
                }
            }

            Message::InitializeInput(id) => {
                self.id = Some(id);
                Task::none()
            }

            Message::GestureSend(send) => {
                let id = self.id.expect("should be initialized");
                let window = iced::window::run_with_handle(id, move |handle| {
                    match handle.as_raw() {
                        RawWindowHandle::AppKit(handle) => {
                            let ns_view = handle.ns_view.as_ptr();
                            gestures::start_tracking_gestures(ns_view, send);
                        }
                        _ => unreachable!("unknown handle {handle:?} for platform"),
                    }
                    Message::HandleLoaded
                });

                window
            }

            Message::HandleLoaded => Task::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        match &self.screen {
            Screen::CanvasScreen(screen) => screen.view().map(Message::CanvasScreen),
            Screen::HomeScreen(screen) => screen.view().map(Message::HomeScreen),
        }
    }

    fn input_stream() -> impl Stream<Item = Message> {
        stream::channel(100, async move |mut output| {
            let (send, mut recv) = mpsc::channel::<GestureEvent>(10);

            output
                .send(Message::GestureSend(send))
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

    fn tablet_stream() -> impl Stream<Item = Message> {
        stream::channel::<Message>(100, async move |mut _output| {
            let (send, mut recv) = mpsc::channel::<TabletEvent>(10);

            tablet::start_tracking_tablet(send);

            loop {
                let Some(input) = recv.recv().await else {
                    println!("channel broken");
                    break;
                };

                println!("{input:?}");
            }
        })
    }

    fn subscription(&self) -> Subscription<Message> {
        if let Some(_) = &self.id {
            let input_sub = Subscription::run(Rainstorm::input_stream);
            let tablet_sub = Subscription::run(Rainstorm::tablet_stream);
            Subscription::batch(vec![input_sub, tablet_sub])
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
