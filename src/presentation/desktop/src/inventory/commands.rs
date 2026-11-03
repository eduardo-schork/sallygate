#![allow(
    clippy::needless_pass_by_value,
    reason = "Tauri commands receive their arguments by value"
)]

use sallygate_application::inventory::{HostStoreError, ImportError};
use sallygate_domain::inventory::HostId;
use tauri::State;

use super::contract::{HostDto, ImportReportDto, SaveHostRequest};
use crate::command_error::{CommandError, CommandErrorKind};
use crate::composition::AppServices;

fn store_error(error: &HostStoreError) -> CommandError {
    CommandError::new(CommandErrorKind::HostStoreUnavailable, error)
}

/// The saved hosts, grouped and sorted.
#[tauri::command]
pub fn list_hosts(services: State<'_, AppServices>) -> Result<Vec<HostDto>, CommandError> {
    let hosts = services
        .list_hosts
        .execute()
        .map_err(|error| store_error(&error))?;
    Ok(hosts.iter().map(HostDto::from).collect())
}

/// Saves a new host or the changes to one, and returns it as saved.
#[tauri::command]
pub fn save_host(
    services: State<'_, AppServices>,
    request: SaveHostRequest,
) -> Result<HostDto, CommandError> {
    let host = request.into_domain()?;
    services
        .save_host
        .execute(&host)
        .map_err(|error| store_error(&error))?;
    Ok(HostDto::from(&host))
}

/// Forgets a saved host.
#[tauri::command]
pub fn delete_host(services: State<'_, AppServices>, host_id: String) -> Result<(), CommandError> {
    let host_id = host_id
        .parse::<HostId>()
        .map_err(|error| CommandError::invalid_input(&error))?;
    services
        .delete_host
        .execute(host_id)
        .map_err(|error| store_error(&error))
}

/// Saves the hosts from `~/.ssh/config` that are not saved yet.
#[tauri::command]
pub fn import_ssh_config(
    services: State<'_, AppServices>,
) -> Result<ImportReportDto, CommandError> {
    let report = services
        .import_ssh_config
        .execute()
        .map_err(|error| match error {
            ImportError::Config(error) => {
                CommandError::new(CommandErrorKind::SshConfigUnreadable, &error)
            }
            ImportError::Store(error) => store_error(&error),
        })?;
    Ok(ImportReportDto::from(&report))
}
