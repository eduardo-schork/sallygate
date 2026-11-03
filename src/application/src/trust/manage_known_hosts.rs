use std::sync::Arc;

use sallygate_domain::trust::KnownHostEntry;

use super::{KnownHosts, KnownHostsError};

/// Use case: review the trusted server keys and forget one, e.g. after a server is rebuilt.
pub struct ManageKnownHosts {
    known_hosts: Arc<dyn KnownHosts>,
}

impl ManageKnownHosts {
    #[must_use]
    pub fn new(known_hosts: Arc<dyn KnownHosts>) -> Self {
        Self { known_hosts }
    }

    /// # Errors
    ///
    /// Returns an error when the list cannot be read.
    pub fn list(&self) -> Result<Vec<KnownHostEntry>, KnownHostsError> {
        self.known_hosts.entries()
    }

    /// # Errors
    ///
    /// Returns an error when the entry moved or the list cannot be written.
    pub fn forget(&self, entry: &KnownHostEntry) -> Result<(), KnownHostsError> {
        self.known_hosts.forget(entry)
    }
}
