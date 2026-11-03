use std::path::PathBuf;
use std::sync::{Mutex, PoisonError};

use sallygate_application::snippets::{SnippetRepository, SnippetStoreError};
use sallygate_domain::snippets::{Snippet, SnippetCommand, SnippetId, SnippetName};
use serde::{Deserialize, Serialize};

use crate::json_file;

/// The file format version; bump it and migrate when the shape changes.
const FORMAT_VERSION: u32 = 1;

/// Saved snippets in one JSON file (`snippets.json` in the app data folder). Before the file
/// exists, the list is `starters`; the first change saves them with it, so a deleted starter
/// stays deleted.
pub struct JsonSnippetRepository {
    path: PathBuf,
    starters: Vec<Snippet>,
    // Serializes read-modify-write cycles so two saves cannot lose each other's change.
    write_lock: Mutex<()>,
}

impl JsonSnippetRepository {
    #[must_use]
    pub fn new(path: PathBuf, starters: Vec<Snippet>) -> Self {
        Self {
            path,
            starters,
            write_lock: Mutex::new(()),
        }
    }

    fn read_file(&self) -> Result<Vec<Snippet>, SnippetStoreError> {
        let Some(file) =
            json_file::read::<SnippetsFile>(&self.path).map_err(SnippetStoreError::Read)?
        else {
            return Ok(self.starters.clone());
        };
        file.snippets
            .into_iter()
            .map(SnippetRecord::into_snippet)
            .collect()
    }

    fn write_file(&self, snippets: &[Snippet]) -> Result<(), SnippetStoreError> {
        let file = SnippetsFile {
            version: FORMAT_VERSION,
            snippets: snippets.iter().map(SnippetRecord::from_snippet).collect(),
        };
        json_file::write_atomically(&self.path, &file).map_err(SnippetStoreError::Write)
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, ()> {
        self.write_lock
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }
}

impl SnippetRepository for JsonSnippetRepository {
    fn all(&self) -> Result<Vec<Snippet>, SnippetStoreError> {
        let _guard = self.lock();
        self.read_file()
    }

    fn save(&self, snippet: &Snippet) -> Result<(), SnippetStoreError> {
        let _guard = self.lock();
        let mut snippets = self.read_file()?;
        match snippets.iter_mut().find(|saved| saved.id == snippet.id) {
            Some(saved) => *saved = snippet.clone(),
            None => snippets.push(snippet.clone()),
        }
        self.write_file(&snippets)
    }

    fn delete(&self, snippet_id: SnippetId) -> Result<(), SnippetStoreError> {
        let _guard = self.lock();
        let mut snippets = self.read_file()?;
        snippets.retain(|snippet| snippet.id != snippet_id);
        self.write_file(&snippets)
    }
}

#[derive(Serialize, Deserialize)]
struct SnippetsFile {
    version: u32,
    snippets: Vec<SnippetRecord>,
}

#[derive(Serialize, Deserialize)]
struct SnippetRecord {
    id: String,
    name: String,
    command: String,
}

impl SnippetRecord {
    fn from_snippet(snippet: &Snippet) -> Self {
        Self {
            id: snippet.id.to_string(),
            name: snippet.name.as_str().to_owned(),
            command: snippet.command.as_str().to_owned(),
        }
    }

    /// A saved snippet goes through the same validation as a new one.
    fn into_snippet(self) -> Result<Snippet, SnippetStoreError> {
        let invalid = |error: &dyn ToString| {
            SnippetStoreError::Read(format!("snippet \"{}\": {}", self.name, error.to_string()))
        };
        Ok(Snippet {
            id: self.id.parse().map_err(|error| invalid(&error))?,
            name: SnippetName::parse(&self.name).map_err(|error| invalid(&error))?,
            command: SnippetCommand::parse(&self.command).map_err(|error| invalid(&error))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snippet(name: &str, command: &str) -> Snippet {
        Snippet {
            id: SnippetId::new(),
            name: SnippetName::parse(name).unwrap(),
            command: SnippetCommand::parse(command).unwrap(),
        }
    }

    #[test]
    fn keeps_snippets_across_instances() {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("snippets.json");
        let logs = snippet("Logs", "journalctl -fu nginx");
        let disk = snippet("Disk", "df -h\ndu -sh /var/log");
        let repository = JsonSnippetRepository::new(path.clone(), Vec::new());
        repository.save(&logs).unwrap();
        repository.save(&disk).unwrap();
        repository.delete(logs.id).unwrap();

        assert_eq!(
            JsonSnippetRepository::new(path, Vec::new()).all().unwrap(),
            vec![disk]
        );
    }

    #[test]
    fn starts_with_the_starters_and_keeps_a_deleted_one_deleted() {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("snippets.json");
        let disk = snippet("Disk", "df -h");
        let memory = snippet("Memory", "free -h");
        let repository = JsonSnippetRepository::new(path.clone(), vec![disk.clone(), memory]);
        assert_eq!(repository.all().unwrap().len(), 2);

        repository.delete(disk.id).unwrap();
        repository.delete(repository.all().unwrap()[0].id).unwrap();

        let reopened = JsonSnippetRepository::new(path, vec![disk]);
        assert_eq!(reopened.all().unwrap(), Vec::new());
    }
}
