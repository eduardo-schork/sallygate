use thiserror::Error;

/// Why a value in the `credentials` context was rejected.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CredentialsError {
    #[error("the private key path is empty")]
    EmptyKeyFilePath,
}
