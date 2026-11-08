//! Files on this computer and on a server, as the file browser shows and copies them.

mod file_entry;
mod free_name;

pub use file_entry::{EntryKind, FileEntry, file_name, join_remote_path};
pub use free_name::free_name;
