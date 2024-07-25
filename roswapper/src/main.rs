use rdev::{listen, Event, EventType, Button};
use std::sync::{Arc, Mutex};

// Main - Start
fn main() {
    // Wrap the shared state in an Arc<Mutex<_>> for thread-safe access
    let key = Arc::new(Mutex::new(1));

    // Clone the Arc to pass it to the callback function
    let key_clone = Arc::clone(&key);

    if let Err(error) = listen(move |event| callback(event, &key_clone)) {
        println!("Error: {:?}", error);
    }
}

// Detect keys
fn callback(event: Event, key: &Arc<Mutex<u8>>) {
    match event.event_type {
        EventType::ButtonPress(button) => {
            match button {
                Button::Unknown(1) => backward(key),
                Button::Unknown(2) => forward(key),
                _ => (),
            }
        },
        _ => (),
    }
}

// Handle keys
fn forward(key: &Arc<Mutex<u8>>) {
    let mut key = key.lock().unwrap();
    if *key == 10 {
        *key = 1;
    } else {
        *key += 1;
    }
    output(*key);
}

fn backward(key: &Arc<Mutex<u8>>) {
    let mut key = key.lock().unwrap();
    if *key == 1 {
        *key = 10;
    } else {
        *key -= 1;
    }
    
    output(*key);
}

// Handle output
fn output(key: u8) {
    println!("{}", key);
}
