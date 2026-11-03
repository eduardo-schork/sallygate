//! Adapters for a server's files.

mod local_file_system;
mod sftp_remote_files;

pub use local_file_system::LocalFileSystem;
pub use sftp_remote_files::SftpRemoteFiles;
