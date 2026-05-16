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
mod popup_win32;
mod settings_win32;


use api::PromptMode;
use hotkeys::HotkeyAction;
use native_dialog::{MessageDialog, MessageType};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder, TrayIconEvent,
};
use chrono::Local;

const ID_UPGRADE: &str = "upgrade";
const ID_STARTUP: &str = "startup";
const ID_HOTKEYS: &str = "hotkeys";
const ID_QUIT: &str = "quit";

fn main() {
    log("App started");
    config::enforce_daily_reset();

    let cfg = config::load_config();
    let hotkeys_state = Arc::new(Mutex::new((cfg.enhance_hotkey.clone(), cfg.compress_hotkey.clone())));
    let hotkeys_clone = hotkeys_state.clone();

    // Start global hotkey listener
    hotkeys::start_hotkey_listener(move |action| {
        let mode = match action {
            HotkeyAction::Enhance => PromptMode::Enhance,
            HotkeyAction::Compress => PromptMode::Compress,
        };
        process_clipboard(mode);
    }, hotkeys_clone);

    // Build tray menu with dynamic stat labels
    let menu = Menu::new();

    let label_item = MenuItem::with_id("label", "⚡ Craftr", false, None);
    let usage_item = MenuItem::with_id("usage", "📊 Loading...", false, None);
    let timer_item = MenuItem::with_id("timer", "⏰ Loading...", false, None);
    let upgrade_item = MenuItem::with_id(ID_UPGRADE, "✨ Upgrade to Pro →", true, None);
    let quit_item = MenuItem::with_id(ID_QUIT, "Quit", true, None);

    let cfg = config::load_config();
    update_startup_registry(cfg.launch_at_startup);

    let startup_text = if cfg.launch_at_startup { "✓ Launch at startup" } else { "Launch at startup" };
    let startup_item = MenuItem::with_id(ID_STARTUP, startup_text, true, None);
    let hotkeys_item = MenuItem::with_id(ID_HOTKEYS, "⌨️  Change Hotkeys →", true, None);

    
    menu.append(&label_item).ok();
    menu.append(&PredefinedMenuItem::separator()).ok();
    menu.append(&usage_item).ok();
    menu.append(&timer_item).ok();
    menu.append(&PredefinedMenuItem::separator()).ok();
    menu.append(&upgrade_item).ok();
    menu.append(&PredefinedMenuItem::separator()).ok();
    menu.append(&hotkeys_item).ok();
    menu.append(&startup_item).ok();
    menu.append(&quit_item).ok();

    log("Creating tray icon...");
    let icon = create_tray_icon();

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Craftr — AI Prompt Power-ups")
        .with_icon(icon)
        .build()
        .unwrap_or_else(|e| {
            let err = format!("Failed to create tray icon: {:?}", e);
            log(&err);
            panic!("{}", err);
        });
    log("Tray icon created successfully");

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    let menu_receiver = MenuEvent::receiver();
    let tray_receiver = TrayIconEvent::receiver();

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

        if let Ok(event) = menu_receiver.recv_timeout(std::time::Duration::from_millis(50)) {
            log(&format!("Menu item clicked: {:?}", event.id.0));
            match event.id.0.as_str() {
                ID_UPGRADE => {
                    let _ = opener::open("https://getcraftr.vercel.app/pricing");
                }
                ID_HOTKEYS => {
                    let hs_clone = hotkeys_state.clone();
                    std::thread::spawn(move || {
                        settings_win32::show_settings_window(hs_clone);
                    });
                }
                ID_STARTUP => {
                    let mut current_cfg = config::load_config();
                    current_cfg.launch_at_startup = !current_cfg.launch_at_startup;
                    if current_cfg.launch_at_startup {
                        startup_item.set_text("✓ Launch at startup");
                    } else {
                        startup_item.set_text("Launch at startup");
                    }
                    let _ = config::save_config(&current_cfg);
                    update_startup_registry(current_cfg.launch_at_startup);
                }
                ID_QUIT => {
                    running_clone.store(false, Ordering::Relaxed);
                }

                _ => {}
            }
        }

        if let Ok(event) = tray_receiver.try_recv() {
            if let TrayIconEvent::Click { button, .. } = &event {
                if *button == tray_icon::MouseButton::Left {
                    log("LEFT CLICK DETECTED");
                    log("Attempting to create popup window...");
                    // Spawn pure Win32 borderless window
                    let hs_clone = hotkeys_state.clone();
                    std::thread::spawn(move || {
                        popup_win32::show_popup(hs_clone);
                        log("Popup window created");
                    });
                } else if *button == tray_icon::MouseButton::Right {
                    log("RIGHT CLICK DETECTED");
                }
            }
        }

        // Process Windows messages so the tray icon remains responsive
        unsafe {
            use winapi::um::winuser::{PeekMessageW, TranslateMessage, DispatchMessageW, MSG, PM_REMOVE};
            let mut msg: MSG = std::mem::zeroed();
            while PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
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
        let _ = opener::open("https://getcraftr.vercel.app/pricing");
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
    let icon_bytes = include_bytes!("../assets/icon-32.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    tray_icon::Icon::from_rgba(rgba, width, height).expect("Failed to create tray icon")
}

use chrono::Timelike;

fn log(msg: &str) {
    use std::io::Write;
    let base_dir = format!("{}\\Craftr", std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string()));
    std::fs::create_dir_all(&base_dir).ok();
    
    let path = format!("{}\\debug.log", base_dir);
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    let time = chrono::Local::now()
        .format("%H:%M:%S").to_string();
    writeln!(file, "[{}] {}", time, msg).unwrap();
}

#[cfg(target_os = "windows")]
fn update_startup_registry(enable: bool) {
    use winreg::RegKey;
    use winreg::enums::*;
    
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(run) = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_SET_VALUE
    ) {
        let exe_path = std::env::current_exe()
            .unwrap()
            .to_string_lossy()
            .to_string();
            
        if enable {
            let _ = run.set_value("Craftr", &exe_path);
        } else {
            let _ = run.delete_value("Craftr");
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn update_startup_registry(_enable: bool) {}
