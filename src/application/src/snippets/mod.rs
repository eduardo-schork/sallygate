//! Use cases and the port for saved snippets.

mod manage_snippets;
mod snippet_repository;

pub use manage_snippets::ManageSnippets;
pub use snippet_repository::{SnippetRepository, SnippetStoreError};
