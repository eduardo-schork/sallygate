use super::HostKey;

/// One line of the trusted key list: which servers it names and the key it trusts for them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnownHostEntry {
    /// The line in the file, from 1. With `host_key`, it tells which line to forget.
    pub line: usize,
    /// The servers as written, e.g. `github.com` or `[127.0.0.1]:2222`; `None` when hashed.
    pub hosts: Option<String>,
    pub host_key: HostKey,
}
