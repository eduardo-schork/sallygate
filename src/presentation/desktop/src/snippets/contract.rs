use sallygate_domain::snippets::{Snippet, SnippetCommand, SnippetId, SnippetName};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::command_error::CommandError;

/// A saved snippet as the UI shows it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TSnippet",
    export,
    export_to = "../../ui/snippets/snippets-contract.ts"
)]
pub struct SnippetDto {
    pub id: String,
    pub name: String,
    pub command: String,
}

impl From<&Snippet> for SnippetDto {
    fn from(snippet: &Snippet) -> Self {
        Self {
            id: snippet.id.to_string(),
            name: snippet.name.as_str().to_owned(),
            command: snippet.command.as_str().to_owned(),
        }
    }
}

/// A new snippet (no `id`) or the changes to a saved one.
#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TSaveSnippetRequest",
    export,
    export_to = "../../ui/snippets/snippets-contract.ts"
)]
pub struct SaveSnippetRequest {
    #[ts(optional)]
    pub id: Option<String>,
    pub name: String,
    pub command: String,
}

impl SaveSnippetRequest {
    pub fn into_domain(self) -> Result<Snippet, CommandError> {
        let invalid = |error: &dyn ToString| CommandError::invalid_input(&error.to_string());
        Ok(Snippet {
            id: match self.id.as_deref() {
                Some(id) => id.parse::<SnippetId>().map_err(|error| invalid(&error))?,
                None => SnippetId::new(),
            },
            name: SnippetName::parse(&self.name).map_err(|error| invalid(&error))?,
            command: SnippetCommand::parse(&self.command).map_err(|error| invalid(&error))?,
        })
    }
}
