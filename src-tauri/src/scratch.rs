use core_graphics::event::{CGEvent, CGEventSource, CGEventSourceStateID};

fn main() {
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();
    let event = CGEvent::new(source).unwrap();
    let loc = event.location();
    println!("Mouse at: {}, {}", loc.x, loc.y);
}
