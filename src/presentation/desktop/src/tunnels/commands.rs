#![allow(
    clippy::needless_pass_by_value,
    reason = "Tauri commands receive their arguments by value"
)]

use sallygate_application::tunnels::TunnelStoreError;
use sallygate_domain::credentials::Secret;
use sallygate_domain::tunnels::TunnelId;
use tauri::State;

use super::contract::{SaveTunnelRequest, TunnelDto, from_start_tunnel_error};
use crate::command_error::{CommandError, CommandErrorKind};
use crate::composition::AppServices;

fn store_error(error: &TunnelStoreError) -> CommandError {
    CommandError::new(CommandErrorKind::TunnelStoreUnavailable, error)
}

fn parse_tunnel_id(tunnel_id: &str) -> Result<TunnelId, CommandError> {
    tunnel_id
        .parse()
        .map_err(|error| CommandError::invalid_input(&error))
}

/// The saved rules, by name, each with whether it is on.
#[tauri::command]
pub fn list_tunnels(services: State<'_, AppServices>) -> Result<Vec<TunnelDto>, CommandError> {
    let states = services
        .tunnels
        .list()
        .map_err(|error| store_error(&error))?;
    Ok(states.iter().map(TunnelDto::from).collect())
}

/// Saves a new rule or the changes to one.
#[tauri::command]
pub fn save_tunnel(
    services: State<'_, AppServices>,
    request: SaveTunnelRequest,
) -> Result<(), CommandError> {
    let tunnel = request.into_domain()?;
    services
        .tunnels
        .save(&tunnel)
        .map_err(|error| store_error(&error))
}

/// Turns a rule off and forgets it.
#[tauri::command]
pub async fn delete_tunnel(
    services: State<'_, AppServices>,
    tunnel_id: String,
) -> Result<(), CommandError> {
    services
        .tunnels
        .delete(parse_tunnel_id(&tunnel_id)?)
        .await
        .map_err(|error| store_error(&error))
}

/// Logs in to the rule's host and starts forwarding. A password or passphrase, when the host
/// needs one, is used once and never kept (ADR-0015).
#[tauri::command]
pub async fn start_tunnel(
    services: State<'_, AppServices>,
    tunnel_id: String,
    secret: Option<String>,
) -> Result<(), CommandError> {
    services
        .tunnels
        .start(parse_tunnel_id(&tunnel_id)?, secret.map(Secret::new))
        .await
        .map_err(from_start_tunnel_error)
}

/// Stops forwarding.
#[tauri::command]
pub async fn stop_tunnel(
    services: State<'_, AppServices>,
    tunnel_id: String,
) -> Result<(), CommandError> {
    services.tunnels.stop(parse_tunnel_id(&tunnel_id)?).await;
    Ok(())
}
