use std::fs;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use russh::keys::ssh_key::LineEnding;
use russh::keys::{Algorithm, HashAlg, PrivateKey};
use sallygate_application::credentials::{KeyStore, KeyStoreError};
use sallygate_domain::credentials::{Secret, SshKey, SshKeyName};

/// Private keys are readable by their owner only, or `ssh` refuses them.
const PRIVATE_KEY_MODE: u32 = 0o600;
const PUBLIC_KEY_MODE: u32 = 0o644;

/// The key pairs in one folder, normally `~/.ssh`, in `OpenSSH` format.
pub struct SshFolderKeyStore {
    folder: PathBuf,
    /// How paths are shown and saved in hosts, e.g. `~/.ssh`.
    display_folder: String,
}

impl SshFolderKeyStore {
    #[must_use]
    pub fn new(folder: PathBuf, display_folder: String) -> Self {
        Self {
            folder,
            display_folder,
        }
    }

    fn describe(&self, name: SshKeyName, key: &PrivateKey) -> Result<SshKey, String> {
        let public_key = key.public_key();
        Ok(SshKey {
            path: format!("{}/{name}", self.display_folder.trim_end_matches('/')),
            name,
            algorithm: public_key.algorithm().to_string(),
            fingerprint: public_key.fingerprint(HashAlg::Sha256).to_string(),
            public_key: public_key.to_openssh().map_err(|error| error.to_string())?,
            is_encrypted: key.is_encrypted(),
        })
    }

    /// Refuses a name that is taken by either half, so nothing is overwritten.
    fn free_path(&self, name: &SshKeyName) -> Result<PathBuf, KeyStoreError> {
        let path = self.folder.join(name.as_str());
        if path.exists() || path.with_extension(public_extension(&path)).exists() {
            return Err(KeyStoreError::AlreadyExists(name.to_string()));
        }
        Ok(path)
    }

    fn write_pair(&self, path: &Path, key: &PrivateKey) -> Result<(), KeyStoreError> {
        let write_error = |error: &dyn ToString| KeyStoreError::Write(error.to_string());
        fs::create_dir_all(&self.folder).map_err(|error| write_error(&error))?;
        let private_text = key
            .to_openssh(LineEnding::LF)
            .map_err(|error| write_error(&error))?;
        let public_text = key
            .public_key()
            .to_openssh()
            .map_err(|error| write_error(&error))?;
        write_new_file(path, private_text.as_bytes(), PRIVATE_KEY_MODE)
            .map_err(|error| write_error(&error))?;
        write_new_file(
            &path.with_extension(public_extension(path)),
            format!("{public_text}\n").as_bytes(),
            PUBLIC_KEY_MODE,
        )
        .map_err(|error| write_error(&error))
    }
}

/// `id_ed25519` → `id_ed25519.pub`; `oracle.key` → `oracle.key.pub`.
fn public_extension(path: &Path) -> String {
    match path.extension() {
        Some(extension) => format!("{}.pub", extension.to_string_lossy()),
        None => "pub".to_owned(),
    }
}

/// Creates the file with its permissions from the start; fails if it already exists.
fn write_new_file(path: &Path, contents: &[u8], mode: u32) -> std::io::Result<()> {
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(mode)
        .open(path)?
        .write_all(contents)
}

impl KeyStore for SshFolderKeyStore {
    fn list(&self) -> Result<Vec<SshKey>, KeyStoreError> {
        let entries = match fs::read_dir(&self.folder) {
            Ok(entries) => entries,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(KeyStoreError::Read(error.to_string())),
        };
        Ok(entries
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
            .filter_map(|entry| {
                let name = SshKeyName::parse(&entry.file_name().to_string_lossy()).ok()?;
                // Only files that parse as private keys; config, known_hosts and the rest are skipped.
                let key = PrivateKey::read_openssh_file(entry.path()).ok()?;
                self.describe(name, &key).ok()
            })
            .collect())
    }

