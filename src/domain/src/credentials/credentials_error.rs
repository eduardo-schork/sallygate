use thiserror::Error;

/// Why a value in the `credentials` context was rejected.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CredentialsError {
    #[error("the private key path is empty")]
    EmptyKeyFilePath,
    #[error("\"{0}\" is not a key name: use letters, digits, ., _ or -, and no .pub ending")]
    InvalidKeyName(String),
}
