//! Use cases and ports for browsing a server's files and copying them (SFTP).

mod browse_remote_files;
mod open_file_systems;
mod remote_files;
mod transfer_error;
mod transfer_files;

#[cfg(test)]
mod test_doubles;

pub use browse_remote_files::{BrowseRemoteFiles, RemoteFolder};
pub use open_file_systems::OpenFileSystems;
pub use remote_files::{RemoteFiles, RemoteFilesError, TransferProgress};
pub use transfer_error::TransferError;
pub use transfer_files::TransferFiles;
