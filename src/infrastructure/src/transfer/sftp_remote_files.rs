use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use russh_sftp::client::SftpSession;
use russh_sftp::protocol::FileType;
use sallygate_application::transfer::{RemoteFiles, RemoteFilesError, TransferProgress};
use sallygate_domain::transfer::{EntryKind, FileEntry, file_name};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// Bytes copied between progress reports.
const CHUNK_BYTES: usize = 256 * 1024;

/// A server's files over SFTP (`russh-sftp`), on a channel of an open SSH connection.
pub struct SftpRemoteFiles {
    sftp: SftpSession,
}

impl SftpRemoteFiles {
    #[must_use]
    pub fn new(sftp: SftpSession) -> Self {
        Self { sftp }
    }
}

fn remote_error(error: &impl ToString) -> RemoteFilesError {
    RemoteFilesError(error.to_string())
}

fn entry_kind(file_type: FileType) -> EntryKind {
    match file_type {
        FileType::Dir => EntryKind::Folder,
        FileType::Symlink => EntryKind::Link,
        FileType::File | FileType::Other => EntryKind::File,
    }
}

/// `notes.txt`, then `notes (1).txt`, `notes (2).txt`… whichever is free in `folder`.
fn free_local_path(folder: &Path, name: &str) -> PathBuf {
    let candidate = folder.join(name);
    if !candidate.exists() {
        return candidate;
    }
    let (stem, extension) = match name.rsplit_once('.') {
        Some((stem, extension)) if !stem.is_empty() => (stem, format!(".{extension}")),
        _ => (name, String::new()),
    };
    (1..u32::MAX)
        .map(|number| folder.join(format!("{stem} ({number}){extension}")))
        .find(|path| !path.exists())
        .unwrap_or(candidate)
}

/// Copies `reader` into `writer`, reporting progress after each chunk.
async fn copy_with_progress(
    reader: &mut (impl AsyncRead + Unpin),
    writer: &mut (impl AsyncWrite + Unpin),
    total_bytes: u64,
    progress: &dyn TransferProgress,
) -> std::io::Result<()> {
    let mut buffer = vec![0; CHUNK_BYTES];
    let mut done_bytes = 0;
    progress.advanced(0, total_bytes);
    loop {
        let read = reader.read(&mut buffer).await?;
        if read == 0 {
            break;
        }
        writer.write_all(&buffer[..read]).await?;
        done_bytes += read as u64;
        progress.advanced(done_bytes, total_bytes);
    }
    writer.shutdown().await
}

#[async_trait]
impl RemoteFiles for SftpRemoteFiles {
    async fn home(&self) -> Result<String, RemoteFilesError> {
        self.sftp
            .canonicalize(".")
            .await
            .map_err(|error| remote_error(&error))
    }

    async fn list(&self, folder: &str) -> Result<Vec<FileEntry>, RemoteFilesError> {
        let entries = self
            .sftp
            .read_dir(folder)
            .await
            .map_err(|error| remote_error(&error))?;
        Ok(entries
            .filter(|entry| !matches!(entry.file_name().as_str(), "." | ".."))
            .map(|entry| {
                let metadata = entry.metadata();
                FileEntry {
                    path: sallygate_domain::transfer::join_remote_path(folder, &entry.file_name()),
                    name: entry.file_name(),
                    kind: entry_kind(entry.file_type()),
                    size_bytes: metadata.size.unwrap_or_default(),
                    modified_at: metadata.mtime.map(u64::from),
                    permissions: metadata.permissions.map(|mode| mode & 0o7777),
                }
            })
            .collect())
    }

    async fn make_folder(&self, path: &str) -> Result<(), RemoteFilesError> {
        self.sftp
            .create_dir(path)
            .await
            .map_err(|error| remote_error(&error))
    }

    async fn rename(&self, from: &str, to: &str) -> Result<(), RemoteFilesError> {
        self.sftp
            .rename(from, to)
            .await
            .map_err(|error| remote_error(&error))
    }

    async fn remove(&self, path: &str, kind: EntryKind) -> Result<(), RemoteFilesError> {
        match kind {
            EntryKind::Folder => self.sftp.remove_dir(path).await,
            EntryKind::File | EntryKind::Link => self.sftp.remove_file(path).await,
        }
        .map_err(|error| remote_error(&error))
    }

    async fn download(
        &self,
        remote_path: &str,
        local_folder: &Path,
        progress: Arc<dyn TransferProgress>,
    ) -> Result<PathBuf, RemoteFilesError> {
        let total_bytes = self
            .sftp
            .metadata(remote_path)
            .await
            .map_err(|error| remote_error(&error))?
            .size
            .unwrap_or_default();
        let mut remote = self
            .sftp
            .open(remote_path)
            .await
            .map_err(|error| remote_error(&error))?;
        let local_path = free_local_path(local_folder, file_name(remote_path));
        let mut local = tokio::fs::File::create(&local_path)
            .await
            .map_err(|error| remote_error(&error))?;
        if let Err(error) =
            copy_with_progress(&mut remote, &mut local, total_bytes, progress.as_ref()).await
        {
            // A half-written download would look like a real file.
            let _ = tokio::fs::remove_file(&local_path).await;
            return Err(remote_error(&error));
        }
        Ok(local_path)
    }

    async fn upload(
        &self,
        local_path: &Path,
        remote_path: &str,
        progress: Arc<dyn TransferProgress>,
    ) -> Result<(), RemoteFilesError> {
        let mut local = tokio::fs::File::open(local_path)
            .await
            .map_err(|error| remote_error(&error))?;
        let total_bytes = local
            .metadata()
            .await
            .map_err(|error| remote_error(&error))?
            .len();
        let mut remote = self
            .sftp
            .create(remote_path)
            .await
            .map_err(|error| remote_error(&error))?;
        copy_with_progress(&mut local, &mut remote, total_bytes, progress.as_ref())
            .await
            .map_err(|error| remote_error(&error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_a_free_local_name() {
        let folder = tempfile::tempdir().unwrap();
        assert_eq!(
            free_local_path(folder.path(), "notes.txt"),
            folder.path().join("notes.txt")
        );

        std::fs::write(folder.path().join("notes.txt"), "").unwrap();
        std::fs::write(folder.path().join("notes (1).txt"), "").unwrap();
        std::fs::write(folder.path().join("Makefile"), "").unwrap();

        assert_eq!(
            free_local_path(folder.path(), "notes.txt"),
            folder.path().join("notes (2).txt")
        );
        assert_eq!(
            free_local_path(folder.path(), "Makefile"),
            folder.path().join("Makefile (1)")
        );
    }
}
