use async_trait::async_trait;
use sallygate_domain::sessions::TerminalSize;
use thiserror::Error;

/// Port: one open terminal on a remote server.
#[async_trait]
pub trait LiveSession: Send + Sync {
    /// Sends keystrokes or pasted text to the remote shell.
    async fn write(&self, bytes: &[u8]) -> Result<(), LiveSessionError>;

    /// Tells the remote shell the terminal changed size.
    async fn resize(&self, size: TerminalSize) -> Result<(), LiveSessionError>;

    /// Closes the session and its connection.
    async fn close(&self) -> Result<(), LiveSessionError>;
}

/// A live session could not carry out a request; the text is safe to show the user.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("the session is no longer available: {0}")]
pub struct LiveSessionError(pub String);
