//! Sallygate desktop app: Tauri commands and events (driving adapters) and the
//! composition root that wires adapters into use cases.

mod command_error;
mod composition;
mod credentials;
mod inventory;
mod sessions;
mod snippets;
mod transfer;
mod trust;

use tauri::Manager;

/// Starts the desktop app.
///
/// # Errors
///
/// Returns the Tauri error when the window or runtime cannot start.
pub fn run() -> tauri::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app.manage(composition::compose(&app.path().app_data_dir()?)?);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            inventory::commands::list_hosts,
            inventory::commands::save_host,
            inventory::commands::delete_host,
            inventory::commands::import_ssh_config,
            sessions::commands::open_session,
            sessions::commands::write_to_session,
            sessions::commands::resize_session,
            sessions::commands::close_session,
            trust::commands::trust_host_key,
            trust::commands::list_known_hosts,
            trust::commands::forget_known_host,
            credentials::commands::list_ssh_keys,
            credentials::commands::generate_ssh_key,
            credentials::commands::import_ssh_key,
            snippets::commands::list_snippets,
            snippets::commands::save_snippet,
            snippets::commands::delete_snippet,
            transfer::commands::list_remote_folder,
            transfer::commands::make_remote_folder,
            transfer::commands::rename_remote_entry,
            transfer::commands::delete_remote_entry,
            transfer::commands::download_remote_file,
            transfer::commands::upload_local_file,
        ])
        .run(tauri::generate_context!())
}
