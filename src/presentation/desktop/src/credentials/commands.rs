#![allow(
    clippy::needless_pass_by_value,
    reason = "Tauri commands receive their arguments by value"
)]

use std::path::PathBuf;

use sallygate_application::credentials::KeyStoreError;
use sallygate_domain::credentials::SshKeyName;
use tauri::State;

use super::contract::{GenerateSshKeyRequest, SshKeyDto};
use crate::command_error::{CommandError, CommandErrorKind};
use crate::composition::AppServices;

fn key_store_error(error: &KeyStoreError) -> CommandError {
    let kind = match error {
        KeyStoreError::AlreadyExists(_) | KeyStoreError::NotAKey(_) => {
            CommandErrorKind::InvalidInput
        }
        KeyStoreError::Read(_) | KeyStoreError::Write(_) => CommandErrorKind::KeyStoreUnavailable,
    };
    CommandError::new(kind, error)
}

/// The key pairs in `~/.ssh`, by name.
#[tauri::command]
pub fn list_ssh_keys(services: State<'_, AppServices>) -> Result<Vec<SshKeyDto>, CommandError> {
    let keys = services
        .ssh_keys
        .list()
        .map_err(|error| key_store_error(&error))?;
    Ok(keys.iter().map(SshKeyDto::from).collect())
}

/// Makes a new Ed25519 pair in `~/.ssh`.
#[tauri::command]
pub fn generate_ssh_key(
    services: State<'_, AppServices>,
    mut request: GenerateSshKeyRequest,
) -> Result<SshKeyDto, CommandError> {
    let passphrase = request.take_passphrase();
    let name =
        SshKeyName::parse(&request.name).map_err(|error| CommandError::invalid_input(&error))?;
    let key = services
        .ssh_keys
        .generate(&name, &request.comment, passphrase.as_ref())
        .map_err(|error| key_store_error(&error))?;
    Ok(SshKeyDto::from(&key))
}

/// Copies a private key file the user picked into `~/.ssh`.
#[tauri::command]
pub fn import_ssh_key(
    services: State<'_, AppServices>,
    path: String,
) -> Result<SshKeyDto, CommandError> {
    let key = services
        .ssh_keys
        .import(&PathBuf::from(path))
        .map_err(|error| key_store_error(&error))?;
    Ok(SshKeyDto::from(&key))
}
