use evdev::{Device, EventType, Key};
use std::io;

// Function to scan input events and return a complete sequence
pub fn scan(device: &mut Device) -> io::Result<String> {
    let mut key_sequence: Vec<u8> = Vec::new();

    // Loop to read events
    loop {
        for event in device.fetch_events()? {
            // Check if it's a key release event
            if event.value() == 0 && matches!(event.event_type(), EventType::KEY) {
                let key = Key::new(event.code());

                // Handle numeric keys
                if let Some(num) = key_to_number(key) {
                    key_sequence.push(num);
                }

                // Handle Enter key to finalize and return the sequence
                if key == Key::KEY_ENTER {
                    let result: String = key_sequence.iter().map(|n| n.to_string()).collect();
                    // Clear the sequence after processing
                    key_sequence.clear();
                    return Ok(result); // Return the complete sequence
                }
            }
        }
    }
}

// Helper function to map keys like KEY_0 -> 0, KEY_1 -> 1, etc.
fn key_to_number(key: Key) -> Option<u8> {
    match key {
        Key::KEY_0 => Some(0),
        Key::KEY_1 => Some(1),
        Key::KEY_2 => Some(2),
        Key::KEY_3 => Some(3),
        Key::KEY_4 => Some(4),
        Key::KEY_5 => Some(5),
        Key::KEY_6 => Some(6),
        Key::KEY_7 => Some(7),
        Key::KEY_8 => Some(8),
        Key::KEY_9 => Some(9),
        _ => None, // Ignore non-numeric keys
    }
}
