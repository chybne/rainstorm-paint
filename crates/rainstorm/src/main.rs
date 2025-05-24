use iced;
use ui::{self, Rainstorm};

pub fn main() -> iced::Result {
    iced::application("my first app", Rainstorm::update, Rainstorm::view).run()
}
