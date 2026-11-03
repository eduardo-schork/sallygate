use thiserror::Error;

use super::RemoteFilesError;
use crate::sessions::{LiveSessionError, SessionCommandError};

/// Why a file browser request failed. Messages are safe to show the user.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TransferError {
    #[error(transparent)]
    Session(#[from] SessionCommandError),
    #[error("could not open the server's files: {0}")]
    Unavailable(#[from] LiveSessionError),
    #[error(transparent)]
    Remote(#[from] RemoteFilesError),
}
