use super::KeyFilePath;

/// How a session authenticates. Password and ssh-agent arrive in Stage 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthMethod {
    /// A private key file without a passphrase.
    KeyFile(KeyFilePath),
}
