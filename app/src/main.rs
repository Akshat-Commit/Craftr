// main.rs — Craftr: System tray app for AI prompt enhancement
//
// All stats shown directly in the tray right-click menu.
// No popup windows. Clean and reliable.

#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod api;
mod clipboard;
mod config;
mod hotkeys;

use api::PromptMode;
use hotkeys::HotkeyAction;
use native_dialog::{MessageDialog, MessageType};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder,
};
use chrono::Local;

const ID_UPGRADE: &str = "upgrade";
const ID_QUIT: &str = "quit";

fn main() {
    // Start global hotkey listener
    hotkeys::start_hotkey_listener(move |action| {
        let mode = match action {
            HotkeyAction::Enhance => PromptMode::Enhance,
            HotkeyAction::Compress => PromptMode::Compress,
        };
        process_clipboard(mode);
    });

    // Build tray menu with dynamic stat labels
    let menu = Menu::new();

    let label_item = MenuItem::with_id("label", "⚡ Craftr", false, None);
    let usage_item = MenuItem::with_id("usage", "📊 Loading...", false, None);
    let timer_item = MenuItem::with_id("timer", "⏰ Loading...", false, None);
    let upgrade_item = MenuItem::with_id(ID_UPGRADE, "✨ Upgrade to Pro →", true, None);
    let quit_item = MenuItem::with_id(ID_QUIT, "Quit", true, None);

    menu.append(&label_item).ok();
    menu.append(&PredefinedMenuItem::separator()).ok();
    menu.append(&usage_item).ok();
    menu.append(&timer_item).ok();
    menu.append(&PredefinedMenuItem::separator()).ok();
    menu.append(&upgrade_item).ok();
    menu.append(&PredefinedMenuItem::separator()).ok();
    menu.append(&quit_item).ok();

    let icon = create_tray_icon();

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Craftr — AI Prompt Power-ups")
        .with_icon(icon)
        .build()
        .expect("Failed to create tray icon");

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    let menu_receiver = MenuEvent::receiver();

    // Track last update time so we refresh stats regularly
    let mut last_update = std::time::Instant::now();

    // Initial stats update
    update_menu_stats(&usage_item, &timer_item, &upgrade_item);

    while running_clone.load(Ordering::Relaxed) {
        // Update menu stats every 2 seconds
        if last_update.elapsed().as_secs() >= 2 {
            update_menu_stats(&usage_item, &timer_item, &upgrade_item);
            last_update = std::time::Instant::now();
        }

        if let Ok(event) = menu_receiver.recv_timeout(std::time::Duration::from_millis(100)) {
            match event.id.0.as_str() {
                ID_UPGRADE => {
                    let _ = opener::open("https://craftr.app/upgrade");
                }
                ID_QUIT => {
                    running_clone.store(false, Ordering::Relaxed);
                }
                _ => {}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

/// Update the tray menu labels with current stats from config
fn update_menu_stats(
    usage_item: &MenuItem,
    timer_item: &MenuItem,
    upgrade_item: &MenuItem,
) {
    let cfg = config::load_config();

    if cfg.is_pro {
        usage_item.set_text("✅ Pro — Unlimited requests");
        timer_item.set_text("");
        upgrade_item.set_enabled(false);
        upgrade_item.set_text("✅ Pro Active");
    } else {
        let used = cfg.requests_today;
        let remaining = 10u32.saturating_sub(used);
        usage_item.set_text(&format!("📊 {}/10 requests used today", used));

        let now = Local::now();
        let hours_left = 23 - now.hour();
        let mins_left = 59 - now.minute();
        timer_item.set_text(&format!("⏰ Resets in {}h {}m", hours_left, mins_left));

        upgrade_item.set_enabled(true);
        upgrade_item.set_text("✨ Upgrade to Pro →");
    }
}

fn process_clipboard(mode: PromptMode) {
    match config::check_daily_limit() {
        Ok(true) => {}
        Ok(false) => {
            show_upgrade_notification();
            return;
        }
        Err(e) => {
            show_error(&format!("Limit check error: {}", e));
            return;
        }
    }

    let text = match clipboard::read_clipboard() {
        Ok(t) if !t.trim().is_empty() => t,
        Ok(_) => {
            show_error("No text selected — clipboard is empty.");
            return;
        }
        Err(e) => {
            show_error(&format!("Clipboard error: {}", e));
            return;
        }
    };

    match api::call_groq(&text, mode) {
        Ok(result) => {
            if let Err(e) = clipboard::write_clipboard(&result) {
                show_error(&format!("Failed to write to clipboard: {}", e));
                return;
            }
            clipboard::simulate_paste();
            config::increment_request_count();
        }
        Err(e) => {
            show_error(&format!("Groq API error: {}", e));
        }
    }
}

fn show_upgrade_notification() {
    let result = MessageDialog::new()
        .set_title("Craftr — Daily Limit Reached")
        .set_text("Daily limit reached (10/10).\n\nUpgrade to Pro for unlimited use.\n\nClick OK to open the upgrade page.")
        .set_type(MessageType::Info)
        .show_confirm()
        .unwrap_or(false);

    if result {
        let _ = opener::open("https://craftr.app/upgrade");
    }
}

fn show_error(msg: &str) {
    MessageDialog::new()
        .set_title("Craftr Error")
        .set_text(msg)
        .show_alert()
        .ok();
}

fn create_tray_icon() -> tray_icon::Icon {
    let size = 32u32;
    let mut rgba = vec![0u8; (size * size * 4) as usize];
    let bg = (10u8, 10u8, 10u8);
    let accent = (170u8, 255u8, 0u8);

    for y in 0..size {
        for x in 0..size {
            let idx = ((y * size + x) * 4) as usize;
            let cx = (x as f32 - size as f32 / 2.0) / (size as f32 / 2.0);
            let cy = (y as f32 - size as f32 / 2.0) / (size as f32 / 2.0);
            let dist = cx.abs().max(cy.abs());

            if dist < 0.9 {
                let is_c = (x >= 7 && x <= 25 && y >= 5 && y <= 8)
                    || (x >= 7 && x <= 10 && y >= 8 && y <= 23)
                    || (x >= 7 && x <= 25 && y >= 23 && y <= 26);

                let is_bolt = (x >= 17 && x <= 21 && y >= 8 && y <= 11)
                    || (x >= 15 && x <= 19 && y >= 11 && y <= 14)
                    || (x >= 13 && x <= 21 && y >= 14 && y <= 16)
                    || (x >= 15 && x <= 19 && y >= 16 && y <= 19)
                    || (x >= 13 && x <= 17 && y >= 19 && y <= 23);

                if is_c || is_bolt {
                    rgba[idx] = accent.0;
                    rgba[idx + 1] = accent.1;
                    rgba[idx + 2] = accent.2;
                    rgba[idx + 3] = 255;
                } else {
                    rgba[idx] = bg.0;
                    rgba[idx + 1] = bg.1;
                    rgba[idx + 2] = bg.2;
                    rgba[idx + 3] = 255;
                }
            } else if dist < 0.98 {
                let alpha = ((0.98 - dist) / 0.08 * 255.0) as u8;
                rgba[idx] = bg.0;
                rgba[idx + 1] = bg.1;
                rgba[idx + 2] = bg.2;
                rgba[idx + 3] = alpha;
            }
        }
    }

    tray_icon::Icon::from_rgba(rgba, size, size).expect("Failed to create tray icon")
}

use chrono::Timelike;
