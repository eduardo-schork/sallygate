use thiserror::Error;

/// Why a value in the `sessions` context was rejected.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SessionsError {
    #[error("\"{0}\" is not a session id")]
    InvalidSessionId(String),
    #[error("a terminal needs at least one column and one row")]
    EmptyTerminalSize,
}
