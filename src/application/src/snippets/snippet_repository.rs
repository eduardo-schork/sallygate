use sallygate_domain::snippets::{Snippet, SnippetId};
use thiserror::Error;

/// Port: where saved snippets are kept.
pub trait SnippetRepository: Send + Sync {
    /// # Errors
    ///
    /// Returns an error when the snippets cannot be read.
    fn all(&self) -> Result<Vec<Snippet>, SnippetStoreError>;

    /// Adds the snippet, or replaces the one with the same id.
    ///
    /// # Errors
    ///
    /// Returns an error when the snippets cannot be written.
    fn save(&self, snippet: &Snippet) -> Result<(), SnippetStoreError>;

    /// Removing a snippet that is not there is not an error.
    ///
    /// # Errors
    ///
    /// Returns an error when the snippets cannot be written.
    fn delete(&self, snippet_id: SnippetId) -> Result<(), SnippetStoreError>;
}

/// Why snippets could not be read or written. Messages are safe to show the user.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SnippetStoreError {
    #[error("could not read the snippets: {0}")]
    Read(String),
    #[error("could not save the snippets: {0}")]
    Write(String),
}
