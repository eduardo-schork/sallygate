use thiserror::Error;

/// Why a value in the `inventory` context was rejected.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InventoryError {
    #[error("the host address is empty")]
    EmptyHostAddress,
    #[error("the host address \"{0}\" contains whitespace")]
    HostAddressWithWhitespace(String),
    #[error("the host address is longer than {max} characters")]
    HostAddressTooLong { max: usize },
    #[error("port 0 is not a valid SSH port")]
    ZeroPort,
    #[error("the username is empty")]
    EmptyUsername,
    #[error("the username \"{0}\" contains whitespace")]
    UsernameWithWhitespace(String),
    #[error("\"{0}\" is not a host id")]
    InvalidHostId(String),
    #[error("the host name is empty")]
    EmptyHostLabel,
    #[error("the host name is longer than {max} characters")]
    HostLabelTooLong { max: usize },
    #[error("the group name is empty")]
    EmptyGroupName,
    #[error("the group name is longer than {max} characters")]
    GroupNameTooLong { max: usize },
}
