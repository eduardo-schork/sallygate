use sallygate_domain::transfer::FileEntry;
use thiserror::Error;

/// Port: the files on this computer, read only, for the file browser's local pane.
pub trait LocalFiles: Send + Sync {
    /// The user's home folder.
    fn home(&self) -> String;

    /// The entries of a folder, in no particular order.
    ///
    /// # Errors
    ///
    /// Returns an error when the folder cannot be read.
    fn list(&self, folder: &str) -> Result<Vec<FileEntry>, LocalFilesError>;
}

/// A local folder could not be read; the text is safe to show the user.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("could not read the folder: {0}")]
pub struct LocalFilesError(pub String);
