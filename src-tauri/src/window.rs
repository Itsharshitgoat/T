use tauri::{AppHandle, Manager, LogicalPosition};

#[tauri::command]
pub fn show_window(app: AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}

#[tauri::command]
pub fn hide_window(app: AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        window.hide().unwrap();
    }
}

#[tauri::command]
pub fn set_window_position(x: f64, y: f64, app: AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_position(LogicalPosition::new(x, y));
    }
}
