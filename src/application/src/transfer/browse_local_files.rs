use std::sync::Arc;

use super::{FolderListing, LocalFiles, LocalFilesError};

/// Use case: look around this computer's files, to pick what to upload and where downloads go.
pub struct BrowseLocalFiles {
    files: Arc<dyn LocalFiles>,
}

impl BrowseLocalFiles {
    #[must_use]
    pub fn new(files: Arc<dyn LocalFiles>) -> Self {
        Self { files }
    }

    /// Lists `folder`, or the home folder when `None`.
    ///
    /// # Errors
    ///
    /// Returns an error when the folder cannot be read.
    pub fn list(&self, folder: Option<&str>) -> Result<FolderListing, LocalFilesError> {
        let path = folder.map_or_else(|| self.files.home(), ToOwned::to_owned);
        let entries = self.files.list(&path)?;
        Ok(FolderListing::new(path, entries))
    }
}
