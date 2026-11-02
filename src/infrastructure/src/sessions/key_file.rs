use std::path::{Path, PathBuf};

use russh::keys::{PrivateKey, load_secret_key};
use sallygate_application::sessions::OpenSessionError;
use sallygate_domain::credentials::KeyFilePath;

/// Reads a private key file without a passphrase. A leading `~` means the home folder,
/// as in the `ssh` command.
pub(super) fn load_private_key(path: &KeyFilePath) -> Result<PrivateKey, OpenSessionError> {
    let path = expand_home(path.as_path());

    load_secret_key(&path, None).map_err(|error| {
        OpenSessionError::KeyFileUnreadable(format!("{}: {error}", path.display()))
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

        let error = load_private_key(&path).unwrap_err();

        assert!(
            matches!(error, OpenSessionError::KeyFileUnreadable(message) if message.contains("/does/not/exist"))
        );
    }
}
