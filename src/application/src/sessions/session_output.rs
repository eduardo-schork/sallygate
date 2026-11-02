use sallygate_domain::sessions::SessionEnd;

/// Port: where one session's output goes (the UI, in the app).
pub trait SessionOutput: Send + Sync {
    /// Bytes the remote shell wrote, exactly as received.
    fn data(&self, bytes: &[u8]);

    /// The session ended on its own. Called at most once.
    fn ended(&self, end: SessionEnd);
}
