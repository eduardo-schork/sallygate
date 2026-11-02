use tauri::State;

use super::TrustHostKeyRequest;
use crate::command_error::{CommandError, CommandErrorKind};
use crate::composition::AppServices;

/// Records the server key the user chose to trust.
#[tauri::command]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Tauri commands receive their arguments by value"
)]
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
