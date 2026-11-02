use std::sync::Arc;

use sallygate_domain::inventory::SshEndpoint;
use sallygate_domain::trust::HostKey;

use super::{KnownHosts, KnownHostsError};

/// Use case: the user decided to trust the key a server presented.
pub struct TrustHostKey {
    known_hosts: Arc<dyn KnownHosts>,
}

impl TrustHostKey {
    #[must_use]
    pub fn new(known_hosts: Arc<dyn KnownHosts>) -> Self {
        Self { known_hosts }
    }

    /// # Errors
    ///
    /// Returns an error when the key cannot be saved.
    pub fn execute(&self, endpoint: &SshEndpoint, key: &HostKey) -> Result<(), KnownHostsError> {
        self.known_hosts.trust(endpoint, key)
    }
}
