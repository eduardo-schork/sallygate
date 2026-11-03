use std::path::Path;

use sallygate_domain::credentials::{Secret, SshKey, SshKeyName};
use thiserror::Error;

/// Port: the key pairs in the user's SSH folder.
pub trait KeyStore: Send + Sync {
    /// Every private key the folder holds, by name. Other files are left out.
    ///
    /// # Errors
    ///
    /// Returns an error when the folder cannot be read.
    fn list(&self) -> Result<Vec<SshKey>, KeyStoreError>;

    /// Makes a new Ed25519 pair, encrypted with `passphrase` when given.
    ///
    /// # Errors
    ///
    /// Returns an error when the name is taken or the files cannot be written.
    fn generate(
        &self,
        name: &SshKeyName,
        comment: &str,
        passphrase: Option<&Secret>,
    ) -> Result<SshKey, KeyStoreError>;

    /// Copies a private key file into the folder, keeping its name, with its public half.
    ///
    /// # Errors
    ///
    /// Returns an error when the file is not a private key, the name is taken or the copy fails.
    fn import(&self, source: &Path) -> Result<SshKey, KeyStoreError>;
}

/// Why the SSH folder could not be used. Messages are safe to show the user.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum KeyStoreError {
    #[error("could not read the SSH folder: {0}")]
    Read(String),
    #[error("could not write the key: {0}")]
    Write(String),
    #[error("a key named {0} already exists")]
    AlreadyExists(String),
    #[error("{0} is not an OpenSSH private key")]
    NotAKey(String),
}
