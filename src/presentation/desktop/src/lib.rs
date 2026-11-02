//! Sallygate desktop app: Tauri commands and events (driving adapters) and the
//! composition root that wires adapters into use cases.

mod command_error;
mod composition;
mod inventory;
mod sessions;
mod trust;

use tauri::Manager;

/// Starts the desktop app.
///
/// # Errors
///
/// Returns the Tauri error when the window or runtime cannot start.
pub fn run() -> tauri::Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(composition::compose(&app.path().app_data_dir()?)?);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            inventory::commands::list_hosts,
            inventory::commands::save_host,
            inventory::commands::delete_host,
            sessions::commands::open_session,
            sessions::commands::write_to_session,
            sessions::commands::resize_session,
            sessions::commands::close_session,
            trust::commands::trust_host_key,
        ])
        .run(tauri::generate_context!())
}
