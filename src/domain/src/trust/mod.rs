//! Whether a server is who it claims to be.

mod host_key;
mod host_key_status;
mod known_host_entry;

pub use host_key::HostKey;
pub use host_key_status::HostKeyStatus;
pub use known_host_entry::KnownHostEntry;
