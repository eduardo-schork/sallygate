use std::path::{Path, PathBuf};
use std::sync::Arc;

use sallygate_domain::sessions::SessionId;
use sallygate_domain::transfer::{EntryKind, file_name, free_name, join_remote_path};

use super::{LocalFiles, OpenFileSystems, RemoteFiles, TransferError, TransferProgress};

/// Use case: copy files and folders between this computer and the server. A copy never
/// overwrites: a taken name becomes `name (1)`. Links inside a folder are skipped, so a link
/// loop cannot make a copy endless.
pub struct TransferFiles {
    file_systems: Arc<OpenFileSystems>,
    local: Arc<dyn LocalFiles>,
}

/// Turns one file's progress into the whole folder's: bytes already copied plus this file's.
struct FolderProgress {
    copied_bytes: u64,
    total_bytes: u64,
    folder: Arc<dyn TransferProgress>,
}

impl TransferProgress for FolderProgress {
    fn advanced(&self, done_bytes: u64, _file_bytes: u64) {
        self.folder
            .advanced(self.copied_bytes + done_bytes, self.total_bytes);
    }
}

/// What a folder copy will do, worked out before the first byte moves.
struct CopyPlan<Source, Target> {
    /// Destination folders, parents before children.
    folders: Vec<Target>,
    /// Each file: where it is, the folder it goes to, its size.
    files: Vec<(Source, Target, u64)>,
    total_bytes: u64,
}

impl TransferFiles {
    #[must_use]
    pub fn new(file_systems: Arc<OpenFileSystems>, local: Arc<dyn LocalFiles>) -> Self {
        Self {
            file_systems,
            local,
        }
    }

    /// Copies a remote file or folder into `local_folder` and returns where it landed.
    ///
    /// # Errors
    ///
    /// Returns an error when the session is closed or a copy fails.
    pub async fn download(
        &self,
        session_id: SessionId,
        remote_path: &str,
        kind: EntryKind,
        local_folder: &Path,
        progress: Arc<dyn TransferProgress>,
    ) -> Result<PathBuf, TransferError> {
        let files = self.file_systems.of(session_id).await?;
        if kind != EntryKind::Folder {
            return Ok(files.download(remote_path, local_folder, progress).await?);
        }

        let taken = self.local_names(local_folder)?;
        let root = local_folder.join(free_name(file_name(remote_path), |name| {
            taken.iter().any(|taken| taken == name)
        }));
        let plan = remote_plan(files.as_ref(), remote_path, &root).await?;
        for folder in &plan.folders {
            self.local.make_folder(folder)?;
        }
        let mut copied_bytes = 0;
        for (remote_file, local_folder, size) in &plan.files {
            let file_progress = Arc::new(FolderProgress {
                copied_bytes,
                total_bytes: plan.total_bytes,
                folder: Arc::clone(&progress),
            });
            files
                .download(remote_file, local_folder, file_progress)
                .await?;
            copied_bytes += size;
        }
        progress.advanced(plan.total_bytes, plan.total_bytes);
        Ok(root)
    }

    /// Copies a local file or folder into `remote_folder` and returns the remote path. A file
    /// keeps its name and replaces one of the same name; a folder gets a free name.
    ///
    /// # Errors
    ///
    /// Returns an error when the session is closed or a copy fails.
    pub async fn upload(
        &self,
        session_id: SessionId,
        local_path: &Path,
        remote_folder: &str,
        progress: Arc<dyn TransferProgress>,
    ) -> Result<String, TransferError> {
        let files = self.file_systems.of(session_id).await?;
        let name = file_name(&local_path.to_string_lossy()).to_owned();
        if self.local.kind(local_path)? != EntryKind::Folder {
            let remote_path = join_remote_path(remote_folder, &name);
            files.upload(local_path, &remote_path, progress).await?;
            return Ok(remote_path);
        }

        let taken: Vec<String> = files
            .list(remote_folder)
            .await?
            .into_iter()
            .map(|entry| entry.name)
            .collect();
        let root = join_remote_path(
            remote_folder,
            &free_name(&name, |candidate| {
                taken.iter().any(|taken| taken == candidate)
            }),
        );
        let plan = self.local_plan(local_path, &root)?;
        for folder in &plan.folders {
            files.make_folder(folder).await?;
        }
        let mut copied_bytes = 0;
        for (local_file, remote_folder, size) in &plan.files {
            let file_progress = Arc::new(FolderProgress {
                copied_bytes,
                total_bytes: plan.total_bytes,
                folder: Arc::clone(&progress),
            });
            let remote_path =
                join_remote_path(remote_folder, file_name(&local_file.to_string_lossy()));
            files
                .upload(local_file, &remote_path, file_progress)
                .await?;
            copied_bytes += size;
        }
        progress.advanced(plan.total_bytes, plan.total_bytes);
        Ok(root)
    }

    fn local_names(&self, folder: &Path) -> Result<Vec<String>, TransferError> {
        Ok(self
            .local
            .list(&folder.to_string_lossy())?
            .into_iter()
            .map(|entry| entry.name)
            .collect())
    }

