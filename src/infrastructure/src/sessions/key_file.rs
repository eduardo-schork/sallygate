use std::path::{Path, PathBuf};

use russh::keys::{Error as KeyError, PrivateKey, load_secret_key};
use sallygate_application::sessions::OpenSessionError;
use sallygate_domain::credentials::{KeyFilePath, Secret};

/// Reads a private key file, decrypting it with `passphrase` when it has one. A leading
/// `~` means the home folder, as in the `ssh` command.
pub(super) fn load_private_key(
    path: &KeyFilePath,
    passphrase: Option<&Secret>,
) -> Result<PrivateKey, OpenSessionError> {
    let path = expand_home(path.as_path());

    load_secret_key(&path, passphrase.map(Secret::expose)).map_err(|error| match error {
        KeyError::KeyIsEncrypted => OpenSessionError::SecretRequired,
        error => OpenSessionError::KeyFileUnreadable(format!("{}: {error}", path.display())),
    })
}

fn expand_home(path: &Path) -> PathBuf {
    match (path.strip_prefix("~"), std::env::home_dir()) {
        (Ok(relative), Some(home)) => home.join(relative),
        _ => path.to_path_buf(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_a_leading_tilde_to_the_home_folder() {
        let home = std::env::home_dir().unwrap();
        assert_eq!(
            expand_home(Path::new("~/.ssh/id_ed25519")),
            home.join(".ssh/id_ed25519")
        );
    }

    #[test]
    fn leaves_other_paths_as_they_are() {
        assert_eq!(
            expand_home(Path::new("/keys/id_ed25519")),
            PathBuf::from("/keys/id_ed25519")
        );
    }

    #[test]
    fn reports_a_missing_key_file_with_its_path() {
        let path = KeyFilePath::parse("/does/not/exist").unwrap();

        let error = load_private_key(&path, None).unwrap_err();

        assert!(
            matches!(error, OpenSessionError::KeyFileUnreadable(message) if message.contains("/does/not/exist"))
        );
    }

    /// An ed25519 key encrypted with the passphrase "test", from russh's own tests.
    const ENCRYPTED_KEY: &str = "-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAACmFlczI1Ni1jdHIAAAAGYmNyeXB0AAAAGAAAABD1phlku5
A2G7Q9iP+DcOc9AAAAEAAAAAEAAAAzAAAAC3NzaC1lZDI1NTE5AAAAIHeLC1lWiCYrXsf/
85O/pkbUFZ6OGIt49PX3nw8iRoXEAAAAkKRF0st5ZI7xxo9g6A4m4l6NarkQre3mycqNXQ
dP3jryYgvsCIBAA5jMWSjrmnOTXhidqcOy4xYCrAttzSnZ/cUadfBenL+DQq6neffw7j8r
0tbCxVGp6yCQlKrgSZf6c0Hy7dNEIU2bJFGxLe6/kWChcUAt/5Ll5rI7DVQPJdLgehLzvv
sJWR7W+cGvJ/vLsw==
-----END OPENSSH PRIVATE KEY-----
";

    fn encrypted_key_file() -> (tempfile::TempDir, KeyFilePath) {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("id_ed25519");
        std::fs::write(&path, ENCRYPTED_KEY).unwrap();
        let key_file_path = KeyFilePath::parse(path.to_str().unwrap()).unwrap();
        (directory, key_file_path)
    }

    #[test]
    fn asks_for_the_passphrase_of_an_encrypted_key() {
        let (_directory, path) = encrypted_key_file();

        let error = load_private_key(&path, None).unwrap_err();

        assert_eq!(error, OpenSessionError::SecretRequired);
    }

    #[test]
    fn decrypts_a_key_with_its_passphrase() {
        let (_directory, path) = encrypted_key_file();

        let key = load_private_key(&path, Some(&Secret::new("test".to_owned())));

        assert!(key.is_ok(), "{:?}", key.err());
    }
}
