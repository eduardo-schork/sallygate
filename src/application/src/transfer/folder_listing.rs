use sallygate_domain::transfer::{EntryKind, FileEntry};

/// A folder and what is in it, on this computer or on a server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderListing {
    pub path: String,
    /// Folders first, then by name (case-insensitive).
    pub entries: Vec<FileEntry>,
}

impl FolderListing {
    #[must_use]
    pub fn new(path: String, mut entries: Vec<FileEntry>) -> Self {
        entries.sort_by_cached_key(|entry| {
            (entry.kind != EntryKind::Folder, entry.name.to_lowercase())
        });
        Self { path, entries }
    }
}
