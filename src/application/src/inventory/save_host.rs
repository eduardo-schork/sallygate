use std::sync::Arc;

use sallygate_domain::inventory::Host;

use super::{HostRepository, HostStoreError};

/// Use case: save a new host, or the changes to an existing one.
pub struct SaveHost {
    repository: Arc<dyn HostRepository>,
}

impl SaveHost {
    #[must_use]
    pub fn new(repository: Arc<dyn HostRepository>) -> Self {
        Self { repository }
    }

    /// # Errors
    ///
    /// Returns an error when the host cannot be written.
    pub fn execute(&self, host: &Host) -> Result<(), HostStoreError> {
        self.repository.save(host)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::test_doubles::{InMemoryHosts, host};

    #[test]
    fn adds_a_new_host() {
        let repository = InMemoryHosts::with(vec![]);
        let new_host = host("web", None);

        SaveHost::new(repository.clone())
            .execute(&new_host)
            .unwrap();

        assert_eq!(repository.all().unwrap(), vec![new_host]);
    }

    #[test]
    fn replaces_the_host_with_the_same_id() {
        let original = host("web", None);
        let repository = InMemoryHosts::with(vec![original.clone()]);
        let renamed = Host::new(
            original.id(),
            sallygate_domain::inventory::HostLabel::parse("web-01").unwrap(),
            original.endpoint().clone(),
            original.auth().clone(),
            None,
        );

        SaveHost::new(repository.clone()).execute(&renamed).unwrap();

        assert_eq!(repository.all().unwrap(), vec![renamed]);
    }
}
