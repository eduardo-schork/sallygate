use sallygate_domain::sessions::SessionId;
use thiserror::Error;

use super::LiveSessionError;

/// Why a request to an open session failed.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SessionCommandError {
    #[error("session {0} is not open")]
    NotFound(SessionId),
    #[error(transparent)]
    Session(#[from] LiveSessionError),
}
