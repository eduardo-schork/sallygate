/// What `known_hosts` says about the key a server presented.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostKeyStatus {
    /// The key matches the one already trusted for this server.
    Trusted,
    /// The server has never been trusted. The user decides.
    Unknown,
    /// The server is trusted with a different key: possibly an attack.
    Changed,
}
