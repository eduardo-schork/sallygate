//! Files on a remote server, as the file browser shows them.

mod remote_entry;

pub use remote_entry::{EntryKind, RemoteEntry, file_name, join_remote_path};
