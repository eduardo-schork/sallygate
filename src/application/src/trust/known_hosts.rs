use sallygate_domain::inventory::SshEndpoint;
use sallygate_domain::trust::{HostKey, HostKeyStatus, KnownHostEntry};
use thiserror::Error;

/// Port: the list of server keys the user already trusts.
pub trait KnownHosts: Send + Sync {
    /// Compares the key a server presented with the trusted one, if any.
    ///
    /// # Errors
    ///
    /// Returns an error when the list cannot be read.
    fn check(
        &self,
        endpoint: &SshEndpoint,
        key: &HostKey,
    ) -> Result<HostKeyStatus, KnownHostsError>;

    /// Records a key as trusted for this server.
    ///
    /// # Errors
    ///
    /// Returns an error when the list cannot be written.
    fn trust(&self, endpoint: &SshEndpoint, key: &HostKey) -> Result<(), KnownHostsError>;

    /// Every trusted key, in file order. Lines that are not keys (comments, markers) are left out.
    ///
    /// # Errors
    ///
    /// Returns an error when the list cannot be read.
    fn entries(&self) -> Result<Vec<KnownHostEntry>, KnownHostsError>;

    /// Removes one entry. Fails, changing nothing, when its line no longer holds that key.
    ///
    /// # Errors
    ///
    /// Returns an error when the entry moved or the list cannot be written.
    fn forget(&self, entry: &KnownHostEntry) -> Result<(), KnownHostsError>;
}

/// Why the trusted key list could not be used.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum KnownHostsError {
    #[error("could not read the known hosts file: {0}")]
    Read(String),
    #[error("could not save to the known hosts file: {0}")]
    Write(String),
    #[error("the known hosts file changed since it was shown; look at it again")]
    Moved,
}
