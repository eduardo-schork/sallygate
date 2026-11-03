use std::sync::Arc;

use sallygate_domain::sessions::SessionId;
use sallygate_domain::transfer::{EntryKind, file_name, join_remote_path};

use super::{FolderListing, OpenFileSystems, TransferError};

/// Use case: look around the server's files and tidy them.
pub struct BrowseRemoteFiles {
    file_systems: Arc<OpenFileSystems>,
}

impl BrowseRemoteFiles {
    #[must_use]
    pub fn new(file_systems: Arc<OpenFileSystems>) -> Self {
        Self { file_systems }
    }

    /// Lists `folder`, or the login folder when `None`.
    ///
    /// # Errors
    ///
    /// Returns an error when the session is closed or the folder cannot be read.
    pub async fn list(
        &self,
        session_id: SessionId,
        folder: Option<&str>,
    ) -> Result<FolderListing, TransferError> {
        let files = self.file_systems.of(session_id).await?;
        let path = match folder {
            Some(folder) => folder.to_owned(),
            None => files.home().await?,
        };
        let entries = files.list(&path).await?;
        Ok(FolderListing::new(path, entries))
    }

    /// # Errors
    ///
    /// Returns an error when the session is closed or the folder cannot be made.
    pub async fn make_folder(
        &self,
        session_id: SessionId,
        parent: &str,
        name: &str,
    ) -> Result<(), TransferError> {
        let files = self.file_systems.of(session_id).await?;
        Ok(files.make_folder(&join_remote_path(parent, name)).await?)
    }

    /// Gives an entry a new name in the same folder.
    ///
    /// # Errors
    ///
    /// Returns an error when the session is closed or the server refuses.
    pub async fn rename(
        &self,
        session_id: SessionId,
        path: &str,
        new_name: &str,
    ) -> Result<(), TransferError> {
        let files = self.file_systems.of(session_id).await?;
        let folder = path.strip_suffix(file_name(path)).unwrap_or("/");
        Ok(files
            .rename(path, &join_remote_path(folder, new_name))
            .await?)
    }

    /// # Errors
    ///
    /// Returns an error when the session is closed or the server refuses (e.g. a folder that is
    /// not empty).
    pub async fn delete(
        &self,
        session_id: SessionId,
        path: &str,
        kind: EntryKind,
    ) -> Result<(), TransferError> {
        let files = self.file_systems.of(session_id).await?;
        Ok(files.remove(path, kind).await?)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;

    use super::*;
    use crate::sessions::SessionCommandError;
    use crate::transfer::test_doubles::{FakeRemoteFiles, HOME, open_session};

    fn browse(
        files: Arc<FakeRemoteFiles>,
    ) -> (
        BrowseRemoteFiles,
        SessionId,
        Arc<crate::transfer::test_doubles::FilesSession>,
        Arc<crate::sessions::SessionRegistry>,
    ) {
        let (registry, session_id, session) = open_session(files);
        let browser = BrowseRemoteFiles::new(Arc::new(OpenFileSystems::new(Arc::clone(&registry))));
        (browser, session_id, session, registry)
    }

    #[tokio::test]
    async fn starts_in_the_login_folder_with_folders_first() {
        let files = FakeRemoteFiles::with(&[
            ("/home/me/zeta.txt", EntryKind::File),
            ("/home/me/Apps", EntryKind::Folder),
            ("/home/me/alpha.txt", EntryKind::File),
            ("/home/me/Apps/inside.txt", EntryKind::File),
        ]);
        let (browser, session_id, _, _) = browse(files);

        let folder = browser.list(session_id, None).await.unwrap();

        assert_eq!(folder.path, HOME);
        let names: Vec<_> = folder
            .entries
            .iter()
            .map(|entry| entry.name.as_str())
            .collect();
        assert_eq!(names, ["Apps", "alpha.txt", "zeta.txt"]);
    }

    #[tokio::test]
    async fn opens_the_files_of_a_session_once() {
        let (browser, session_id, session, _) = browse(FakeRemoteFiles::with(&[]));

        browser.list(session_id, None).await.unwrap();
        browser.list(session_id, Some("/tmp")).await.unwrap();

        assert_eq!(session.opens.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn fails_once_the_session_is_closed() {
        let (browser, session_id, _, registry) = browse(FakeRemoteFiles::with(&[]));
        browser.list(session_id, None).await.unwrap();
        registry.remove(session_id);

        let error = browser.list(session_id, None).await.unwrap_err();

        assert_eq!(
            error,
            TransferError::Session(SessionCommandError::NotFound(session_id))
        );
    }

    #[tokio::test]
    async fn renames_inside_the_same_folder_and_makes_folders_by_name() {
        let files = FakeRemoteFiles::with(&[]);
        let (browser, session_id, _, _) = browse(files.clone());

        browser
            .rename(session_id, "/srv/old.txt", "new.txt")
            .await
            .unwrap();
        browser
            .make_folder(session_id, "/srv", "logs")
            .await
            .unwrap();
        browser
            .delete(session_id, "/srv/logs", EntryKind::Folder)
            .await
            .unwrap();

        assert_eq!(
            *files.calls.lock().unwrap(),
            [
                "rename /srv/old.txt /srv/new.txt",
                "make_folder /srv/logs",
                "remove /srv/logs Folder",
            ]
        );
    }
}
