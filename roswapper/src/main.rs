use rdev::{listen, Event, EventType, Button};
fn main() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}

fn callback(event: Event) {
    match event.event_type {
        EventType::ButtonPress(button) => {
            match button {
                Button::Unknown(1) => backward(),
                Button::Unknown(2) => forward(),
                _ => (),
            }
        },
        _ => (),
    }
}


fn forward() {
    println!("Side Button 2 (Forward) pressed");
}

fn backward() {
    println!("Side Button 1 (Back) pressed");
}