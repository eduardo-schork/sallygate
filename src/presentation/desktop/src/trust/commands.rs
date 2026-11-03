#![allow(
    clippy::needless_pass_by_value,
    reason = "Tauri commands receive their arguments by value"
)]

use tauri::State;

use super::{KnownHostDto, TrustHostKeyRequest};
use crate::command_error::{CommandError, CommandErrorKind};
use crate::composition::AppServices;

/// Records the server key the user chose to trust.
#[tauri::command]
pub fn trust_host_key(
    services: State<'_, AppServices>,
    request: TrustHostKeyRequest,
) -> Result<(), CommandError> {
    let endpoint = request.endpoint.into_domain()?;

    services
        .trust_host_key
        .execute(&endpoint, &request.host_key.into())
        .map_err(|error| CommandError::new(CommandErrorKind::KnownHostsUnavailable, &error))
}

/// The trusted server keys in `~/.ssh/known_hosts`, in file order.
#[tauri::command]
pub fn list_known_hosts(
    services: State<'_, AppServices>,
) -> Result<Vec<KnownHostDto>, CommandError> {
    let entries = services
        .known_hosts
        .list()
        .map_err(|error| CommandError::new(CommandErrorKind::KnownHostsUnavailable, &error))?;
    Ok(entries.iter().map(KnownHostDto::from).collect())
}

/// Removes one trusted key; the next connection to that server asks again.
#[tauri::command]
pub fn forget_known_host(
    services: State<'_, AppServices>,
    entry: KnownHostDto,
) -> Result<(), CommandError> {
    services
        .known_hosts
        .forget(&entry.into())
        .map_err(|error| CommandError::new(CommandErrorKind::KnownHostsUnavailable, &error))
}
