use std::path::Path;

use sallygate_domain::transfer::{EntryKind, FileEntry};
use thiserror::Error;

/// Port: the files on this computer, for the file browser's local pane and folder copies.
pub trait LocalFiles: Send + Sync {
    /// The user's home folder.
    fn home(&self) -> String;

    /// The entries of a folder, in no particular order.
    ///
    /// # Errors
    ///
    /// Returns an error when the folder cannot be read.
    fn list(&self, folder: &str) -> Result<Vec<FileEntry>, LocalFilesError>;

    /// What a path is, without following a link.
    ///
    /// # Errors
    ///
    /// Returns an error when the path cannot be read.
    fn kind(&self, path: &Path) -> Result<EntryKind, LocalFilesError>;

    /// Makes one folder; its parent must exist.
    ///
    /// # Errors
    ///
    /// Returns an error when the folder cannot be made.
    fn make_folder(&self, path: &Path) -> Result<(), LocalFilesError>;
}

/// A local file or folder could not be used; the text is safe to show the user.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("on this computer: {0}")]
pub struct LocalFilesError(pub String);
