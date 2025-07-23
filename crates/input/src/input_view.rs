use objc2::{
    self, DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, rc::Retained, sel,
};

use objc2_app_kit::{NSEvent, NSMagnificationGestureRecognizer, NSView};
use objc2_foundation::{NSPoint, NSRect};
use tokio::sync::mpsc::Sender;

use crate::TabletEvent;

use super::{GestureEvent, InputEvent};

// enum TabletState {
//     NotAvailable,
//     Hover,
//     Drawing,
// }

#[derive(Debug)]
pub struct Ivars {
    send: Sender<super::InputEvent>,
}

define_class!(
    #[unsafe(super(NSView))]

    #[ivars = Ivars]
    pub struct InputView;

    impl InputView {

        #[unsafe(method(acceptsFirstResponder))]
        fn accepts_first_responder(&self) -> bool {
            true
        }

        #[unsafe(method(isFlipped))]
        fn is_flipped(&self) -> bool {
            true
        }

        #[unsafe(method(scrollWheel:))]
        fn scroll_wheel(&self, event: &NSEvent) {
            let scroll = unsafe {(event.scrollingDeltaX(), event.scrollingDeltaY())};

            let _ = self.ivars().send.try_send(InputEvent::Gesture(GestureEvent::PanGesture { dx: scroll.0, dy: scroll.1 }));
            unsafe {
                if let Some(responder) = self.nextResponder() {
                    responder.scrollWheel(event);
                }
            }
        }

        #[unsafe(method(tabletPoint:))]
        fn tablet_point(&self, event: &NSEvent) {
            let point = unsafe {(event.absoluteX(), event.absoluteY(), event.absoluteZ())};
            println!("caught point: {point:?}");
            unsafe {
                if let Some(responder) = self.nextResponder() {
                    responder.tabletPoint(event);
                }
            }
        }

        #[unsafe(method(mouseDragged:))]
        fn mouse_dragged(&self, event: &NSEvent) {

            let point: NSPoint = self.convertPoint_fromView(unsafe {event.locationInWindow()}, None);
            let pressure = unsafe { event.pressure() };

            unsafe {
                let _tilt_x = event.tilt().x;
                let _tilt_y = event.tilt().y;
            }

            let _ = self.ivars().send.try_send(InputEvent::Tablet(TabletEvent::TabletPoint { x: point.x, y: point.y, pressure }));

            unsafe {
                if let Some(responder) = self.nextResponder() {
                    responder.mouseDragged(event);
                }
            }
        }

        #[unsafe(method(mouseMoved:))]
        fn mouse_moved(&self, event: &NSEvent) {
            let point: NSPoint = self.convertPoint_fromView(unsafe {event.locationInWindow()}, None);

            let _ = self.ivars().send.try_send(InputEvent::Tablet(TabletEvent::TabletMoved { x: point.x, y: point.y}));

            unsafe {
                if let Some(responder) = self.nextResponder() {
                    responder.mouseMoved(event);
                }
            }
        }


        #[unsafe(method(tabletProximity:))]
        fn tablet_proximity(&self, event: &NSEvent) {
            unsafe {
                if let Some(responder) = self.nextResponder() {
                    responder.tabletProximity(event);
                }
            }

        }

        #[unsafe(method(mouseUp:))]
        fn mouse_released(&self, event: &NSEvent) {

            unsafe {
                if let Some(responder) = self.nextResponder() {
                    responder.mouseUp(event);
                }
            }
        }

        #[unsafe(method(handleMagnify:))]
        fn handle_magnify(&self, recognizer: &NSMagnificationGestureRecognizer) {
            let mag = unsafe {recognizer.magnification()};

            let _ = self.ivars().send.try_send(InputEvent::Gesture(GestureEvent::MagnifyGesture { scale: mag }) );
        }
    }


);

impl InputView {
    pub fn init_with_frame(
        frame: NSRect,
        mtm: MainThreadMarker,
        send: Sender<super::InputEvent>,
    ) -> Retained<Self> {
        unsafe {
            // Call super's initWithFrame
            let this = Self::alloc(mtm).set_ivars(Ivars { send });
            msg_send![super(this), initWithFrame: frame]
        }
    }

    pub fn install_recognizers(&self, mtm: MainThreadMarker) {
        let mag_recognizer = NSMagnificationGestureRecognizer::alloc(mtm);

        let mag_recognizer = unsafe {
            NSMagnificationGestureRecognizer::initWithTarget_action(
                mag_recognizer,
                Some(self),
                Some(sel!(handleMagnify:)),
            )
        };

        unsafe {
            self.addGestureRecognizer(&mag_recognizer);
        }
    }
}
