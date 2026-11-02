use std::path::{Path, PathBuf};

use super::CredentialsError;

/// Where a private key file is on this machine. The key itself is never held here.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyFilePath(PathBuf);

impl KeyFilePath {
    /// Validates a key file path typed by the user. Whether the file exists is
    /// checked when it is read, not here.
    ///
    /// # Errors
    ///
    /// Returns an error when the path is empty.
    pub fn parse(path: &str) -> Result<Self, CredentialsError> {
        let path = path.trim();

        if path.is_empty() {
            return Err(CredentialsError::EmptyKeyFilePath);
        }

        Ok(Self(PathBuf::from(path)))
    }

    #[must_use]
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_and_trims_a_path() {
        let path = KeyFilePath::parse(" ~/.ssh/id_ed25519 ").unwrap();
        assert_eq!(path.as_path(), Path::new("~/.ssh/id_ed25519"));
    }

    #[test]
    fn rejects_an_empty_path() {
        assert_eq!(
            KeyFilePath::parse(" "),
            Err(CredentialsError::EmptyKeyFilePath)
        );
    }
}
