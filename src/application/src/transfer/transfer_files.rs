use std::path::{Path, PathBuf};
use std::sync::Arc;

use sallygate_domain::sessions::SessionId;
use sallygate_domain::transfer::{file_name, join_remote_path};

use super::{OpenFileSystems, TransferError, TransferProgress};

/// Use case: copy files between this computer and the server.
pub struct TransferFiles {
    file_systems: Arc<OpenFileSystems>,
}

impl TransferFiles {
    #[must_use]
    pub fn new(file_systems: Arc<OpenFileSystems>) -> Self {
        Self { file_systems }
    }

    /// Copies a remote file into `local_folder` and returns where it landed.
    ///
    /// # Errors
    ///
    /// Returns an error when the session is closed or the copy fails.
    pub async fn download(
        &self,
        session_id: SessionId,
        remote_path: &str,
        local_folder: &Path,
        progress: Arc<dyn TransferProgress>,
    ) -> Result<PathBuf, TransferError> {
        let files = self.file_systems.of(session_id).await?;
        Ok(files.download(remote_path, local_folder, progress).await?)
    }

    /// Copies a local file into `remote_folder`, keeping its name, and returns the remote path.
    ///
    /// # Errors
    ///
    /// Returns an error when the session is closed or the copy fails.
    pub async fn upload(
        &self,
        session_id: SessionId,
        local_path: &Path,
        remote_folder: &str,
        progress: Arc<dyn TransferProgress>,
    ) -> Result<String, TransferError> {
        let files = self.file_systems.of(session_id).await?;
        let remote_path = join_remote_path(remote_folder, file_name(&local_path.to_string_lossy()));
        files.upload(local_path, &remote_path, progress).await?;
        Ok(remote_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transfer::test_doubles::{FakeRemoteFiles, IgnoreProgress, open_session};

    #[tokio::test]
    async fn uploads_into_the_folder_keeping_the_file_name() {
        let files = FakeRemoteFiles::with(&[]);
        let (registry, session_id, _) = open_session(files.clone());
        let transfers = TransferFiles::new(Arc::new(OpenFileSystems::new(registry)));

        let remote_path = transfers
            .upload(
                session_id,
                Path::new("/Users/me/report.pdf"),
                "/srv/",
                Arc::new(IgnoreProgress),
            )
            .await
            .unwrap();

        assert_eq!(remote_path, "/srv/report.pdf");
        assert_eq!(
            *files.calls.lock().unwrap(),
            ["upload /Users/me/report.pdf /srv/report.pdf"]
        );
    }

    #[tokio::test]
    async fn downloads_into_the_local_folder() {
        let (registry, session_id, _) = open_session(FakeRemoteFiles::with(&[]));
        let transfers = TransferFiles::new(Arc::new(OpenFileSystems::new(registry)));

        let local = transfers
            .download(
                session_id,
                "/srv/report.pdf",
                Path::new("/Users/me/Downloads"),
                Arc::new(IgnoreProgress),
            )
            .await
            .unwrap();

        assert_eq!(local, Path::new("/Users/me/Downloads/report.pdf"));
    }
}
