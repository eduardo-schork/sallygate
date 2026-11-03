//! Use cases and ports for saved hosts.

mod delete_host;
mod host_repository;
mod import_ssh_config;
mod list_hosts;
mod save_host;
mod ssh_config_source;

#[cfg(test)]
pub(crate) mod test_doubles;

pub use delete_host::DeleteHost;
pub use host_repository::{HostRepository, HostStoreError};
pub use import_ssh_config::{ImportError, ImportReport, ImportSshConfig};
pub use list_hosts::ListHosts;
pub use save_host::SaveHost;
pub use ssh_config_source::{SshConfigEntry, SshConfigError, SshConfigSource};
