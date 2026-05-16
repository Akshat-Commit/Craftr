// hotkeys.rs — Global hotkey detection using rdev
//
// Platform-specific hotkey bindings:
//   Windows: Ctrl+E (enhance), Ctrl+Shift+E (compress)
//   macOS:   Cmd+E (enhance),  Cmd+Shift+E (compress)

use rdev::{listen, Event, EventType, Key};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

/// The action triggered by a hotkey combo
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HotkeyAction {
    Enhance,
    Compress,
}

/// Start listening for global hotkeys in a background thread.
///
/// The `callback` is invoked on the listener thread whenever a
/// matching hotkey combination is detected.
pub fn start_hotkey_listener<F>(callback: F, current_hotkeys: Arc<Mutex<(String, String)>>)
where
    F: Fn(HotkeyAction) + Send + Sync + 'static,
{
    let callback = Arc::new(callback);
    let pressed_keys: Arc<Mutex<HashSet<Key>>> = Arc::new(Mutex::new(HashSet::new()));

    std::thread::spawn(move || {
        let cb = callback.clone();
        let keys = pressed_keys.clone();
        let hotkeys_ref = current_hotkeys.clone();

        let listener_callback = move |event: Event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    let mut pressed = keys.lock().unwrap();
                    pressed.insert(key);

                    // Check for hotkey combos
                    let hotkeys = hotkeys_ref.lock().unwrap().clone();
                    if let Some(action) = check_hotkey_combo(&pressed, &hotkeys) {
                        // Clear pressed keys to avoid re-triggering
                        pressed.clear();
                        drop(pressed);
                        cb(action);
                    }

                }
                EventType::KeyRelease(key) => {
                    let mut pressed = keys.lock().unwrap();
                    pressed.remove(&key);
                }
                _ => {}
            }
        };

        // This blocks forever, listening for all input events
        if let Err(e) = listen(listener_callback) {
            eprintln!("Hotkey listener error: {:?}", e);
        }
    });
}

/// Check if the currently pressed keys match a hotkey combination
fn check_hotkey_combo(pressed: &HashSet<Key>, hotkeys: &(String, String)) -> Option<HotkeyAction> {
    if matches_hotkey(pressed, &hotkeys.1) {
        return Some(HotkeyAction::Compress);
    }
    if matches_hotkey(pressed, &hotkeys.0) {
        return Some(HotkeyAction::Enhance);
    }
    None
}

fn matches_hotkey(pressed: &HashSet<Key>, combo_str: &str) -> bool {
    let combo_str = combo_str.to_lowercase();
    let parts: Vec<&str> = combo_str.split('+').collect();
    
    let mut needs_ctrl = false;
    let mut needs_shift = false;
    let mut needs_alt = false;
    let mut needs_meta = false;
    let mut main_key = None;

    for part in parts {
        match part.trim() {
            "ctrl" => needs_ctrl = true,
            "shift" => needs_shift = true,
            "alt" => needs_alt = true,
            "cmd" | "win" | "meta" => needs_meta = true,
            k => main_key = Some(k.to_string()),
        }
    }

    let has_ctrl = pressed.contains(&Key::ControlLeft) || pressed.contains(&Key::ControlRight);
    let has_shift = pressed.contains(&Key::ShiftLeft) || pressed.contains(&Key::ShiftRight);
    let has_alt = pressed.contains(&Key::Alt) || pressed.contains(&Key::AltGr);
    let has_meta = pressed.contains(&Key::MetaLeft) || pressed.contains(&Key::MetaRight);

    if has_ctrl != needs_ctrl || has_shift != needs_shift || has_alt != needs_alt || has_meta != needs_meta {
        return false;
    }

    if let Some(mk) = main_key {
        for k in pressed {
            if key_to_string(k) == mk {
                return true;
            }
        }
    }
    false
}

fn key_to_string(key: &Key) -> String {
    let s = format!("{:?}", key).to_lowercase();
    if s.starts_with("key") {
        s.replace("key", "")
    } else if s.starts_with("num") {
        s.replace("num", "")
    } else if s.starts_with("f") {
        s // f1-f12
    } else {
        s
    }
}

