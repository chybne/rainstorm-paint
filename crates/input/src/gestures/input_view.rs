use objc2::{
    self, DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, rc::Retained, sel,
};

use objc2_app_kit::{NSEvent, NSMagnificationGestureRecognizer, NSView};
use objc2_foundation::NSRect;
use tokio::sync::mpsc::Sender;

use super::GestureEvent;

#[derive(Debug)]
pub struct Ivars {
    send: Sender<super::GestureEvent>,
}

define_class!(
    #[unsafe(super(NSView))]

    #[ivars = Ivars]
    pub struct InputView;

    impl InputView {

        #[unsafe(method(scrollWheel:))]
        #[cfg(feature = "gesture-pan")]
        fn scroll_wheel(&self, event: &NSEvent) {
            // let scroll = unsafe {(event.deltaX(), event.deltaY())};
            // let _ = self.ivars().send.try_send(GestureEvent::PanGesture { dx: scroll.0, dy: scroll.1 });
            println!("caught scroll");
            unsafe {
                if let Some(superview) = self.superview() {
                    superview.scrollWheel(event);
                }
            }
        }

        #[unsafe(method(tabletPoint:))]
        fn tablet_point(&self, event: &NSEvent) {
            let point = unsafe {(event.absoluteX(), event.absoluteY(), event.absoluteZ())};
            // let _ = self.ivars().send.try_send(GestureEvent::PanGesture { dx: scroll.0, dy: scroll.1 });
            println!("caught point: {point:?}");
            unsafe {
                if let Some(superview) = self.superview() {
                    superview.tabletPoint(event);
                }
            }
        }

        #[cfg(feature = "gesture-magnify")]
        #[unsafe(method(handleMagnify:))]
        fn handle_magnify(&self, recognizer: &NSMagnificationGestureRecognizer) {
            let mag = unsafe {recognizer.magnification()};

            let _ = self.ivars().send.try_send(GestureEvent::MagnifyGesture {scale: mag});
        }
    }


);

impl InputView {
    pub fn init_with_frame(
        frame: NSRect,
        mtm: MainThreadMarker,
        send: Sender<super::GestureEvent>,
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
