/// How a session ended on its own (not by the user closing it).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionEnd {
    /// The remote shell exited with this status code.
    Exited(u32),
    /// The connection dropped; the text says why, safe to show the user.
    Disconnected(String),
}
