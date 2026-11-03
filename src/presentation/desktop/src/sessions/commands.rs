#![allow(
    clippy::needless_pass_by_value,
    reason = "Tauri commands receive their arguments by value"
)]

use std::sync::Arc;

use sallygate_application::sessions::OpenSessionInput;
use sallygate_domain::sessions::{SessionId, TerminalSize};
use tauri::State;
use tauri::ipc::Channel;

use super::channel_session_output::ChannelSessionOutput;
use super::contract::{OpenSessionRequest, SessionEventDto};
use super::error_mapping::{from_open_session_error, from_session_command_error};
use crate::command_error::CommandError;
use crate::composition::AppServices;

pub fn parse_session_id(session_id: &str) -> Result<SessionId, CommandError> {
    session_id
        .parse()
        .map_err(|error| CommandError::invalid_input(&error))
}

fn terminal_size(columns: u16, rows: u16) -> Result<TerminalSize, CommandError> {
    TerminalSize::new(columns, rows).map_err(|error| CommandError::invalid_input(&error))
}

/// Opens a terminal and returns its session id. Output arrives on `on_event`.
#[tauri::command]
pub async fn open_session(
    services: State<'_, AppServices>,
    mut request: OpenSessionRequest,
    on_event: Channel<SessionEventDto>,
) -> Result<String, CommandError> {
    let input = OpenSessionInput {
        secret: request.take_secret(),
        endpoint: request.endpoint.into_domain()?,
        auth: request.auth.into_domain()?,
        size: terminal_size(request.columns, request.rows)?,
    };

    let session_id = services
        .open_session
        .execute(input, Arc::new(ChannelSessionOutput::new(on_event)))
        .await
        .map_err(from_open_session_error)?;

    Ok(session_id.to_string())
}

/// Sends what the user typed or pasted to the session.
#[tauri::command]
pub async fn write_to_session(
    services: State<'_, AppServices>,
    session_id: String,
    data: String,
) -> Result<(), CommandError> {
    services
        .write_to_session
        .execute(parse_session_id(&session_id)?, data.as_bytes())
        .await
        .map_err(|error| from_session_command_error(&error))
}

/// Tells the session the terminal on screen changed size.
#[tauri::command]
pub async fn resize_session(
    services: State<'_, AppServices>,
    session_id: String,
    columns: u16,
    rows: u16,
) -> Result<(), CommandError> {
    services
        .resize_session
        .execute(
            parse_session_id(&session_id)?,
            terminal_size(columns, rows)?,
        )
        .await
        .map_err(|error| from_session_command_error(&error))
}

/// Closes the session and its connection.
#[tauri::command]
pub async fn close_session(
    services: State<'_, AppServices>,
    session_id: String,
) -> Result<(), CommandError> {
    services
        .close_session
        .execute(parse_session_id(&session_id)?)
        .await
        .map_err(|error| from_session_command_error(&error))
}
