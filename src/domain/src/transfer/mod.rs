//! Files on a remote server, as the file browser shows them.

mod file_entry;

pub use file_entry::{EntryKind, FileEntry, file_name, join_remote_path};
