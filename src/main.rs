use rdev::{listen, Event, EventType, Button, simulate, Key, SimulateError};
use std::sync::{Arc, Mutex};
use std::{thread, time};

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
    if *key == 9 {
        *key = 0;
    } else {
        *key += 1;
    }
    output(*key);
}

fn backward(key: &Arc<Mutex<u8>>) {
    let mut key = key.lock().unwrap();
    if *key == 0 {
        *key = 9;
    } else {
        *key -= 1;
    }
    output(*key);
}

// Handle output
fn output(key: u8) {
    println!("Output Key: {}", key);
    if key == 10 {
        press_number(0);
    } else {
        press_number(key);
    }
}

fn press_number(number: u8) {
    // Convert number to the corresponding key
    let key = match number {
        0 => Key::Num0,
        1 => Key::Num1,
        2 => Key::Num2,
        3 => Key::Num3,
        4 => Key::Num4,
        5 => Key::Num5,
        6 => Key::Num6,
        7 => Key::Num7,
        8 => Key::Num8,
        9 => Key::Num9,
        _ => unreachable!(),
    };

    // Debug statement to check the key being pressed
    println!("Pressing Key: {:?}", key);

    // Simulate the key press and release with delays
    if let Err(SimulateError) = simulate(&EventType::KeyPress(key)) {
        println!("Failed to simulate key press");
    }
    thread::sleep(time::Duration::from_millis(50)); // 50 ms delay
    if let Err(SimulateError) = simulate(&EventType::KeyRelease(key)) {
        println!("Failed to simulate key release");
    }
    println!("Released Key: {:?}", key);
}

