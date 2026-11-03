//! Tauri commands and the output channel for live terminal sessions.

mod channel_session_output;
// Tauri's generate_handler! needs the defining module's path, so it is not re-exported.
pub mod commands;
mod contract;
mod error_mapping;

pub use commands::parse_session_id;
pub use contract::{AuthDto, EndpointDto};
pub use error_mapping::from_session_command_error;
