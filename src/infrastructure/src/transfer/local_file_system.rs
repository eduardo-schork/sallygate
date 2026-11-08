use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use sallygate_application::transfer::{LocalFiles, LocalFilesError};
use sallygate_domain::transfer::{EntryKind, FileEntry};

/// What an entry is, without following a link.
fn entry_kind(metadata: &Metadata) -> EntryKind {
    if metadata.is_symlink() {
        EntryKind::Link
    } else if metadata.is_dir() {
        EntryKind::Folder
    } else {
        EntryKind::File
    }
}

/// This computer's files, read with the standard library.
pub struct LocalFileSystem {
    home: PathBuf,
}

impl LocalFileSystem {
    #[must_use]
    pub fn new(home: PathBuf) -> Self {
        Self { home }
    }
}

impl LocalFiles for LocalFileSystem {
    fn home(&self) -> String {
        self.home.to_string_lossy().into_owned()
    }

    fn list(&self, folder: &str) -> Result<Vec<FileEntry>, LocalFilesError> {
        let entries =
            std::fs::read_dir(folder).map_err(|error| LocalFilesError(error.to_string()))?;
        Ok(entries
            .filter_map(Result::ok)
            .filter_map(|entry| {
                // Not followed: a link shows as a link, like on the server side.
                let metadata = entry.path().symlink_metadata().ok()?;
                let kind = entry_kind(&metadata);
                Some(FileEntry {
                    name: entry.file_name().to_string_lossy().into_owned(),
                    path: entry.path().to_string_lossy().into_owned(),
                    kind,
                    size_bytes: metadata.len(),
                    modified_at: metadata
                        .modified()
                        .ok()
                        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
                        .map(|age| age.as_secs()),
                    permissions: Some(metadata.permissions().mode() & 0o7777),
                })
            })
            .collect())
    }

    fn kind(&self, path: &Path) -> Result<EntryKind, LocalFilesError> {
        let metadata = path
            .symlink_metadata()
            .map_err(|error| LocalFilesError(error.to_string()))?;
        Ok(entry_kind(&metadata))
    }

    fn make_folder(&self, path: &Path) -> Result<(), LocalFilesError> {
        std::fs::create_dir(path).map_err(|error| LocalFilesError(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_files_folders_and_links_with_their_details() {
        let folder = tempfile::tempdir().unwrap();
        std::fs::write(folder.path().join("notes.txt"), "hello").unwrap();
        std::fs::create_dir(folder.path().join("apps")).unwrap();
        std::os::unix::fs::symlink("notes.txt", folder.path().join("current")).unwrap();
        let files = LocalFileSystem::new(folder.path().to_path_buf());

        let mut entries = files.list(&files.home()).unwrap();
        entries.sort_by(|left, right| left.name.cmp(&right.name));

        let kinds: Vec<_> = entries
            .iter()
            .map(|entry| (entry.name.as_str(), entry.kind))
            .collect();
        assert_eq!(
            kinds,
            [
                ("apps", EntryKind::Folder),
                ("current", EntryKind::Link),
                ("notes.txt", EntryKind::File)
            ]
        );
        assert_eq!(entries[2].size_bytes, 5);
        assert!(entries[2].permissions.is_some());
    }

    #[test]
    fn reports_a_folder_it_cannot_read() {
        let files = LocalFileSystem::new(PathBuf::from("/"));
        assert!(files.list("/no/such/folder").is_err());
    }

    #[test]
    fn makes_a_folder_and_tells_kinds_apart() {
        let folder = tempfile::tempdir().unwrap();
        let files = LocalFileSystem::new(folder.path().to_path_buf());
        let made = folder.path().join("apps");

        files.make_folder(&made).unwrap();
        std::fs::write(folder.path().join("a.txt"), "").unwrap();

        assert_eq!(files.kind(&made).unwrap(), EntryKind::Folder);
        assert_eq!(
            files.kind(&folder.path().join("a.txt")).unwrap(),
            EntryKind::File
        );
        assert!(
            files.make_folder(&made).is_err(),
            "an existing folder is not made again"
        );
    }
}
