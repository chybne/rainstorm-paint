use crate::InputInitError;
use objc2::{
    self, ClassType, MainThreadMarker, MainThreadOnly, define_class, msg_send, rc::Retained,
};
use objc2_app_kit::{self, NSEvent, NSEventType, NSView};
use objc2_foundation::{NSPoint, NSRect};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

use block2::RcBlock;
use std::ptr::NonNull;

use crate::{InputEventSink, TabletEvent};

use super::InputEvent;

#[derive(Debug)]
pub struct Ivars {
    _keep_this_alive: RcBlock<dyn Fn(NonNull<NSEvent>) -> *mut NSEvent>,
}

define_class!(
    #[unsafe(super(NSView))]

    #[ivars = Ivars]
    pub struct InputView;

    impl InputView {
        #[unsafe(method_id(hitTest:))]
        fn hit_test(&self, _point: NSPoint) -> Option<Retained<NSView>> {
            None
        }
    }


);

impl InputView {
    pub fn init_with_frame(
        frame: NSRect,
        mtm: MainThreadMarker,
        event_sink: Box<dyn InputEventSink>,
    ) -> Retained<Self> {
        let handler = unsafe {
            use objc2_app_kit::{NSEvent, NSEventMask};

            let mask = NSEventMask::LeftMouseDown | NSEventMask::LeftMouseDragged;

            let mtm2 = mtm.clone();
            let handler = RcBlock::new(move |event: NonNull<NSEvent>| -> *mut NSEvent {
                let e = &*event.as_ptr();

                let window = e.window(mtm2).unwrap();

                let event_type = e.r#type();

                let frame = window.frame();
                let height = frame.size.height;
                let location = e.locationInWindow();
                let pos_x = location.x;
                let pos_y = height - location.y;

                let pressure = e.pressure();

                match event_type {
                    NSEventType::LeftMouseDown => {
                        event_sink
                            .try_send(InputEvent::Tablet(TabletEvent::TabletDown {
                                x: pos_x,
                                y: pos_y,
                                pressure,
                            }))
                            .ok();
                    }
                    NSEventType::LeftMouseDragged => {
                        event_sink
                            .try_send(InputEvent::Tablet(TabletEvent::TabletMoved {
                                x: pos_x,
                                y: pos_y,
                                pressure,
                            }))
                            .ok();
                    }
                    _ => {}
                }

                event.as_ptr()
            });

            NSEvent::addLocalMonitorForEventsMatchingMask_handler(mask, &handler);
            handler
        };

        unsafe {
            // Call super's initWithFrame
            let this = Self::alloc(mtm).set_ivars(Ivars {
                _keep_this_alive: handler,
            });
            msg_send![super(this), initWithFrame: frame]
        }
    }
}

pub fn start_tracking_gestures_macos(
    handle: impl HasWindowHandle,
    event_sink: Box<dyn InputEventSink>,
) -> Result<(), InputInitError> {
    let handle = handle.window_handle()?;
    let raw_handle = handle.as_raw();

    let RawWindowHandle::AppKit(appkit_handle) = raw_handle else {
        return Err(InputInitError::HandleWrongPlatform);
    };

    let ns_view = appkit_handle.ns_view.as_ptr();

    /*
     * probably should let user provide an mtm instead of assuming
     * that we are on the main thread
     */
    let mtm = MainThreadMarker::new().ok_or(InputInitError::NotOnMainThread)?;

    let ns_view: Retained<NSView> =
        unsafe { Retained::retain(ns_view.cast()) }.ok_or(InputInitError::NullWindowPointer)?;

    println!("{ns_view:?}");

    let window = ns_view.window().ok_or(InputInitError::InvalidNSView);

    println!("{window:?}");

    let frame = ns_view.frame();

    println!("{frame:?}");

    let view = InputView::init_with_frame(frame, mtm, event_sink);

    ns_view.addSubview(view.as_super());

    Ok(())
}
