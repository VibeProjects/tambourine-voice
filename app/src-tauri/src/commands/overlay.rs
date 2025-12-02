use once_cell::sync::Lazy;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

// Store the fixed center point (calculated from initial position)
static OVERLAY_CENTER: Lazy<Mutex<Option<(f64, f64)>>> = Lazy::new(|| Mutex::new(None));

#[tauri::command]
pub async fn resize_overlay(app: AppHandle, width: f64, height: f64) -> Result<(), String> {
    // Enforce minimum dimensions to prevent invisible window
    let min_size = 48.0;
    let width = width.max(min_size);
    let height = height.max(min_size);

    if let Some(window) = app.get_webview_window("overlay") {
        // Get or initialize the center point
        let center = {
            let mut center_guard = OVERLAY_CENTER.lock().unwrap();
            if center_guard.is_none() {
                // Calculate initial center from current position and size
                if let (Ok(pos), Ok(size)) = (window.outer_position(), window.outer_size()) {
                    let scale = window.scale_factor().unwrap_or(1.0);
                    let x = pos.x as f64 / scale;
                    let y = pos.y as f64 / scale;
                    let w = size.width as f64 / scale;
                    let h = size.height as f64 / scale;
                    *center_guard = Some((x + w / 2.0, y + h / 2.0));
                }
            }
            *center_guard
        };

        // Set the new size
        window
            .set_size(tauri::Size::Logical(tauri::LogicalSize { width, height }))
            .map_err(|e| e.to_string())?;

        // Reposition to keep center fixed
        if let Some((cx, cy)) = center {
            let x = cx - width / 2.0;
            let y = cy - height / 2.0;
            window
                .set_position(tauri::Position::Logical(tauri::LogicalPosition { x, y }))
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
