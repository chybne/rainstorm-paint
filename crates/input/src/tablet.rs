use objc2_core_foundation::{CFMachPort, CFRunLoop, kCFRunLoopCommonModes};
use objc2_core_graphics::{
    self, CGEvent, CGEventField, CGEventMask, CGEventTapLocation, CGEventTapOptions,
    CGEventTapPlacement, CGEventTapProxy, CGEventType,
};
use std::ffi::c_void;
use std::ops::Deref;
use std::ptr::NonNull;
use std::thread;
use tokio::sync::mpsc::Sender;

unsafe extern "C-unwind" fn call_back(
    _proxy: CGEventTapProxy,
    type_: CGEventType,
    event: NonNull<CGEvent>,
    _user_info: *mut c_void,
) -> *mut CGEvent {
    println!("{type_:?}");

    {
        let event = unsafe { event.as_ref() };

        match type_ {
            CGEventType::TabletPointer => {
                let coords = unsafe {
                    (
                        CGEvent::integer_value_field(Some(event), CGEventField::TabletEventPointX),
                        CGEvent::integer_value_field(Some(event), CGEventField::TabletEventPointY),
                        CGEvent::integer_value_field(Some(event), CGEventField::TabletEventPointZ),
                    )
                };

                println!("{coords:?}");
            }
            CGEventType::TabletProximity => {
                let proximity = unsafe {
                    CGEvent::double_value_field(
                        Some(event),
                        CGEventField::TabletProximityEventEnterProximity,
                    )
                };
                println!("{proximity:?}");
            }
            _ => (),
        }
    }

    event.as_ptr()
}

pub fn start_tracking_tablet(sender: Sender<TabletEvent>) {
    thread::spawn(move || unsafe {
        let boxed_sender = Box::new(sender);
        // let sender_ptr = Box::into_raw(b)

        let mask: CGEventMask = 1u64 << CGEventType::TabletPointer.0 as u64
            | 1u64 << CGEventType::TabletProximity.0 as u64;

        let tap = CGEvent::tap_create(
            CGEventTapLocation::HIDEventTap,
            CGEventTapPlacement::HeadInsertEventTap,
            CGEventTapOptions::Default,
            mask,
            Some(call_back),
            std::ptr::null_mut(),
        )
        .expect("failed to create tap");

        let t = boxed_sender.deref();
        let _ = t.send(TabletEvent::None);
        let source = CFMachPort::new_run_loop_source(None, Some(&tap), 0)
            .expect("couldnt create new run loop source");

        let run_loop = CFRunLoop::current().expect("no run loop");

        run_loop.add_source(Some(&source), kCFRunLoopCommonModes);

        CFRunLoop::run();
    });
}

#[derive(Debug)]
pub enum TabletEvent {
    None,
}
