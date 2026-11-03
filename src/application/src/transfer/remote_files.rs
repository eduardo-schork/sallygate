use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use sallygate_domain::transfer::{EntryKind, FileEntry};
use thiserror::Error;

/// Port: the files on the server behind one open session.
#[async_trait]
pub trait RemoteFiles: Send + Sync {
    /// The folder the login starts in.
    async fn home(&self) -> Result<String, RemoteFilesError>;

    /// The entries of a folder, in no particular order, without `.` and `..`.
    async fn list(&self, folder: &str) -> Result<Vec<FileEntry>, RemoteFilesError>;

    async fn make_folder(&self, path: &str) -> Result<(), RemoteFilesError>;

    async fn rename(&self, from: &str, to: &str) -> Result<(), RemoteFilesError>;

    /// Removes a file or link, or an empty folder.
    async fn remove(&self, path: &str, kind: EntryKind) -> Result<(), RemoteFilesError>;

    /// Copies a remote file into a local folder under a name not taken yet, and returns where.
    async fn download(
        &self,
        remote_path: &str,
        local_folder: &Path,
        progress: Arc<dyn TransferProgress>,
    ) -> Result<PathBuf, RemoteFilesError>;

    /// Copies a local file to a remote path, replacing what is there.
    async fn upload(
        &self,
        local_path: &Path,
        remote_path: &str,
        progress: Arc<dyn TransferProgress>,
    ) -> Result<(), RemoteFilesError>;
}

/// Port: told how far a transfer got, so the UI can show it.
pub trait TransferProgress: Send + Sync {
    fn advanced(&self, done_bytes: u64, total_bytes: u64);
}

/// A remote file operation failed; the text is safe to show the user.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{0}")]
pub struct RemoteFilesError(pub String);
