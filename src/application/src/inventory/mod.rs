//! Use cases and ports for saved hosts.

mod delete_host;
mod host_repository;
mod list_hosts;
mod save_host;

#[cfg(test)]
mod test_doubles;

pub use delete_host::DeleteHost;
pub use host_repository::{HostRepository, HostStoreError};
pub use list_hosts::ListHosts;
pub use save_host::SaveHost;
