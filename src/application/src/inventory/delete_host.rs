use std::sync::Arc;

use sallygate_domain::inventory::HostId;

use super::{HostRepository, HostStoreError};

/// Use case: forget a saved host. Open sessions to it stay open.
pub struct DeleteHost {
    repository: Arc<dyn HostRepository>,
}

impl DeleteHost {
    #[must_use]
    pub fn new(repository: Arc<dyn HostRepository>) -> Self {
        Self { repository }
    }

    /// # Errors
    ///
    /// Returns an error when the hosts cannot be written.
    pub fn execute(&self, host_id: HostId) -> Result<(), HostStoreError> {
        self.repository.delete(host_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::test_doubles::{InMemoryHosts, host};

    #[test]
    fn removes_only_the_chosen_host() {
        let kept = host("web", None);
        let removed = host("db", None);
        let repository = InMemoryHosts::with(vec![kept.clone(), removed.clone()]);

        DeleteHost::new(repository.clone())
            .execute(removed.id())
            .unwrap();

        assert_eq!(repository.all().unwrap(), vec![kept]);
    }
}
