// clipboard.rs — Clipboard read/write and paste simulation
//
// Uses `arboard` for clipboard access and `rdev` for key simulation

use arboard::Clipboard;
use rdev::{simulate, EventType, Key};
use std::thread;
use std::time::Duration;

/// Small delay between key events to ensure OS registers them
const KEY_DELAY_MS: u64 = 50;

/// Read the current clipboard text content
pub fn read_clipboard() -> Result<String, String> {
    let mut clipboard =
        Clipboard::new().map_err(|e| format!("Failed to access clipboard: {}", e))?;

    clipboard
        .get_text()
        .map_err(|e| format!("Failed to read clipboard: {}", e))
}

/// Write text to the clipboard
pub fn write_clipboard(text: &str) -> Result<(), String> {
    let mut clipboard =
        Clipboard::new().map_err(|e| format!("Failed to access clipboard: {}", e))?;

    clipboard
        .set_text(text.to_string())
        .map_err(|e| format!("Failed to write clipboard: {}", e))
}

/// Helper to simulate a single key event with a small delay
fn send_key(event_type: &EventType) {
    simulate(event_type).ok();
    thread::sleep(Duration::from_millis(KEY_DELAY_MS));
}

/// Simulate a paste keystroke (Ctrl+V on Windows, Cmd+V on macOS)
/// This makes the processed text appear in the active application
pub fn simulate_paste() {
    // Small delay to let the clipboard settle
    thread::sleep(Duration::from_millis(100));

    #[cfg(target_os = "windows")]
    {
        send_key(&EventType::KeyPress(Key::ControlLeft));
        send_key(&EventType::KeyPress(Key::KeyV));
        send_key(&EventType::KeyRelease(Key::KeyV));
        send_key(&EventType::KeyRelease(Key::ControlLeft));
    }

    #[cfg(target_os = "macos")]
    {
        send_key(&EventType::KeyPress(Key::MetaLeft));
        send_key(&EventType::KeyPress(Key::KeyV));
        send_key(&EventType::KeyRelease(Key::KeyV));
        send_key(&EventType::KeyRelease(Key::MetaLeft));
    }
}
