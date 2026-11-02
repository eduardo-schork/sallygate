//! Use cases and ports for trusting servers.

mod known_hosts;
mod trust_host_key;

pub use known_hosts::{KnownHosts, KnownHostsError};
pub use trust_host_key::TrustHostKey;
