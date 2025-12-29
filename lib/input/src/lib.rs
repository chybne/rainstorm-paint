#[cfg(target_os = "macos")]
mod macos;

use raw_window_handle::HandleError;
use raw_window_handle::HasWindowHandle;
use std::fmt::{Debug, Display};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputInitError {
    #[error("handle error")]
    HandleError(#[from] HandleError),
    #[error("handle from wrong platform")]
    HandleWrongPlatform,
    #[error("Not on Main thread")]
    NotOnMainThread,
    #[error("window pointer was Null")]
    NullWindowPointer,
    #[error("invalid window pointer")]
    InvalidNSView,
}

#[derive(Error, Debug)]
pub enum InputSendError {
    #[error("Failed to send event")]
    FailedSend,
}

pub fn start_tracking_gestures(
    handle: impl HasWindowHandle,
    event_sink: Box<dyn InputEventSink>,
) -> Result<(), InputInitError> {
    #[cfg(target_os = "macos")]
    return macos::start_tracking_gestures_macos(handle, event_sink);
}

pub trait InputEventSink: Send + Debug + 'static {
    fn try_send(&self, event: InputEvent) -> Result<(), InputSendError>;
}

#[derive(Debug)]
pub enum InputEvent {
    Gesture(GestureEvent),
    Tablet(TabletEvent),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum TabletEvent {
    TabletDown { x: f64, y: f64, pressure: f32 },
    TabletMoved { x: f64, y: f64, pressure: f32 },
    TabletReleased,
}

impl Display for TabletEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TabletEvent::TabletDown { x, y, pressure } => {
                write!(f, "TabletDown(x: {x}, y: {y}, pressure: {pressure})")
            }
            TabletEvent::TabletMoved { x, y, pressure } => {
                write!(f, "TabletMoved(x: {x}, y: {y}, pressure: {pressure})")
            }
            TabletEvent::TabletReleased => {
                write!(f, "TabletReleased!")
            }
        }
    }
}

#[derive(Debug)]
pub enum GestureEvent {
    MagnifyGesture { scale: f64 },
    PanGesture { dx: f64, dy: f64 },
    RotationGesture { rotation: f64 },
}