    fn generate(
        &self,
        name: &SshKeyName,
        comment: &str,
        passphrase: Option<&Secret>,
    ) -> Result<SshKey, KeyStoreError> {
        let path = self.free_path(name)?;
        let mut rng = rand::rng();
        let mut key = PrivateKey::random(&mut rng, Algorithm::Ed25519)
            .map_err(|error| KeyStoreError::Write(error.to_string()))?;
        key.set_comment(comment);
        if let Some(passphrase) = passphrase {
            key = key
                .encrypt(&mut rng, passphrase.expose())
                .map_err(|error| KeyStoreError::Write(error.to_string()))?;
        }
        self.write_pair(&path, &key)?;
        self.describe(name.clone(), &key)
            .map_err(KeyStoreError::Write)
    }

    fn import(&self, source: &Path) -> Result<SshKey, KeyStoreError> {
        let shown = source.display().to_string();
        let file_name = source
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let name = SshKeyName::parse(&file_name)
            .map_err(|error| KeyStoreError::NotAKey(format!("{shown} ({error})")))?;
        let key =
            PrivateKey::read_openssh_file(source).map_err(|_| KeyStoreError::NotAKey(shown))?;
        let path = self.free_path(&name)?;
        self.write_pair(&path, &key)?;
        self.describe(name, &key).map_err(KeyStoreError::Write)
    }
}

#[cfg(test)]
mod tests {
    use std::os::unix::fs::PermissionsExt;

    use super::*;

    fn store(folder: &Path) -> SshFolderKeyStore {
        SshFolderKeyStore::new(folder.to_path_buf(), "~/.ssh".to_owned())
    }

    fn mode(path: &Path) -> u32 {
        fs::metadata(path).unwrap().permissions().mode() & 0o777
    }

    #[test]
    fn generates_a_pair_ssh_accepts_and_lists_it() {
        let folder = tempfile::tempdir().unwrap();
        let keys = store(folder.path());
        let name = SshKeyName::parse("id_sallygate").unwrap();

        let key = keys.generate(&name, "me@mac", None).unwrap();

        assert_eq!(key.path, "~/.ssh/id_sallygate");
        assert_eq!(key.algorithm, "ssh-ed25519");
        assert!(key.public_key.starts_with("ssh-ed25519 "));
        assert!(key.public_key.ends_with(" me@mac"));
        assert!(!key.is_encrypted);
        assert_eq!(mode(&folder.path().join("id_sallygate")), 0o600);
        assert_eq!(mode(&folder.path().join("id_sallygate.pub")), 0o644);
        assert_eq!(keys.list().unwrap(), vec![key]);
    }

    #[test]
    fn encrypts_with_a_passphrase_and_never_overwrites() {
        let folder = tempfile::tempdir().unwrap();
        let keys = store(folder.path());
        let name = SshKeyName::parse("locked").unwrap();
        let secret = Secret::new("correct horse".to_owned());

        let key = keys.generate(&name, "", Some(&secret)).unwrap();

        assert!(key.is_encrypted);
        assert_eq!(
            keys.generate(&name, "", None),
            Err(KeyStoreError::AlreadyExists("locked".to_owned()))
        );
    }

    #[test]
    fn imports_a_private_key_and_refuses_other_files() {
        let source_folder = tempfile::tempdir().unwrap();
        let keys_folder = tempfile::tempdir().unwrap();
        store(source_folder.path())
            .generate(&SshKeyName::parse("oracle.key").unwrap(), "", None)
            .unwrap();
        let notes = source_folder.path().join("notes.txt");
        fs::write(&notes, "hello").unwrap();
        let keys = store(keys_folder.path());

        let imported = keys
            .import(&source_folder.path().join("oracle.key"))
            .unwrap();

        assert_eq!(imported.path, "~/.ssh/oracle.key");
        assert!(keys_folder.path().join("oracle.key.pub").exists());
        assert!(matches!(
            keys.import(&notes),
            Err(KeyStoreError::NotAKey(_))
        ));
        assert_eq!(keys.list().unwrap().len(), 1);
    }
}
