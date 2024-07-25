// Libaries
use rdev::{listen, Event, EventType, Button};

// Variables
static mut KEY: u8 = 1;

//Functions

// Main - Start
fn main() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}

// Detect keys
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

// Handle keys
fn forward() {
    unsafe {
        KEY = KEY + 1
    }
    output();
}

fn backward() {
    unsafe {
        KEY = KEY - 1
    }
    output();
}

// Handle output
fn output() {
    unsafe {
        println!("{}", KEY);
    }
}
