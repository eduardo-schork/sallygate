//! Sallygate desktop app: Tauri commands and events (driving adapters) and the
//! composition root that wires adapters into use cases.

/// Starts the desktop app.
///
/// # Errors
///
/// Returns the Tauri error when the window or runtime cannot start.
pub fn run() -> tauri::Result<()> {
    tauri::Builder::default().run(tauri::generate_context!())
}
