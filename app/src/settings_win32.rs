// settings_win32.rs — Native Win32 window for changing hotkeys
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
#[cfg(target_os = "windows")]
use winapi::shared::windef::{HBRUSH, HDC, HFONT, HWND, RECT};
#[cfg(target_os = "windows")]
use winapi::um::libloaderapi::GetModuleHandleW;
#[cfg(target_os = "windows")]
use winapi::um::wingdi::*;
#[cfg(target_os = "windows")]
use winapi::um::winuser::*;

use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

struct SettingsState {
    pub global_hotkeys: Arc<Mutex<(String, String)>>,
    pub enhance: String,
    pub compress: String,
    pub active_input: u8,
    pub saved: bool,
    pub error_msg: Option<String>,
}

#[cfg(target_os = "windows")]
pub fn show_settings_window(hotkeys_state: Arc<Mutex<(String, String)>>) {
    unsafe {
        let class_name = to_wstring("CraftrSettingsClass");
        let h_inst = GetModuleHandleW(null_mut());

        let mut wc: WNDCLASSW = std::mem::zeroed();
        wc.lpfnWndProc = Some(settings_wnd_proc);
        wc.hInstance = h_inst;
        wc.lpszClassName = class_name.as_ptr();
        wc.hbrBackground = CreateSolidBrush(RGB(8, 8, 8)) as HBRUSH;
        wc.hCursor = LoadCursorW(null_mut(), IDC_ARROW);

        RegisterClassW(&wc);

        let width = 380;
        let height = 280;
        let screen_x = GetSystemMetrics(SM_CXSCREEN);
        let screen_y = GetSystemMetrics(SM_CYSCREEN);
        let x = (screen_x - width) / 2;
        let y = (screen_y - height) / 2;

        let hwnd = CreateWindowExW(
            WS_EX_TOOLWINDOW | WS_EX_TOPMOST,
            class_name.as_ptr(),
            to_wstring("Change Hotkeys").as_ptr(),
            WS_POPUP,
            x,
            y,
            width,
            height,
            null_mut(),
            null_mut(),
            h_inst,
            null_mut(),
        );

        if hwnd.is_null() {
            return;
        }

        let current_cfg = crate::config::load_config();
        
        let state = Box::new(SettingsState {
            global_hotkeys: hotkeys_state,
            enhance: current_cfg.enhance_hotkey.clone(),
            compress: current_cfg.compress_hotkey.clone(),
            active_input: 0,
            saved: false,
            error_msg: None,
        });

        SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(state) as isize);

        ShowWindow(hwnd, SW_SHOW);
        SetForegroundWindow(hwnd);
        SetFocus(hwnd);

        let mut msg = std::mem::zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn show_settings_window(_hotkeys_state: Arc<Mutex<(String, String)>>) {}

#[cfg(target_os = "windows")]
unsafe extern "system" fn settings_wnd_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_KILLFOCUS => {
            // Keep window alive if they switch away, but maybe auto-close on lost focus?
            // PostMessageW(hwnd, WM_CLOSE, 0, 0);
            0
        }
        WM_TIMER => {
            if wparam == 1 {
                KillTimer(hwnd, 1);
                PostMessageW(hwnd, WM_CLOSE, 0, 0);
            }
            0
        }
        WM_KEYDOWN | WM_SYSKEYDOWN => {
            let state_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut SettingsState;
            if state_ptr.is_null() { return 0; }
            let state = &mut *state_ptr;

            if state.active_input != 0 {
                let vk = wparam as i32;
                if vk == VK_CONTROL || vk == VK_LCONTROL || vk == VK_RCONTROL ||
                   vk == VK_SHIFT || vk == VK_LSHIFT || vk == VK_RSHIFT ||
                   vk == VK_MENU || vk == VK_LMENU || vk == VK_RMENU ||
                   vk == VK_LWIN || vk == VK_RWIN {
                    return 0;
                }

                let mut parts = Vec::new();
                if (GetAsyncKeyState(VK_CONTROL) as u16 & 0x8000) != 0 { parts.push("ctrl"); }
                if (GetAsyncKeyState(VK_MENU) as u16 & 0x8000) != 0 { parts.push("alt"); }
                if (GetAsyncKeyState(VK_SHIFT) as u16 & 0x8000) != 0 { parts.push("shift"); }
                if (GetAsyncKeyState(VK_LWIN) as u16 & 0x8000) != 0 || (GetAsyncKeyState(VK_RWIN) as u16 & 0x8000) != 0 { parts.push("win"); }

                if parts.is_empty() {
                    state.error_msg = Some("Must include Ctrl/Alt/Cmd".to_string());
                } else if let Some(key_str) = vk_to_string(vk) {
                    parts.push(&key_str);
                    let combo = parts.join("+");
                    if state.active_input == 1 {
                        state.enhance = combo;
                    } else {
                        state.compress = combo;
                    }
                    state.active_input = 0;
                    state.error_msg = None;
                } else {
                    state.error_msg = Some("Invalid key".to_string());
                }
                InvalidateRect(hwnd, null_mut(), 1);
            } else if wparam == VK_ESCAPE as usize {
                PostMessageW(hwnd, WM_CLOSE, 0, 0);
            }
            0
        }
        WM_LBUTTONDOWN => {
            let x = (lparam & 0xFFFF) as i32;
            let y = ((lparam >> 16) & 0xFFFF) as i32;
            let state_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut SettingsState;
            if state_ptr.is_null() { return 0; }
            let state = &mut *state_ptr;

            state.active_input = 0;

            // Enhance box: 20..360, 70..110
            if x >= 20 && x <= 360 && y >= 70 && y <= 110 {
                state.active_input = 1;
                state.error_msg = None;
            }
            // Compress box: 20..360, 160..200
            else if x >= 20 && x <= 360 && y >= 160 && y <= 200 {
                state.active_input = 2;
                state.error_msg = None;
            }
            // Quit Button: 190..270, 230..260
            else if x >= 190 && x <= 270 && y >= 230 && y <= 260 {
                PostMessageW(hwnd, WM_CLOSE, 0, 0);
            }
            // Save Button: 280..360, 230..260
            else if x >= 280 && x <= 360 && y >= 230 && y <= 260 {
                if state.enhance == state.compress {
                    state.error_msg = Some("Hotkeys must be different".to_string());
                } else {
                    let mut cfg = crate::config::load_config();
                    cfg.enhance_hotkey = state.enhance.clone();
                    cfg.compress_hotkey = state.compress.clone();
                    let _ = crate::config::save_config(&cfg);
                    
                    if let Ok(mut g) = state.global_hotkeys.lock() {
                        *g = (state.enhance.clone(), state.compress.clone());
                    }
                    
                    state.saved = true;
                    state.error_msg = None;
                    SetTimer(hwnd, 1, 1500, None);
                }
            }
            InvalidateRect(hwnd, null_mut(), 1);
            0
        }
        WM_PAINT => {
            let state_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut SettingsState;
            if state_ptr.is_null() { return DefWindowProcW(hwnd, msg, wparam, lparam); }
            let state = &mut *state_ptr;

            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);

            // Background
            let mut rect: RECT = std::mem::zeroed();
            GetClientRect(hwnd, &mut rect);
            let bg_brush = CreateSolidBrush(RGB(8, 8, 8));
            FillRect(hdc, &rect, bg_brush);
            DeleteObject(bg_brush as _);

            SetBkMode(hdc, TRANSPARENT as i32);

            let font_header = CreateFontW(16, 0, 0, 0, FW_BOLD, 0, 0, 0, ANSI_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_DONTCARE, to_wstring("Segoe UI").as_ptr());
            let font_small = CreateFontW(12, 0, 0, 0, FW_NORMAL, 0, 0, 0, ANSI_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_DONTCARE, to_wstring("Segoe UI").as_ptr());
            let font_label = CreateFontW(13, 0, 0, 0, FW_NORMAL, 0, 0, 0, ANSI_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_DONTCARE, to_wstring("Segoe UI").as_ptr());
            let font_input = CreateFontW(14, 0, 0, 0, FW_NORMAL, 0, 0, 0, ANSI_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_DONTCARE, to_wstring("Segoe UI").as_ptr());
            
            // Header
            SetTextColor(hdc, RGB(255, 255, 255));
            SelectObject(hdc, font_header as _);
            let mut r = RECT { left: 20, top: 15, right: 360, bottom: 35 };
            DrawTextW(hdc, to_wstring("⌨️ Craftr — Change Hotkeys").as_ptr(), -1, &mut r, DT_LEFT | DT_SINGLELINE);
            
            SetTextColor(hdc, RGB(150, 150, 150));
            SelectObject(hdc, font_small as _);
            r = RECT { left: 20, top: 35, right: 360, bottom: 50 };
            DrawTextW(hdc, to_wstring("Customize your global shortcuts").as_ptr(), -1, &mut r, DT_LEFT | DT_SINGLELINE);

            // Divider
            let pen = CreatePen(PS_SOLID as i32, 1, RGB(50, 50, 50));
            SelectObject(hdc, pen as _);
            MoveToEx(hdc, 20, 55, null_mut());
            LineTo(hdc, 360, 55);
            DeleteObject(pen as _);

            // Enhance Section
            SetTextColor(hdc, RGB(255, 255, 255));
            SelectObject(hdc, font_label as _);
            r = RECT { left: 20, top: 65, right: 360, bottom: 80 };
            DrawTextW(hdc, to_wstring("Enhance Prompt").as_ptr(), -1, &mut r, DT_LEFT | DT_SINGLELINE);

            let mut box_rect = RECT { left: 20, top: 85, right: 360, bottom: 120 };
            let box_brush = CreateSolidBrush(RGB(26, 26, 26)); // #1a1a1a
            FillRect(hdc, &mut box_rect, box_brush);
            DeleteObject(box_brush as _);

            let border_color = if state.active_input == 1 { RGB(170, 255, 0) } else if state.active_input == 1 && state.error_msg.is_some() { RGB(255, 50, 50) } else { RGB(51, 51, 51) };
            let pen = CreatePen(PS_SOLID as i32, 1, border_color);
            SelectObject(hdc, pen as _);
            SelectObject(hdc, GetStockObject(NULL_BRUSH as i32));
            Rectangle(hdc, box_rect.left, box_rect.top, box_rect.right, box_rect.bottom);
            DeleteObject(pen as _);

            SelectObject(hdc, font_input as _);
            box_rect.left += 10;
            if state.active_input == 1 {
                SetTextColor(hdc, RGB(170, 255, 0));
                DrawTextW(hdc, to_wstring("Press your new hotkey...").as_ptr(), -1, &mut box_rect, DT_LEFT | DT_VCENTER | DT_SINGLELINE);
            } else {
                SetTextColor(hdc, RGB(200, 200, 200));
                DrawTextW(hdc, to_wstring(&format_hotkey(&state.enhance)).as_ptr(), -1, &mut box_rect, DT_LEFT | DT_VCENTER | DT_SINGLELINE);
            }

            SetTextColor(hdc, RGB(150, 150, 150));
            SelectObject(hdc, font_small as _);
            r = RECT { left: 20, top: 125, right: 360, bottom: 140 };
            let cfg = crate::config::load_config();
            DrawTextW(hdc, to_wstring(&format!("Currently active: {}", format_hotkey(&cfg.enhance_hotkey))).as_ptr(), -1, &mut r, DT_LEFT | DT_SINGLELINE);

            // Compress Section
            SetTextColor(hdc, RGB(255, 255, 255));
            SelectObject(hdc, font_label as _);
            r = RECT { left: 20, top: 155, right: 360, bottom: 170 };
            DrawTextW(hdc, to_wstring("Compress Prompt").as_ptr(), -1, &mut r, DT_LEFT | DT_SINGLELINE);

            let mut box_rect2 = RECT { left: 20, top: 175, right: 360, bottom: 210 };
            let box_brush2 = CreateSolidBrush(RGB(26, 26, 26)); // #1a1a1a
            FillRect(hdc, &mut box_rect2, box_brush2);
            DeleteObject(box_brush2 as _);

            let border_color2 = if state.active_input == 2 { RGB(170, 255, 0) } else if state.active_input == 2 && state.error_msg.is_some() { RGB(255, 50, 50) } else { RGB(51, 51, 51) };
            let pen2 = CreatePen(PS_SOLID as i32, 1, border_color2);
            SelectObject(hdc, pen2 as _);
            SelectObject(hdc, GetStockObject(NULL_BRUSH as i32));
            Rectangle(hdc, box_rect2.left, box_rect2.top, box_rect2.right, box_rect2.bottom);
            DeleteObject(pen2 as _);

            SelectObject(hdc, font_input as _);
            box_rect2.left += 10;
            if state.active_input == 2 {
                SetTextColor(hdc, RGB(170, 255, 0));
                DrawTextW(hdc, to_wstring("Press your new hotkey...").as_ptr(), -1, &mut box_rect2, DT_LEFT | DT_VCENTER | DT_SINGLELINE);
            } else {
                SetTextColor(hdc, RGB(200, 200, 200));
                DrawTextW(hdc, to_wstring(&format_hotkey(&state.compress)).as_ptr(), -1, &mut box_rect2, DT_LEFT | DT_VCENTER | DT_SINGLELINE);
            }

            SetTextColor(hdc, RGB(150, 150, 150));
            SelectObject(hdc, font_small as _);
            r = RECT { left: 20, top: 215, right: 360, bottom: 230 };
            DrawTextW(hdc, to_wstring(&format!("Currently active: {}", format_hotkey(&cfg.compress_hotkey))).as_ptr(), -1, &mut r, DT_LEFT | DT_SINGLELINE);

            // Quit Button
            let mut quit_btn_rect = RECT { left: 190, top: 230, right: 270, bottom: 260 };
            let quit_btn_brush = CreateSolidBrush(RGB(30, 30, 30));
            FillRect(hdc, &mut quit_btn_rect, quit_btn_brush);
            DeleteObject(quit_btn_brush as _);
            
            SetTextColor(hdc, RGB(200, 200, 200));
            SelectObject(hdc, font_label as _);
            DrawTextW(hdc, to_wstring("Quit").as_ptr(), -1, &mut quit_btn_rect, DT_CENTER | DT_VCENTER | DT_SINGLELINE);

            // Save Button
            let mut btn_rect = RECT { left: 280, top: 230, right: 360, bottom: 260 };
            let btn_brush = CreateSolidBrush(RGB(170, 255, 0));
            FillRect(hdc, &mut btn_rect, btn_brush);
            DeleteObject(btn_brush as _);

            SetTextColor(hdc, RGB(0, 0, 0));
            SelectObject(hdc, font_header as _);
            DrawTextW(hdc, to_wstring("Save ✓").as_ptr(), -1, &mut btn_rect, DT_CENTER | DT_VCENTER | DT_SINGLELINE);

            // Message area
            let mut msg_rect = RECT { left: 20, top: 235, right: 270, bottom: 260 };
            if state.saved {
                SetTextColor(hdc, RGB(170, 255, 0));
                DrawTextW(hdc, to_wstring("✓ Hotkeys saved!").as_ptr(), -1, &mut msg_rect, DT_LEFT | DT_VCENTER | DT_SINGLELINE);
            } else if let Some(err) = &state.error_msg {
                SetTextColor(hdc, RGB(255, 50, 50));
                DrawTextW(hdc, to_wstring(err).as_ptr(), -1, &mut msg_rect, DT_LEFT | DT_VCENTER | DT_SINGLELINE);
            }

            DeleteObject(font_header as _);
            DeleteObject(font_small as _);
            DeleteObject(font_label as _);
            DeleteObject(font_input as _);
            EndPaint(hwnd, &ps);
            0
        }
        WM_DESTROY => {
            let state_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut SettingsState;
            if !state_ptr.is_null() {
                unsafe { let _ = Box::from_raw(state_ptr); }
            }
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

#[cfg(target_os = "windows")]
fn to_wstring(str: &str) -> Vec<u16> {
    OsStr::new(str).encode_wide().chain(Some(0)).collect()
}

#[cfg(target_os = "windows")]
fn RGB(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) | ((g as u32) << 8) | ((b as u32) << 16)
}

fn format_hotkey(s: &str) -> String {
    s.split('+').map(|p| {
        let mut c = p.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }).collect::<Vec<_>>().join(" + ")
}

fn vk_to_string(vk: i32) -> Option<String> {
    if vk >= 0x41 && vk <= 0x5A { // A-Z
        return Some(((vk as u8) as char).to_string().to_lowercase());
    }
    if vk >= 0x30 && vk <= 0x39 { // 0-9
        return Some(((vk as u8) as char).to_string());
    }
    if vk >= 0x70 && vk <= 0x7B { // F1-F12
        return Some(format!("f{}", vk - 0x70 + 1));
    }
    None
}
