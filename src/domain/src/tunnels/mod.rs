//! Port forwarding: reach a service through a saved host's SSH connection.

mod tunnel;
mod tunnels_error;

pub use tunnel::{Tunnel, TunnelId, TunnelKind, TunnelName};
pub use tunnels_error::TunnelsError;
