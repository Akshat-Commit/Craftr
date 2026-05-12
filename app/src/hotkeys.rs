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
pub fn start_hotkey_listener<F>(callback: F)
where
    F: Fn(HotkeyAction) + Send + Sync + 'static,
{
    let callback = Arc::new(callback);
    let pressed_keys: Arc<Mutex<HashSet<Key>>> = Arc::new(Mutex::new(HashSet::new()));

    std::thread::spawn(move || {
        let cb = callback.clone();
        let keys = pressed_keys.clone();

        let listener_callback = move |event: Event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    let mut pressed = keys.lock().unwrap();
                    pressed.insert(key);

                    // Check for hotkey combos
                    if let Some(action) = check_hotkey_combo(&pressed) {
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
fn check_hotkey_combo(pressed: &HashSet<Key>) -> Option<HotkeyAction> {
    let has_e = pressed.contains(&Key::KeyE);

    #[cfg(target_os = "windows")]
    {
        let has_ctrl =
            pressed.contains(&Key::ControlLeft) || pressed.contains(&Key::ControlRight);
        let has_shift = pressed.contains(&Key::ShiftLeft) || pressed.contains(&Key::ShiftRight);

        if has_ctrl && has_shift && has_e {
            return Some(HotkeyAction::Compress);
        }
        if has_ctrl && has_e && !has_shift {
            return Some(HotkeyAction::Enhance);
        }
    }

    #[cfg(target_os = "macos")]
    {
        let has_meta = pressed.contains(&Key::MetaLeft) || pressed.contains(&Key::MetaRight);
        let has_shift = pressed.contains(&Key::ShiftLeft) || pressed.contains(&Key::ShiftRight);

        if has_meta && has_shift && has_e {
            return Some(HotkeyAction::Compress);
        }
        if has_meta && has_e && !has_shift {
            return Some(HotkeyAction::Enhance);
        }
    }

    None
}
