use sallygate_domain::tunnels::{Tunnel, TunnelId};
use thiserror::Error;

/// Port: where forwarding rules are kept.
pub trait TunnelRepository: Send + Sync {
    /// # Errors
    ///
    /// Returns an error when the rules cannot be read.
    fn all(&self) -> Result<Vec<Tunnel>, TunnelStoreError>;

    /// Adds the rule, or replaces the one with the same id.
    ///
    /// # Errors
    ///
    /// Returns an error when the rules cannot be written.
    fn save(&self, tunnel: &Tunnel) -> Result<(), TunnelStoreError>;

    /// # Errors
    ///
    /// Returns an error when the rules cannot be written.
    fn delete(&self, tunnel_id: TunnelId) -> Result<(), TunnelStoreError>;
}

/// Why rules could not be read or written. Messages are safe to show the user.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TunnelStoreError {
    #[error("could not read the forwarding rules: {0}")]
    Read(String),
    #[error("could not save the forwarding rules: {0}")]
    Write(String),
}
