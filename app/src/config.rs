// config.rs — Minimal config for Craftr
//
// Stores only usage tracking and license key.
// Config path: %APPDATA%\Craftr\config.json

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use chrono::Local;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub requests_today: u32,
    pub last_request_date: String,
    pub license_key: Option<String>,
    #[serde(default)]
    pub is_pro: bool,
    pub last_validation_date: Option<String>,
    #[serde(default = "default_enhance")]
    pub enhance_hotkey: String,
    #[serde(default = "default_compress")]
    pub compress_hotkey: String,
    #[serde(default = "default_true")]
    pub launch_at_startup: bool,
}

fn default_true() -> bool { true }
fn default_enhance() -> String { "ctrl+e".to_string() }
fn default_compress() -> String { "ctrl+shift+e".to_string() }


impl Default for Config {
    fn default() -> Self {
        Self {
            requests_today: 0,
            last_request_date: chrono::Local::now().date_naive().to_string(),
            license_key: None,
            is_pro: false,
            last_validation_date: None,
            enhance_hotkey: default_enhance(),
            compress_hotkey: default_compress(),
            launch_at_startup: true,
        }

    }
}

pub fn config_dir() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .to_string_lossy()
            .to_string()
    });
    PathBuf::from(appdata).join("Craftr")
}

pub fn config_file_path() -> PathBuf {
    config_dir().join("config.json")
}

pub fn load_config() -> Config {
    let path = config_file_path();
    if !path.exists() {
        return Config::default();
    }
    let content = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_else(|_| Config::default())
}

pub fn save_config(config: &Config) -> Result<(), String> {
    let path = config_file_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create config dir: {}", e))?;
    }
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(&path, json).map_err(|e| format!("Failed to write config: {}", e))?;
    Ok(())
}

pub fn enforce_daily_reset() {
    let mut config = load_config();
    let today = chrono::Local::now().date_naive().to_string();

    if config.last_request_date != today {
        config.last_request_date = today;
        config.requests_today = 0;
        let _ = save_config(&config);
    }
}

pub fn check_daily_limit() -> Result<bool, String> {
    enforce_daily_reset();
    let config = load_config();

    if config.is_pro {
        return Ok(true);
    }

    if config.requests_today >= 10 {
        return Ok(false);
    }

    Ok(true)
}

pub fn increment_request_count() {
    let mut config = load_config();
    config.requests_today += 1;
    let _ = save_config(&config);
}

pub fn remaining_requests() -> u32 {
    let config = load_config();
    if config.is_pro {
        return u32::MAX;
    }
    10u32.saturating_sub(config.requests_today)
}
