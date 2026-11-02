//! Adapters for trusting servers.

mod known_hosts_file;
mod openssh_host_key;

pub use known_hosts_file::KnownHostsFile;
pub(crate) use openssh_host_key::host_key_from_public_key;
