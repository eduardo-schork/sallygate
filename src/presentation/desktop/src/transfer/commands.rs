#![allow(
    clippy::needless_pass_by_value,
    reason = "Tauri commands receive their arguments by value"
)]

use std::path::PathBuf;
use std::sync::Arc;

use tauri::ipc::Channel;
use tauri::{AppHandle, Manager, State};

use super::contract::{
    ChannelTransferProgress, EntryKindDto, RemoteFolderDto, TransferProgressDto,
    from_transfer_error,
};
use crate::command_error::{CommandError, CommandErrorKind};
use crate::composition::AppServices;
use crate::sessions::parse_session_id;

/// Lists a folder on the session's server; the login folder when `path` is missing.
#[tauri::command]
pub async fn list_remote_folder(
    services: State<'_, AppServices>,
    session_id: String,
    path: Option<String>,
) -> Result<RemoteFolderDto, CommandError> {
    let folder = services
        .browse_remote_files
        .list(parse_session_id(&session_id)?, path.as_deref())
        .await
        .map_err(|error| from_transfer_error(&error))?;
    Ok(RemoteFolderDto::from(&folder))
}

/// Makes a folder named `name` inside `parent`.
#[tauri::command]
pub async fn make_remote_folder(
    services: State<'_, AppServices>,
    session_id: String,
    parent: String,
    name: String,
) -> Result<(), CommandError> {
    services
        .browse_remote_files
        .make_folder(parse_session_id(&session_id)?, &parent, &name)
        .await
        .map_err(|error| from_transfer_error(&error))
}

/// Gives a remote entry a new name in the same folder.
#[tauri::command]
pub async fn rename_remote_entry(
    services: State<'_, AppServices>,
    session_id: String,
    path: String,
    new_name: String,
) -> Result<(), CommandError> {
    services
        .browse_remote_files
        .rename(parse_session_id(&session_id)?, &path, &new_name)
        .await
        .map_err(|error| from_transfer_error(&error))
}

/// Deletes a remote file, link or empty folder.
#[tauri::command]
pub async fn delete_remote_entry(
    services: State<'_, AppServices>,
    session_id: String,
    path: String,
    kind: EntryKindDto,
) -> Result<(), CommandError> {
    services
        .browse_remote_files
        .delete(parse_session_id(&session_id)?, &path, kind.into())
        .await
        .map_err(|error| from_transfer_error(&error))
}

/// Copies a remote file into the user's Downloads folder and returns its local path.
#[tauri::command]
pub async fn download_remote_file(
    app: AppHandle,
    services: State<'_, AppServices>,
    session_id: String,
    remote_path: String,
    on_progress: Channel<TransferProgressDto>,
) -> Result<String, CommandError> {
    let downloads = app
        .path()
        .download_dir()
        .map_err(|error| CommandError::new(CommandErrorKind::FileOperationFailed, &error))?;
    let local_path = services
        .transfer_files
        .download(
            parse_session_id(&session_id)?,
            &remote_path,
            &downloads,
            Arc::new(ChannelTransferProgress(on_progress)),
        )
        .await
        .map_err(|error| from_transfer_error(&error))?;
    Ok(local_path.to_string_lossy().into_owned())
}

/// Copies a local file into a remote folder and returns its remote path.
#[tauri::command]
pub async fn upload_local_file(
    services: State<'_, AppServices>,
    session_id: String,
    local_path: String,
    remote_folder: String,
    on_progress: Channel<TransferProgressDto>,
) -> Result<String, CommandError> {
    services
        .transfer_files
        .upload(
            parse_session_id(&session_id)?,
            &PathBuf::from(local_path),
            &remote_folder,
            Arc::new(ChannelTransferProgress(on_progress)),
        )
        .await
        .map_err(|error| from_transfer_error(&error))
}
