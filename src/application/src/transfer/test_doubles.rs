//! An in-memory server file system and a session that serves it, for use case tests.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use sallygate_domain::sessions::{SessionId, TerminalSize};
use sallygate_domain::transfer::{EntryKind, RemoteEntry, file_name};

use super::{RemoteFiles, RemoteFilesError, TransferProgress};
use crate::sessions::{LiveSession, LiveSessionError, SessionRegistry};

pub const HOME: &str = "/home/me";

/// Remote entries by path, plus a log of the calls that change them.
#[derive(Default)]
pub struct FakeRemoteFiles {
    pub entries: Mutex<Vec<RemoteEntry>>,
    pub calls: Mutex<Vec<String>>,
}

impl FakeRemoteFiles {
    pub fn with(entries: &[(&str, EntryKind)]) -> Arc<Self> {
        let files = Self::default();
        *files.entries.lock().unwrap() = entries
            .iter()
            .map(|(path, kind)| RemoteEntry {
                name: file_name(path).to_owned(),
                path: (*path).to_owned(),
                kind: *kind,
                size_bytes: 0,
                modified_at: None,
            })
            .collect();
        Arc::new(files)
    }

    fn log(&self, call: String) {
        self.calls.lock().unwrap().push(call);
    }
}

#[async_trait]
impl RemoteFiles for FakeRemoteFiles {
    async fn home(&self) -> Result<String, RemoteFilesError> {
        Ok(HOME.to_owned())
    }

    async fn list(&self, folder: &str) -> Result<Vec<RemoteEntry>, RemoteFilesError> {
        let prefix = format!("{}/", folder.trim_end_matches('/'));
        Ok(self
            .entries
            .lock()
            .unwrap()
            .iter()
            .filter(|entry| {
                entry
                    .path
                    .strip_prefix(&prefix)
                    .is_some_and(|rest| !rest.contains('/'))
            })
            .cloned()
            .collect())
    }

    async fn make_folder(&self, path: &str) -> Result<(), RemoteFilesError> {
        self.log(format!("make_folder {path}"));
        Ok(())
    }

    async fn rename(&self, from: &str, to: &str) -> Result<(), RemoteFilesError> {
        self.log(format!("rename {from} {to}"));
        Ok(())
    }

    async fn remove(&self, path: &str, kind: EntryKind) -> Result<(), RemoteFilesError> {
        self.log(format!("remove {path} {kind:?}"));
        Ok(())
    }

    async fn download(
        &self,
        remote_path: &str,
        local_folder: &Path,
        progress: Arc<dyn TransferProgress>,
    ) -> Result<PathBuf, RemoteFilesError> {
        progress.advanced(1, 1);
        Ok(local_folder.join(file_name(remote_path)))
    }

    async fn upload(
        &self,
        local_path: &Path,
        remote_path: &str,
        progress: Arc<dyn TransferProgress>,
    ) -> Result<(), RemoteFilesError> {
        progress.advanced(1, 1);
        self.log(format!("upload {} {remote_path}", local_path.display()));
        Ok(())
    }
}

/// A session whose only job is to hand out its files, counting how often.
pub struct FilesSession {
    pub files: Arc<FakeRemoteFiles>,
    pub opens: AtomicUsize,
}

#[async_trait]
impl LiveSession for FilesSession {
    async fn write(&self, _bytes: &[u8]) -> Result<(), LiveSessionError> {
        Ok(())
    }

    async fn resize(&self, _size: TerminalSize) -> Result<(), LiveSessionError> {
        Ok(())
    }

    async fn open_files(&self) -> Result<Arc<dyn RemoteFiles>, LiveSessionError> {
        self.opens.fetch_add(1, Ordering::SeqCst);
        Ok(self.files.clone())
    }

    async fn close(&self) -> Result<(), LiveSessionError> {
        Ok(())
    }
}

/// A registry with one open session serving `files`.
pub fn open_session(
    files: Arc<FakeRemoteFiles>,
) -> (Arc<SessionRegistry>, SessionId, Arc<FilesSession>) {
    let registry = Arc::new(SessionRegistry::new());
    let session_id = SessionId::new();
    let session = Arc::new(FilesSession {
        files,
        opens: AtomicUsize::new(0),
    });
    registry.add(session_id, session.clone());
    (registry, session_id, session)
}

pub struct IgnoreProgress;

impl TransferProgress for IgnoreProgress {
    fn advanced(&self, _done_bytes: u64, _total_bytes: u64) {}
}
