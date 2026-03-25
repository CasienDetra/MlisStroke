use rdev::{listen, Event, EventType, Key};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub fn run_listener<F>(emit: F)
where
    F: Fn(String) + Send + Sync + 'static,
{
    let pressed_keys = Arc::new(Mutex::new(HashSet::<Key>::new()));
    let emit = Arc::new(emit);

    let callback = {
        let pressed_keys = pressed_keys.clone();
        let emit = emit.clone();

        move |event: Event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    let mut keys = pressed_keys.lock().unwrap();
                    keys.insert(key);

                    if keys.len() > 0 {
                        let combo = format_combo(&keys);
                        emit(combo);
                    }
                }

                EventType::KeyRelease(key) => {
                    let mut keys = pressed_keys.lock().unwrap();
                    keys.remove(&key);
                }

                _ => {}
            }
        }
    };

    if let Err(err) = listen(callback) {
        eprintln!("Error: {:?}", err);
    }
}

fn format_combo(keys: &HashSet<Key>) -> String {
    let mut parts: Vec<String> = keys
        .iter()
        .map(|k| key_to_string(k))
        .collect();

    parts = sort_keys(parts);

    parts.join(" + ")
}

fn sort_keys(mut keys: Vec<String>) -> Vec<String> {
    let order = ["Ctrl", "Shift", "Alt", "Meta"];

    keys.sort_by_key(|k| {
        order.iter().position(|&x| x == k).unwrap_or(100)
    });

    keys
}

fn key_to_string(key: &Key) -> String {
    match key {
        Key::ControlLeft | Key::ControlRight => "Ctrl".into(),
        Key::ShiftLeft | Key::ShiftRight => "Shift".into(),
        Key::Alt | Key::AltGr => "Alt".into(),
        Key::MetaLeft | Key::MetaRight => "Meta".into(),

        Key::Return => "Enter".into(),
        Key::Space => "Space".into(),

        Key::KeyA => "A".into(),
        Key::KeyB => "B".into(),
        Key::KeyC => "C".into(),
        Key::KeyD => "D".into(),
        Key::KeyE => "E".into(),
        Key::KeyF => "F".into(),
        Key::KeyG => "G".into(),
        Key::KeyH => "H".into(),
        Key::KeyI => "I".into(),
        Key::KeyJ => "J".into(),
        Key::KeyK => "K".into(),
        Key::KeyL => "L".into(),
        Key::KeyM => "M".into(),
        Key::KeyN => "N".into(),
        Key::KeyO => "O".into(),
        Key::KeyP => "P".into(),
        Key::KeyQ => "Q".into(),
        Key::KeyR => "R".into(),
        Key::KeyS => "S".into(),
        Key::KeyT => "T".into(),
        Key::KeyU => "U".into(),
        Key::KeyV => "V".into(),
        Key::KeyW => "W".into(),
        Key::KeyX => "X".into(),
        Key::KeyY => "Y".into(),
        Key::KeyZ => "Z".into(),

        _ => format!("{:?}", key),
    }
}
