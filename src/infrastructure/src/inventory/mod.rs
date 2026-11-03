//! Adapters for saved hosts.

mod json_host_repository;
mod ssh_config_file;

pub use json_host_repository::JsonHostRepository;
pub use ssh_config_file::SshConfigFile;
