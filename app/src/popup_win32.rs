// popup_win32.rs — Native Win32 borderless popup for Craftr
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM, HINSTANCE};
#[cfg(target_os = "windows")]
use winapi::shared::windef::{HBRUSH, HDC, HFONT, HWND, RECT};
#[cfg(target_os = "windows")]
use winapi::um::libloaderapi::GetModuleHandleW;
#[cfg(target_os = "windows")]
use winapi::um::wingdi::*;
#[cfg(target_os = "windows")]
use winapi::um::winuser::*;

use std::ptr::null_mut;
use chrono::Local;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

#[cfg(target_os = "windows")]
pub fn show_popup() {
    unsafe {
        let class_name = to_wstring("CraftrPopupClass");
        let h_inst = GetModuleHandleW(null_mut());

        let mut wc: WNDCLASSW = std::mem::zeroed();
        wc.lpfnWndProc = Some(wnd_proc);
        wc.hInstance = h_inst;
        wc.lpszClassName = class_name.as_ptr();
        wc.hbrBackground = CreateSolidBrush(RGB(8, 8, 8)) as HBRUSH;
        wc.hCursor = LoadCursorW(null_mut(), IDC_ARROW);

        RegisterClassW(&wc);

        let mut tray_rect: RECT = std::mem::zeroed();
        SystemParametersInfoW(SPI_GETWORKAREA, 0, &mut tray_rect as *mut _ as *mut _, 0);
        let width = 320;
        let height = 280;
        let x = tray_rect.right - width - 20;
        let y = tray_rect.bottom - height - 20;

        let hwnd = CreateWindowExW(
            WS_EX_TOOLWINDOW | WS_EX_TOPMOST,
            class_name.as_ptr(),
            to_wstring("Craftr Dashboard").as_ptr(),
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
pub fn show_popup() {}

#[cfg(target_os = "windows")]
unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_KILLFOCUS => {
            PostMessageW(hwnd, WM_CLOSE, 0, 0);
            0
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);

            // Draw Background
            let mut rect: RECT = std::mem::zeroed();
            GetClientRect(hwnd, &mut rect);
            let bg_brush = CreateSolidBrush(RGB(8, 8, 8));
            FillRect(hdc, &rect, bg_brush);
            DeleteObject(bg_brush as _);

            // Draw header "⚡ Craftr"
            SetBkMode(hdc, TRANSPARENT as i32);
            SetTextColor(hdc, RGB(170, 255, 0));
            let header_font = CreateFontW(
                24, 0, 0, 0, FW_BOLD, 0, 0, 0, ANSI_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
                CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_DONTCARE, to_wstring("Segoe UI").as_ptr(),
            );
            SelectObject(hdc, header_font as _);
            let mut header_rect = RECT { left: 20, top: 20, right: 300, bottom: 50 };
            DrawTextW(hdc, to_wstring("⚡ Craftr").as_ptr(), -1, &mut header_rect, DT_LEFT | DT_SINGLELINE);
            DeleteObject(header_font as _);

            let cfg = crate::config::load_config();
            
            // Draw stats
            SetTextColor(hdc, RGB(200, 200, 200));
            let normal_font = CreateFontW(
                16, 0, 0, 0, FW_NORMAL, 0, 0, 0, ANSI_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
                CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_DONTCARE, to_wstring("Segoe UI").as_ptr(),
            );
            SelectObject(hdc, normal_font as _);

            let mut stats_rect = RECT { left: 20, top: 70, right: 300, bottom: 90 };
            let mut timer_rect = RECT { left: 20, top: 110, right: 300, bottom: 130 };

            if cfg.is_pro {
                SetTextColor(hdc, RGB(170, 255, 0));
                DrawTextW(hdc, to_wstring("✅ Pro — Unlimited requests").as_ptr(), -1, &mut stats_rect, DT_LEFT | DT_SINGLELINE);
                SetTextColor(hdc, RGB(200, 200, 200));
            } else {
                let used = cfg.requests_today;
                DrawTextW(hdc, to_wstring(&format!("📊 {}/10 requests used today", used)).as_ptr(), -1, &mut stats_rect, DT_LEFT | DT_SINGLELINE);

                // Draw simple progress bar background
                let mut bar_bg = RECT { left: 20, top: 95, right: 300, bottom: 100 };
                let bg_brush = CreateSolidBrush(RGB(40, 40, 40));
                FillRect(hdc, &mut bar_bg, bg_brush);
                DeleteObject(bg_brush as _);

                // Draw simple progress bar foreground
                if used > 0 {
                    let fill_width = ((300 - 20) as f32 * (used as f32 / 10.0).min(1.0)) as i32;
                    let color = if used >= 10 { RGB(255, 51, 102) } else { RGB(170, 255, 0) };
                    let mut bar_fg = RECT { left: 20, top: 95, right: 20 + fill_width, bottom: 100 };
                    let fg_brush = CreateSolidBrush(color);
                    FillRect(hdc, &mut bar_fg, fg_brush);
                    DeleteObject(fg_brush as _);
                }

                let now = Local::now();
                use chrono::Timelike;
                let hours_left = 23 - now.hour();
                let mins_left = 59 - now.minute();
                DrawTextW(hdc, to_wstring(&format!("⏰ Resets in {}h {}m", hours_left, mins_left)).as_ptr(), -1, &mut timer_rect, DT_LEFT | DT_SINGLELINE);
            }
            DeleteObject(normal_font as _);

            // Draw Upgrade Button
            if !cfg.is_pro {
                let mut btn_rect = RECT { left: 20, top: 220, right: 300, bottom: 260 };
                let btn_brush = CreateSolidBrush(RGB(170, 255, 0));
                FillRect(hdc, &mut btn_rect, btn_brush);
                DeleteObject(btn_brush as _);

                SetTextColor(hdc, RGB(0, 0, 0));
                let bold_font = CreateFontW(
                    16, 0, 0, 0, FW_BOLD, 0, 0, 0, ANSI_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
                    CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_DONTCARE, to_wstring("Segoe UI").as_ptr(),
                );
                SelectObject(hdc, bold_font as _);
                DrawTextW(hdc, to_wstring("✨ Upgrade to Pro →").as_ptr(), -1, &mut btn_rect, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
                DeleteObject(bold_font as _);
            }

            EndPaint(hwnd, &ps);
            0
        }
        WM_LBUTTONUP => {
            let x = (lparam & 0xFFFF) as i32;
            let y = ((lparam >> 16) & 0xFFFF) as i32;
            let cfg = crate::config::load_config();
            
            // Check if clicked inside upgrade button
            if !cfg.is_pro && x >= 20 && x <= 300 && y >= 220 && y <= 260 {
                let _ = opener::open("https://craftr.app/upgrade");
                PostMessageW(hwnd, WM_CLOSE, 0, 0);
            }
            0
        }
        WM_DESTROY => {
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
