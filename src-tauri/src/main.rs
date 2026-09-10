#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use std::time::Duration;

use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize};

const MIN_SIZE: f64 = 120.0;
const MAX_SIZE: f64 = 800.0;
const MARGIN: i32 = 40;

#[derive(Default)]
struct OverlayState {
    click_through: bool,
    ignoring: bool,
    // 控制条矩形，物理像素，相对窗口内容原点。
    controls: Option<(i32, i32, i32, i32)>,
}

fn cursor_over_controls(app: &AppHandle, controls: Option<(i32, i32, i32, i32)>) -> bool {
    #[cfg(windows)]
    {
        use windows_sys::Win32::Foundation::POINT;
        use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;

        let Some((cx, cy, cw, ch)) = controls else {
            return false;
        };
        let Some(win) = app.get_webview_window("main") else {
            return false;
        };
        let Ok(origin) = win.outer_position() else {
            return false;
        };

        let mut p = POINT { x: 0, y: 0 };
        if unsafe { GetCursorPos(&mut p) } == 0 {
            return false;
        }
        let (lx, ly) = (p.x - origin.x, p.y - origin.y);
        lx >= cx && lx <= cx + cw && ly >= cy && ly <= cy + ch
    }
    #[cfg(not(windows))]
    {
        let _ = (app, controls);
        false
    }
}

fn apply_ignore(app: &AppHandle, st: &mut OverlayState) {
    let Some(win) = app.get_webview_window("main") else {
        return;
    };
    let ignore = st.click_through && !cursor_over_controls(app, st.controls);
    if ignore != st.ignoring {
        if win.set_ignore_cursor_events(ignore).is_ok() {
            st.ignoring = ignore;
        }
    }
}

fn set_click_through(app: &AppHandle, on: bool) {
    let state = app.state::<Arc<Mutex<OverlayState>>>();
    let mut st = state.lock().unwrap();
    st.click_through = on;
    apply_ignore(app, &mut st);
    drop(st);
    let _ = app.emit("click-through-changed", on);
}

fn do_toggle_click_through(app: &AppHandle) {
    let on = !app
        .state::<Arc<Mutex<OverlayState>>>()
        .lock()
        .unwrap()
        .click_through;
    set_click_through(app, on);
}

#[tauri::command]
fn toggle_click_through(app: AppHandle) {
    do_toggle_click_through(&app);
}

#[tauri::command]
fn resize_window(app: AppHandle, delta: f64) {
    if let Some(win) = app.get_webview_window("main") {
        let size = win.outer_size().unwrap_or(PhysicalSize::new(280, 280));
        let next = ((size.width as f64 + delta).clamp(MIN_SIZE, MAX_SIZE)) as u32;
        let _ = win.set_size(PhysicalSize::new(next, next));
    }
}

#[tauri::command]
fn set_controls_rect(
    app: AppHandle,
    state: tauri::State<'_, Arc<Mutex<OverlayState>>>,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    let mut st = state.lock().unwrap();
    st.controls = Some((x, y, width, height));
    apply_ignore(&app, &mut st);
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(Arc::new(Mutex::new(OverlayState::default())))
        .invoke_handler(tauri::generate_handler![
            toggle_click_through,
            resize_window,
            set_controls_rect
        ])
        .setup(|app| {
            use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

            let win = app.get_webview_window("main").unwrap();

            // 初始位置：主显示器工作区右下角。
            if let Ok(Some(mon)) = win.primary_monitor() {
                let size = win.outer_size().unwrap();
                let wa = mon.work_area();
                let _ = win.set_position(PhysicalPosition::new(
                    wa.position.x + wa.size.width as i32 - size.width as i32 - MARGIN,
                    wa.position.y + wa.size.height as i32 - size.height as i32 - MARGIN,
                ));
            }

            app.global_shortcut()
                .on_shortcut("ctrl+shift+c", |app, _s, event| {
                    if event.state == ShortcutState::Pressed {
                        do_toggle_click_through(app);
                    }
                })?;

            // 点击穿透开启期间轮询光标：一旦移到底部控制条上就恢复鼠标事件，
            // 保证切换按钮（及其相邻按钮）始终可点。Tauri 没有 Electron 的
            // 鼠标移动转发等价物，因此改为轮询系统光标位置。
            let handle = app.handle().clone();
            std::thread::spawn(move || loop {
                std::thread::sleep(Duration::from_millis(33));
                let state = handle.state::<Arc<Mutex<OverlayState>>>();
                let mut st = state.lock().unwrap();
                if st.click_through {
                    apply_ignore(&handle, &mut st);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
