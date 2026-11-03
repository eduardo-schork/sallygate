//! Saved commands the user sends to a terminal with one click.

mod snippet;
mod snippets_error;

pub use snippet::{Snippet, SnippetCommand, SnippetId, SnippetName};
pub use snippets_error::SnippetsError;
