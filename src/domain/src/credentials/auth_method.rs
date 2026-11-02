use super::KeyFilePath;

/// How a session authenticates. The method is saved with a host; a secret it needs
/// (a password or a key passphrase) is not, see [`super::Secret`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthMethod {
    /// A private key file, maybe encrypted with a passphrase.
    KeyFile(KeyFilePath),
    /// A password typed when the session opens.
    Password,
}