    /// Walks a local folder, breadth first, into the remote folders and files to make.
    fn local_plan(
        &self,
        local_root: &Path,
        remote_root: &str,
    ) -> Result<CopyPlan<PathBuf, String>, TransferError> {
        let mut plan = CopyPlan {
            folders: vec![remote_root.to_owned()],
            files: Vec::new(),
            total_bytes: 0,
        };
        let mut pending = vec![(local_root.to_path_buf(), remote_root.to_owned())];
        while let Some((local_folder, remote_folder)) = pending.pop() {
            for entry in self.local.list(&local_folder.to_string_lossy())? {
                match entry.kind {
                    EntryKind::Folder => {
                        let remote = join_remote_path(&remote_folder, &entry.name);
                        plan.folders.push(remote.clone());
                        pending.push((PathBuf::from(entry.path), remote));
                    }
                    EntryKind::File => {
                        plan.total_bytes += entry.size_bytes;
                        plan.files.push((
                            PathBuf::from(entry.path),
                            remote_folder.clone(),
                            entry.size_bytes,
                        ));
                    }
                    EntryKind::Link => {}
                }
            }
        }
        Ok(plan)
    }
}

/// Walks a remote folder, breadth first, into the local folders and files to make.
async fn remote_plan(
    files: &dyn RemoteFiles,
    remote_root: &str,
    local_root: &Path,
) -> Result<CopyPlan<String, PathBuf>, TransferError> {
    let mut plan = CopyPlan {
        folders: vec![local_root.to_path_buf()],
        files: Vec::new(),
        total_bytes: 0,
    };
    let mut pending = vec![(remote_root.to_owned(), local_root.to_path_buf())];
    while let Some((remote_folder, local_folder)) = pending.pop() {
        for entry in files.list(&remote_folder).await? {
            match entry.kind {
                EntryKind::Folder => {
                    let local = local_folder.join(&entry.name);
                    plan.folders.push(local.clone());
                    pending.push((entry.path, local));
                }
                EntryKind::File => {
                    plan.total_bytes += entry.size_bytes;
                    plan.files
                        .push((entry.path, local_folder.clone(), entry.size_bytes));
                }
                EntryKind::Link => {}
            }
        }
    }
    Ok(plan)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transfer::test_doubles::{
        FakeLocalFiles, FakeRemoteFiles, IgnoreProgress, RecordingProgress, open_session,
    };

    fn transfers(
        remote: Arc<FakeRemoteFiles>,
        local: Arc<FakeLocalFiles>,
    ) -> (TransferFiles, SessionId) {
        let (registry, session_id, _) = open_session(remote);
        (
            TransferFiles::new(Arc::new(OpenFileSystems::new(registry)), local),
            session_id,
        )
    }

    #[tokio::test]
    async fn copies_single_files_both_ways() {
        let remote = FakeRemoteFiles::with(&[]);
        let local = FakeLocalFiles::with(&[("/Users/me/report.pdf", EntryKind::File)]);
        let (transfers, session_id) = transfers(remote.clone(), local);

        let uploaded = transfers
            .upload(
                session_id,
                Path::new("/Users/me/report.pdf"),
                "/srv/",
                Arc::new(IgnoreProgress),
            )
            .await
            .unwrap();
        let downloaded = transfers
            .download(
                session_id,
                "/srv/report.pdf",
                EntryKind::File,
                Path::new("/Users/me/Downloads"),
                Arc::new(IgnoreProgress),
            )
            .await
            .unwrap();

        assert_eq!(uploaded, "/srv/report.pdf");
        assert_eq!(downloaded, Path::new("/Users/me/Downloads/report.pdf"));
        assert_eq!(
            *remote.calls.lock().unwrap(),
            ["upload /Users/me/report.pdf /srv/report.pdf"]
        );
    }

    #[tokio::test]
    async fn downloads_a_folder_with_its_subfolders_under_a_free_name() {
        let remote = FakeRemoteFiles::with(&[
            ("/srv/apps/a.txt", EntryKind::File),
            ("/srv/apps/web", EntryKind::Folder),
            ("/srv/apps/web/index.html", EntryKind::File),
            ("/srv/apps/current", EntryKind::Link),
        ]);
        let local = FakeLocalFiles::with(&[("/Users/me/apps", EntryKind::Folder)]);
        let (transfers, session_id) = transfers(remote, local.clone());
        let progress = Arc::new(RecordingProgress::default());

        let landed = transfers
            .download(
                session_id,
                "/srv/apps",
                EntryKind::Folder,
                Path::new("/Users/me"),
                progress.clone(),
            )
            .await
            .unwrap();

        assert_eq!(landed, Path::new("/Users/me/apps (1)"));
        assert_eq!(
            *local.made_folders.lock().unwrap(),
            ["/Users/me/apps (1)", "/Users/me/apps (1)/web"]
        );
        // Two files of 10 bytes; the link is skipped. The fake reports each file as 1 of 1.
        assert_eq!(progress.0.lock().unwrap().last(), Some(&(20, 20)));
        assert!(progress.0.lock().unwrap().contains(&(11, 20)));
    }

    #[tokio::test]
    async fn uploads_a_folder_making_its_subfolders_first() {
        let remote = FakeRemoteFiles::with(&[("/srv/site", EntryKind::Folder)]);
        let local = FakeLocalFiles::with(&[
            ("/Users/me/site", EntryKind::Folder),
            ("/Users/me/site/index.html", EntryKind::File),
            ("/Users/me/site/css", EntryKind::Folder),
            ("/Users/me/site/css/main.css", EntryKind::File),
        ]);
        let (transfers, session_id) = transfers(remote.clone(), local);

        let landed = transfers
            .upload(
                session_id,
                Path::new("/Users/me/site"),
                "/srv",
                Arc::new(IgnoreProgress),
            )
            .await
            .unwrap();

        assert_eq!(landed, "/srv/site (1)");
        assert_eq!(
            *remote.calls.lock().unwrap(),
            [
                "make_folder /srv/site (1)",
                "make_folder /srv/site (1)/css",
                "upload /Users/me/site/index.html /srv/site (1)/index.html",
                "upload /Users/me/site/css/main.css /srv/site (1)/css/main.css",
            ]
        );
    }
}
