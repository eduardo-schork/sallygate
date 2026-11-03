//! Use cases and ports for browsing files on this computer and on a server, and copying them
//! between the two (SFTP).

mod browse_local_files;
mod browse_remote_files;
mod folder_listing;
mod local_files;
mod open_file_systems;
mod remote_files;
mod transfer_error;
mod transfer_files;

#[cfg(test)]
mod test_doubles;

pub use browse_local_files::BrowseLocalFiles;
pub use browse_remote_files::BrowseRemoteFiles;
pub use folder_listing::FolderListing;
pub use local_files::{LocalFiles, LocalFilesError};
pub use open_file_systems::OpenFileSystems;
pub use remote_files::{RemoteFiles, RemoteFilesError, TransferProgress};
pub use transfer_error::TransferError;
pub use transfer_files::TransferFiles;
