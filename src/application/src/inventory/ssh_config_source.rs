use thiserror::Error;

/// One `Host` block of the user's SSH config, with OpenSSH's defaults already filled in.
/// Values are raw text: the use case checks them before they become a saved host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SshConfigEntry {
    pub alias: String,
    pub address: String,
    pub port: u16,
    pub username: String,
    /// `None` when neither the config nor the default key files name a key.
    pub key_file: Option<String>,
}

/// Port: the hosts the user already set up for the `ssh` command.
pub trait SshConfigSource: Send + Sync {
    /// The config's concrete hosts. Patterns such as `Host *` are left out. A missing config is
    /// an empty list, not an error.
    ///
    /// # Errors
    ///
    /// Returns an error when the config exists but cannot be read.
    fn entries(&self) -> Result<Vec<SshConfigEntry>, SshConfigError>;
}

/// Why the SSH config could not be read. Messages are safe to show the user.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SshConfigError {
    #[error("could not read ~/.ssh/config: {0}")]
    Read(String),
}
