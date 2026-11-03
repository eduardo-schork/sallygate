//! Tauri commands for trusting servers.

// Tauri's generate_handler! needs the defining module's path, so it is not re-exported.
pub mod commands;
mod contract;

pub use contract::{HostKeyDto, KnownHostDto, TrustHostKeyRequest};
