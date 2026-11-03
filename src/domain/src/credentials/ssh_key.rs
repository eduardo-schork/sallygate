use std::fmt;

use super::CredentialsError;

/// The file name of a key in `~/.ssh`, like `id_ed25519`. Its public half is `<name>.pub`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SshKeyName(String);

impl SshKeyName {
    pub const MAX_LENGTH: usize = 64;

    /// Letters, digits, `.`, `_` and `-` only, so the name is a plain file in `~/.ssh`.
    ///
    /// # Errors
    ///
    /// Returns an error when the name is empty, too long, hidden, ends in `.pub` or has other
    /// characters.
    pub fn parse(name: &str) -> Result<Self, CredentialsError> {
        let name = name.trim();
        let allowed = |c: char| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-');
        if name.is_empty()
            || name.chars().count() > Self::MAX_LENGTH
            || name.starts_with('.')
            || name.to_ascii_lowercase().ends_with(".pub")
            || !name.chars().all(allowed)
        {
            return Err(CredentialsError::InvalidKeyName(name.to_owned()));
        }
        Ok(Self(name.to_owned()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SshKeyName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A key pair in the user's SSH folder, described by its public half only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SshKey {
    pub name: SshKeyName,
    /// Where the private key is, as a host's key file path would name it (`~/.ssh/<name>`).
    pub path: String,
    pub algorithm: String,
    /// `SHA256:…`
    pub fingerprint: String,
    /// The single line to paste into a server's `authorized_keys`.
    pub public_key: String,
    /// Whether the private key needs a passphrase to be used.
    pub is_encrypted: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_plain_file_names() {
        assert_eq!(
            SshKeyName::parse(" id_ed25519 ").unwrap().as_str(),
            "id_ed25519"
        );
        assert!(SshKeyName::parse("oracle-vps.key").is_ok());
    }

    #[test]
    fn rejects_names_that_are_not_a_plain_key_file() {
        for name in ["", ".hidden", "id.pub", "../escape", "with space", "a/b"] {
            assert!(SshKeyName::parse(name).is_err(), "{name}");
        }
    }
}
