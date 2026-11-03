use thiserror::Error;

/// Why a value in the `snippets` context was rejected.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SnippetsError {
    #[error("\"{0}\" is not a snippet id")]
    InvalidSnippetId(String),
    #[error("the snippet name is empty")]
    EmptyName,
    #[error("the snippet name is longer than {max} characters")]
    NameTooLong { max: usize },
    #[error("the command is empty")]
    EmptyCommand,
    #[error("the command is longer than {max} characters")]
    CommandTooLong { max: usize },
}
