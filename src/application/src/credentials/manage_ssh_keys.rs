use std::path::Path;
use std::sync::Arc;

use sallygate_domain::credentials::{Secret, SshKey, SshKeyName};

use super::{KeyStore, KeyStoreError};

/// Use case: see, make and bring in SSH keys. Copying a public key is the UI's job.
pub struct ManageSshKeys {
    store: Arc<dyn KeyStore>,
}

impl ManageSshKeys {
    #[must_use]
    pub fn new(store: Arc<dyn KeyStore>) -> Self {
        Self { store }
    }

    /// Keys sorted by name.
    ///
    /// # Errors
    ///
    /// Returns an error when the folder cannot be read.
    pub fn list(&self) -> Result<Vec<SshKey>, KeyStoreError> {
        let mut keys = self.store.list()?;
        keys.sort_by(|left, right| left.name.as_str().cmp(right.name.as_str()));
        Ok(keys)
    }

    /// # Errors
    ///
    /// Returns an error when the name is taken or the key cannot be written.
    pub fn generate(
        &self,
        name: &SshKeyName,
        comment: &str,
        passphrase: Option<&Secret>,
    ) -> Result<SshKey, KeyStoreError> {
        self.store.generate(name, comment.trim(), passphrase)
    }

    /// # Errors
    ///
    /// Returns an error when the file is not a key or cannot be copied.
    pub fn import(&self, source: &Path) -> Result<SshKey, KeyStoreError> {
        self.store.import(source)
    }
}
