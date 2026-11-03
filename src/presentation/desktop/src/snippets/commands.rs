#![allow(
    clippy::needless_pass_by_value,
    reason = "Tauri commands receive their arguments by value"
)]

use sallygate_application::snippets::SnippetStoreError;
use sallygate_domain::snippets::SnippetId;
use tauri::State;

use super::contract::{SaveSnippetRequest, SnippetDto};
use crate::command_error::{CommandError, CommandErrorKind};
use crate::composition::AppServices;

fn store_error(error: &SnippetStoreError) -> CommandError {
    CommandError::new(CommandErrorKind::SnippetStoreUnavailable, error)
}

/// The saved snippets, by name.
#[tauri::command]
pub fn list_snippets(services: State<'_, AppServices>) -> Result<Vec<SnippetDto>, CommandError> {
    let snippets = services
        .snippets
        .list()
        .map_err(|error| store_error(&error))?;
    Ok(snippets.iter().map(SnippetDto::from).collect())
}

/// Saves a new snippet or the changes to one, and returns it as saved.
#[tauri::command]
pub fn save_snippet(
    services: State<'_, AppServices>,
    request: SaveSnippetRequest,
) -> Result<SnippetDto, CommandError> {
    let snippet = request.into_domain()?;
    services
        .snippets
        .save(&snippet)
        .map_err(|error| store_error(&error))?;
    Ok(SnippetDto::from(&snippet))
}

/// Forgets a saved snippet.
#[tauri::command]
pub fn delete_snippet(
    services: State<'_, AppServices>,
    snippet_id: String,
) -> Result<(), CommandError> {
    let snippet_id = snippet_id
        .parse::<SnippetId>()
        .map_err(|error| CommandError::invalid_input(&error))?;
    services
        .snippets
        .delete(snippet_id)
        .map_err(|error| store_error(&error))
}
