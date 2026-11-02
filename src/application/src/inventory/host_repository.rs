use sallygate_domain::inventory::{Host, HostId};
use thiserror::Error;

/// Port: where saved hosts are kept.
pub trait HostRepository: Send + Sync {
    /// Every saved host, in no particular order.
    ///
    /// # Errors
    ///
    /// Returns an error when the hosts cannot be read.
    fn all(&self) -> Result<Vec<Host>, HostStoreError>;

    /// Adds the host, or replaces the saved host with the same id.
    ///
    /// # Errors
    ///
    /// Returns an error when the host cannot be written.
    fn save(&self, host: &Host) -> Result<(), HostStoreError>;

    /// Removes the host with this id. Removing a host that is not there is not an error.
    ///
    /// # Errors
    ///
    /// Returns an error when the hosts cannot be written.
    fn delete(&self, host_id: HostId) -> Result<(), HostStoreError>;
}

/// Why saved hosts could not be read or written. Messages are safe to show the user.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum HostStoreError {
    #[error("could not read the saved hosts: {0}")]
    Read(String),
    #[error("could not save the hosts: {0}")]
    Write(String),
}
