use std::sync::Arc;

use sallygate_domain::snippets::{Snippet, SnippetId};

use super::{SnippetRepository, SnippetStoreError};

/// Use case: list, save and delete snippets. Sending one to a terminal is a session write.
pub struct ManageSnippets {
    repository: Arc<dyn SnippetRepository>,
}

impl ManageSnippets {
    #[must_use]
    pub fn new(repository: Arc<dyn SnippetRepository>) -> Self {
        Self { repository }
    }

    /// Every snippet, sorted by name (case-insensitive).
    ///
    /// # Errors
    ///
    /// Returns an error when the snippets cannot be read.
    pub fn list(&self) -> Result<Vec<Snippet>, SnippetStoreError> {
        let mut snippets = self.repository.all()?;
        snippets.sort_by_cached_key(|snippet| snippet.name.as_str().to_lowercase());
        Ok(snippets)
    }

    /// # Errors
    ///
    /// Returns an error when the snippet cannot be written.
    pub fn save(&self, snippet: &Snippet) -> Result<(), SnippetStoreError> {
        self.repository.save(snippet)
    }

    /// # Errors
    ///
    /// Returns an error when the snippets cannot be written.
    pub fn delete(&self, snippet_id: SnippetId) -> Result<(), SnippetStoreError> {
        self.repository.delete(snippet_id)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use sallygate_domain::snippets::{SnippetCommand, SnippetName};

    use super::*;

    #[derive(Default)]
    struct InMemorySnippets(Mutex<Vec<Snippet>>);

    impl SnippetRepository for InMemorySnippets {
        fn all(&self) -> Result<Vec<Snippet>, SnippetStoreError> {
            Ok(self.0.lock().unwrap().clone())
        }

        fn save(&self, snippet: &Snippet) -> Result<(), SnippetStoreError> {
            let mut snippets = self.0.lock().unwrap();
            snippets.retain(|saved| saved.id != snippet.id);
            snippets.push(snippet.clone());
            Ok(())
        }

        fn delete(&self, snippet_id: SnippetId) -> Result<(), SnippetStoreError> {
            self.0
                .lock()
                .unwrap()
                .retain(|saved| saved.id != snippet_id);
            Ok(())
        }
    }

    fn snippet(name: &str) -> Snippet {
        Snippet {
            id: SnippetId::new(),
            name: SnippetName::parse(name).unwrap(),
            command: SnippetCommand::parse("uptime").unwrap(),
        }
    }

    #[test]
    fn lists_snippets_by_name_and_forgets_deleted_ones() {
        let snippets = ManageSnippets::new(Arc::new(InMemorySnippets::default()));
        let logs = snippet("logs");
        snippets.save(&snippet("Disk")).unwrap();
        snippets.save(&logs).unwrap();
        snippets.save(&snippet("apt upgrade")).unwrap();

        snippets.delete(logs.id).unwrap();

        let names: Vec<_> = snippets
            .list()
            .unwrap()
            .iter()
            .map(|snippet| snippet.name.as_str().to_owned())
            .collect();
        assert_eq!(names, ["apt upgrade", "Disk"]);
    }
}
