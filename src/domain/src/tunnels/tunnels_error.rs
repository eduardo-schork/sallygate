use thiserror::Error;

/// Why a value in the `tunnels` context was rejected.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TunnelsError {
    #[error("\"{0}\" is not a rule id")]
    InvalidTunnelId(String),
    #[error("the rule name is empty")]
    EmptyName,
    #[error("the rule name is longer than {max} characters")]
    NameTooLong { max: usize },
}
