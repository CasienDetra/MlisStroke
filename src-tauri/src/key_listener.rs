use rdev::{listen, Event, EventType, Key};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub fn run_listener<F>(emit: F)
where
    F: Fn(String) + Send + Sync + 'static,
{
    let pressed_keys = Arc::new(Mutex::new(HashSet::<Key>::new()));
    let last_combo = Arc::new(Mutex::new(String::new()));
    let caps_lock_on = Arc::new(Mutex::new(false));
    let emit = Arc::new(emit);

    let callback = {
        let pressed_keys = pressed_keys.clone();
        let last_combo = last_combo.clone();
        let caps_lock_on = caps_lock_on.clone();
        let emit = emit.clone();

        move |event: Event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    // Track caps lock state
                    if key == Key::CapsLock {
                        let mut caps = caps_lock_on.lock().unwrap();
                        *caps = !*caps;
                    }

                    let mut keys = pressed_keys.lock().unwrap();
                    keys.insert(key);

                    let caps = caps_lock_on.lock().unwrap();
                    let combo = format_combo(&keys, *caps);

                    let mut last = last_combo.lock().unwrap();
                    if *last != combo {
                        *last = combo.clone();
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

fn format_combo(keys: &HashSet<Key>, caps_lock_on: bool) -> String {
    let has_shift = keys
        .iter()
        .any(|k| matches!(k, Key::ShiftLeft | Key::ShiftRight));
    let has_caps_lock = keys.iter().any(|k| k == &Key::CapsLock);

    let parts: Vec<String> = keys
        .iter()
        .map(|k| key_to_string(k, has_shift, caps_lock_on))
        .collect();

    // Separate modifiers from other keys (letters, special keys, etc.)
    let mut modifiers: Vec<String> = Vec::new();
    let mut other_keys: Vec<String> = Vec::new();

    for part in parts {
        if part == "Ctrl" || part == "Shift" || part == "Alt" || part == "Meta" {
            modifiers.push(part);
        } else {
            other_keys.push(part);
        }
    }

    // Remove "Shift" from modifiers if it's only being used for letter capitalization
    if has_shift && !has_caps_lock {
        modifiers.retain(|p| p != "Shift");
    }

    // Convert modifier names to icons and sort
    let modifier_icons: Vec<String> = modifiers.iter().map(|m| modifier_to_icon(m)).collect();
    let sorted_icons = sort_modifier_icons(modifier_icons);

    // Convert other keys to icons where applicable and sort
    let other_icons: Vec<String> = other_keys.iter().map(|k| key_to_icon(k)).collect();
    let mut sorted_others: Vec<String> = other_icons.clone();

    // Sort non-modifier keys: letters alphabetically, special keys as-is
    sorted_others.sort_by(|a, b| {
        let a_is_letter = a.chars().all(|c| c.is_alphabetic());
        let b_is_letter = b.chars().all(|c| c.is_alphabetic());

        match (a_is_letter, b_is_letter) {
            (true, true) => a.to_lowercase().cmp(&b.to_lowercase()),
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            (false, false) => a.cmp(b),
        }
    });

    // Combine: modifiers first, then other keys
    let mut result = sorted_icons;
    result.extend(sorted_others);

    result.join(" ")
}

fn modifier_to_icon(modifier: &str) -> String {
    match modifier {
        "Ctrl" => "⌃".to_string(),
        "Shift" => "⇧".to_string(),
        "Alt" => "⌥".to_string(),
        "Meta" => "⌘".to_string(),
        _ => modifier.to_string(),
    }
}

fn key_to_icon(key: &str) -> String {
    match key {
        "Enter" => "⏎".to_string(),
        "Space" => "␣".to_string(),
        "Tab" => "⇆".to_string(),
        "CapsLock" => "⇪".to_string(),
        _ => key.to_string(),
    }
}

fn sort_modifier_icons(mut keys: Vec<String>) -> Vec<String> {
    let order = ["⌃", "⇧", "⌥", "⌘"];

    keys.sort_by(|a, b| {
        let a_pos = order.iter().position(|&x| x == a).unwrap_or(100);
        let b_pos = order.iter().position(|&x| x == b).unwrap_or(100);

        a_pos.cmp(&b_pos)
    });

    keys
}

fn key_to_string(key: &Key, has_shift: bool, caps_lock_on: bool) -> String {
    match key {
        Key::ControlLeft | Key::ControlRight => "Ctrl".into(),
        Key::ShiftLeft | Key::ShiftRight => "Shift".into(),
        Key::Alt | Key::AltGr => "Alt".into(),
        Key::MetaLeft | Key::MetaRight => "Meta".into(),
        Key::CapsLock => "CapsLock".into(),

        Key::Return => "Enter".into(),
        Key::Space => "Space".into(),

        Key::KeyA => {
            if has_shift || caps_lock_on {
                "A".into()
            } else {
                "a".into()
            }
        }
        Key::KeyB => {
            if has_shift || caps_lock_on {
                "B".into()
            } else {
                "b".into()
            }
        }
        Key::KeyC => {
            if has_shift || caps_lock_on {
                "C".into()
            } else {
                "c".into()
            }
        }
        Key::KeyD => {
            if has_shift || caps_lock_on {
                "D".into()
            } else {
                "d".into()
            }
        }
        Key::KeyE => {
            if has_shift || caps_lock_on {
                "E".into()
            } else {
                "e".into()
            }
        }
        Key::KeyF => {
            if has_shift || caps_lock_on {
                "F".into()
            } else {
                "f".into()
            }
        }
        Key::KeyG => {
            if has_shift || caps_lock_on {
                "G".into()
            } else {
                "g".into()
            }
        }
        Key::KeyH => {
            if has_shift || caps_lock_on {
                "H".into()
            } else {
                "h".into()
            }
        }
        Key::KeyI => {
            if has_shift || caps_lock_on {
                "I".into()
            } else {
                "i".into()
            }
        }
        Key::KeyJ => {
            if has_shift || caps_lock_on {
                "J".into()
            } else {
                "j".into()
            }
        }
        Key::KeyK => {
            if has_shift || caps_lock_on {
                "K".into()
            } else {
                "k".into()
            }
        }
        Key::KeyL => {
            if has_shift || caps_lock_on {
                "L".into()
            } else {
                "l".into()
            }
        }
        Key::KeyM => {
            if has_shift || caps_lock_on {
                "M".into()
            } else {
                "m".into()
            }
        }
        Key::KeyN => {
            if has_shift || caps_lock_on {
                "N".into()
            } else {
                "n".into()
            }
        }
        Key::KeyO => {
            if has_shift || caps_lock_on {
                "O".into()
            } else {
                "o".into()
            }
        }
        Key::KeyP => {
            if has_shift || caps_lock_on {
                "P".into()
            } else {
                "p".into()
            }
        }
        Key::KeyQ => {
            if has_shift || caps_lock_on {
                "Q".into()
            } else {
                "q".into()
            }
        }
        Key::KeyR => {
            if has_shift || caps_lock_on {
                "R".into()
            } else {
                "r".into()
            }
        }
        Key::KeyS => {
            if has_shift || caps_lock_on {
                "S".into()
            } else {
                "s".into()
            }
        }
        Key::KeyT => {
            if has_shift || caps_lock_on {
                "T".into()
            } else {
                "t".into()
            }
        }
        Key::KeyU => {
            if has_shift || caps_lock_on {
                "U".into()
            } else {
                "u".into()
            }
        }
        Key::KeyV => {
            if has_shift || caps_lock_on {
                "V".into()
            } else {
                "v".into()
            }
        }
        Key::KeyW => {
            if has_shift || caps_lock_on {
                "W".into()
            } else {
                "w".into()
            }
        }
        Key::KeyX => {
            if has_shift || caps_lock_on {
                "X".into()
            } else {
                "x".into()
            }
        }
        Key::KeyY => {
            if has_shift || caps_lock_on {
                "Y".into()
            } else {
                "y".into()
            }
        }
        Key::KeyZ => {
            if has_shift || caps_lock_on {
                "Z".into()
            } else {
                "z".into()
            }
        }

        _ => format!("{:?}", key),
    }
}
