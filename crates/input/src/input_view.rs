use objc2::{
    self, DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, rc::Retained, sel,
};

use objc2_app_kit::{NSEvent, NSMagnificationGestureRecognizer, NSView};
use objc2_foundation::NSRect;
use std::cell::RefCell;
use tokio::sync::mpsc::Sender;

use super::{GestureEvent, InputEvent};

// enum TabletState {
//     NotAvailable,
//     Hover,
//     Drawing,
// }

#[derive(Debug)]
pub struct Ivars {
    send: Sender<super::InputEvent>,
    is_drawing: RefCell<bool>,
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
        #[cfg(feature = "gesture-pan")]
        fn scroll_wheel(&self, event: &NSEvent) {
            let scroll = unsafe {(event.deltaX(), event.deltaY())};
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

            self.handle_drawing(event);

            unsafe {
                if let Some(responder) = self.nextResponder() {
                    responder.mouseDragged(event);
                }
            }
        }

        #[unsafe(method(mouseMoved:))]
        fn mouse_moved(&self, event: &NSEvent) {
            self.handle_drawing(event);

            unsafe {
                if let Some(responder) = self.nextResponder() {
                    responder.mouseMoved(event);
                }
            }
        }


        #[unsafe(method(tabletProximity:))]
        fn tablet_proximity(&self, event: &NSEvent) {
            let s = unsafe { event.isEnteringProximity() };
            println!("is entering proximity: {s:?}");
            *self.ivars().is_drawing.borrow_mut() = s;

            unsafe {
                if let Some(responder) = self.nextResponder() {
                    responder.tabletProximity(event);
                }
            }

        }

        #[unsafe(method(mouseUp:))]
        fn mouse_released(&self, event: &NSEvent) {
            println!("released");
            *self.ivars().is_drawing.borrow_mut() = true;

            unsafe {
                if let Some(responder) = self.nextResponder() {
                    responder.mouseUp(event);
                }
            }
        }

        #[cfg(feature = "gesture-magnify")]
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
            let this = Self::alloc(mtm).set_ivars(Ivars {
                send,
                is_drawing: RefCell::new(false),
            });
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

    fn handle_drawing(&self, event: &NSEvent) {
        if *self.ivars().is_drawing.borrow() {
            unsafe {
                let position = event.locationInWindow(); // NSPoint
                let position = self.convertPoint_fromView(position, None);
                let pressure = event.pressure(); // CGFloat (0.0 - 1.0)

                let tilt_x = event.tilt().x;
                let tilt_y = event.tilt().y;

                println!(
                    "position {position:?} pressure {pressure:?} tiltx {tilt_x:?} tilty {tilt_y:?}"
                );
            }
        }
    }
}
