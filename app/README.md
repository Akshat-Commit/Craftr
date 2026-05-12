# Craftr

A blazing-fast system tray application that enhances or compresses your AI prompts with a single hotkey.

Powered by **Groq** (`llama-3.3-70b-versatile`) for near-instant results.

## How It Works

1. Copy any text to your clipboard (or select it)
2. Press the hotkey
3. Craftr sends it to the AI, rewrites it, and pastes the result back — automatically

## Hotkeys

| Action   | Windows          | macOS            |
|----------|------------------|------------------|
| Enhance  | `Ctrl+E`         | `Cmd+E`          |
| Compress | `Ctrl+Shift+E`   | `Cmd+Shift+E`    |

## Tray Menu

- **Craftr** — app name
- **Enhance / Compress** — trigger manually
- **Set API Key** — enter your Groq API key
- **Enter License Key** — activate Pro tier
- **Upgrade to Pro** — opens craftr.app/upgrade
- **Quit**

## Pricing

| Tier | Limit | Price |
|------|-------|-------|
| Free | 10 requests/day | Free |
| Pro  | Unlimited | craftr.app/upgrade |

## Build

```bash
# Debug
cargo build

# Release (optimized, ~1.7 MB)
cargo build --release
```

The binary will be at `target/release/craftr.exe` (Windows) or `target/release/craftr` (macOS).

## Configuration

Config is stored at:
- **Windows**: `%APPDATA%\Craftr\config.json`
- **macOS**: `~/.config/Craftr/config.json`

```json
{
  "api_key": "gsk_...",
  "license_key": null,
  "is_pro": false,
  "requests_today": 0,
  "last_request_date": "2026-05-12",
  "last_validation_date": null
}
```

## Project Structure

```
src/
  main.rs       — tray setup, event loop, orchestration
  hotkeys.rs    — global hotkey registration (platform-specific)
  clipboard.rs  — read/write/paste logic
  api.rs        — Groq API calls
  config.rs     — load/save config & daily limits
```

## License

MIT
