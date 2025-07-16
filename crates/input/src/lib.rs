mod input_view;

use input_view::InputView;

use std::os::raw::c_void;
use tokio::sync::mpsc::Sender;

use objc2::{ClassType, MainThreadMarker, rc::Retained};
use objc2_app_kit::{self, NSView};

pub fn start_tracking_gestures(ns_view: *mut c_void, send: Sender<InputEvent>) {
    /*
     * probably should let user provide an mtm instead of assuming
     * that we are on the main thread
     */
    let mtm = MainThreadMarker::new().unwrap();

    let ns_view: Retained<NSView> = unsafe { Retained::retain(ns_view.cast()) }.unwrap();

    println!("{ns_view:?}");

    let window = ns_view.window().unwrap();

    println!("{window:?}");

    let frame = ns_view.frame();

    println!("{frame:?}");

    let view = InputView::init_with_frame(frame, mtm, send);
    view.install_recognizers(mtm);

    unsafe {
        let d = view.isFlipped();
        println!("frssfd {d:?}");

        ns_view.addSubview(view.as_super());
    }
}

#[derive(Debug)]
pub enum InputEvent {
    Gesture(GestureEvent),
    Tablet(TabletEvent),
}

#[derive(Debug)]
pub enum TabletEvent {
    Point { x: f64, y: f64, z: f64 },
}

#[derive(Debug)]
pub enum GestureEvent {
    #[cfg(feature = "gesture-magnify")]
    MagnifyGesture { scale: f64 },

    #[cfg(feature = "gesture-pan")]
    PanGesture { dx: f64, dy: f64 },
}
