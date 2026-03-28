use rdev::{listen, Event, EventType, Key};
use std::sync::{Arc, Mutex};

pub fn run_listener<F>(emit: F)
where
    F: Fn(String) + Send + Sync + 'static,
{
    let pressed_keys = Arc::new(Mutex::new(Vec::<Key>::new()));
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
                    // Only add key if it's not already pressed (avoid duplicates)
                    if !keys.contains(&key) {
                        keys.push(key);
                    }

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
                    keys.retain(|k| k != &key);
                }

                _ => {}
            }
        }
    };

    if let Err(err) = listen(callback) {
        eprintln!("Error: {:?}", err);
    }
}

// ============================================================================
// COMBO FORMATTING
// ============================================================================

fn format_combo(keys: &Vec<Key>, caps_lock_on: bool) -> String {
    let has_shift = keys
        .iter()
        .any(|k| matches!(k, Key::ShiftLeft | Key::ShiftRight));
    let has_caps_lock = keys.iter().any(|k| k == &Key::CapsLock);

    let parts: Vec<String> = keys
        .iter()
        .map(|k| key_to_string(k, has_shift, caps_lock_on))
        .filter(|s| !s.is_empty()) // Filter out empty strings from ignored keys
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
    // (i.e., when there are other keys pressed AND shift is not needed for them)
    if has_shift && !has_caps_lock && !other_keys.is_empty() {
        modifiers.retain(|p| p != "Shift");
    }

    // Convert modifier names to icons and sort
    let modifier_icons: Vec<String> = modifiers.iter().map(|m| modifier_to_icon(m)).collect();
    let sorted_icons = sort_modifier_icons(modifier_icons);

    // Convert other keys to icons where applicable
    let other_icons: Vec<String> = other_keys.iter().map(|k| key_to_icon(k)).collect();

    // Don't sort other keys - preserve the order they were pressed
    // Just convert icons, no reordering needed

    // Combine: modifiers first, then other keys
    let mut result = sorted_icons;
    result.extend(other_icons);

    result.join(" ")
}

//
// ICON CONVERSION
//

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

//
// KEY STRING CONVERSION
//

fn key_to_string(key: &Key, has_shift: bool, caps_lock_on: bool) -> String {
    match key {
        // Modifier keys
        Key::ControlLeft | Key::ControlRight => "Ctrl".into(),
        Key::ShiftLeft | Key::ShiftRight => "Shift".into(),
        Key::Alt | Key::AltGr => "Alt".into(),
        Key::MetaLeft | Key::MetaRight => "Meta".into(),
        Key::CapsLock => "CapsLock".into(),

        // Special navigation keys
        Key::Return => "Enter".into(),
        Key::Space => "Space".into(),
        Key::Tab => "Tab".into(),

        // Letters A-Z
        Key::KeyA => convert_letter("a", "A", has_shift, caps_lock_on),
        Key::KeyB => convert_letter("b", "B", has_shift, caps_lock_on),
        Key::KeyC => convert_letter("c", "C", has_shift, caps_lock_on),
        Key::KeyD => convert_letter("d", "D", has_shift, caps_lock_on),
        Key::KeyE => convert_letter("e", "E", has_shift, caps_lock_on),
        Key::KeyF => convert_letter("f", "F", has_shift, caps_lock_on),
        Key::KeyG => convert_letter("g", "G", has_shift, caps_lock_on),
        Key::KeyH => convert_letter("h", "H", has_shift, caps_lock_on),
        Key::KeyI => convert_letter("i", "I", has_shift, caps_lock_on),
        Key::KeyJ => convert_letter("j", "J", has_shift, caps_lock_on),
        Key::KeyK => convert_letter("k", "K", has_shift, caps_lock_on),
        Key::KeyL => convert_letter("l", "L", has_shift, caps_lock_on),
        Key::KeyM => convert_letter("m", "M", has_shift, caps_lock_on),
        Key::KeyN => convert_letter("n", "N", has_shift, caps_lock_on),
        Key::KeyO => convert_letter("o", "O", has_shift, caps_lock_on),
        Key::KeyP => convert_letter("p", "P", has_shift, caps_lock_on),
        Key::KeyQ => convert_letter("q", "Q", has_shift, caps_lock_on),
        Key::KeyR => convert_letter("r", "R", has_shift, caps_lock_on),
        Key::KeyS => convert_letter("s", "S", has_shift, caps_lock_on),
        Key::KeyT => convert_letter("t", "T", has_shift, caps_lock_on),
        Key::KeyU => convert_letter("u", "U", has_shift, caps_lock_on),
        Key::KeyV => convert_letter("v", "V", has_shift, caps_lock_on),
        Key::KeyW => convert_letter("w", "W", has_shift, caps_lock_on),
        Key::KeyX => convert_letter("x", "X", has_shift, caps_lock_on),
        Key::KeyY => convert_letter("y", "Y", has_shift, caps_lock_on),
        Key::KeyZ => convert_letter("z", "Z", has_shift, caps_lock_on),

        // Bracket characters
        Key::LeftBracket => convert_char("[", "{", has_shift),
        Key::RightBracket => convert_char("]", "}", has_shift),

        // Punctuation characters
        Key::SemiColon => convert_char(";", ":", has_shift),
        Key::Quote => convert_char("'", "\"", has_shift),
        Key::Comma => convert_char(",", "<", has_shift),
        Key::Dot => convert_char(".", ">", has_shift),
        Key::Slash => convert_char("/", "?", has_shift),
        Key::BackSlash => convert_char("\\", "|", has_shift),
        Key::BackQuote => convert_char("`", "~", has_shift),

        // Operator characters
        Key::Minus => convert_char("-", "_", has_shift),
        Key::Equal => convert_char("=", "+", has_shift),

        // Default case for unknown keys - return empty string if it contains parentheses
        _ => {
            let key_str = format!("{:?}", key);
            if key_str.contains('(') || key_str.contains(')') {
                String::new()
            } else {
                key_str
            }
        }
    }
}

//
// HELPER FUNCTIONS
//

/// Convert a letter character based on shift and caps lock state
fn convert_letter(lowercase: &str, uppercase: &str, has_shift: bool, caps_lock_on: bool) -> String {
    if has_shift || caps_lock_on {
        uppercase.into()
    } else {
        lowercase.into()
    }
}

/// Convert a character based on shift state only
fn convert_char(normal: &str, shifted: &str, has_shift: bool) -> String {
    if has_shift {
        shifted.into()
    } else {
        normal.into()
    }
}
