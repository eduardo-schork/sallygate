use std::sync::Arc;

use sallygate_domain::inventory::Host;

use super::{HostRepository, HostStoreError};

/// Use case: show the saved hosts, grouped and in a stable order.
pub struct ListHosts {
    repository: Arc<dyn HostRepository>,
}

impl ListHosts {
    #[must_use]
    pub fn new(repository: Arc<dyn HostRepository>) -> Self {
        Self { repository }
    }

    /// Hosts sorted by group, then by name (case-insensitive). Hosts without a group come last.
    ///
    /// # Errors
    ///
    /// Returns an error when the hosts cannot be read.
    pub fn execute(&self) -> Result<Vec<Host>, HostStoreError> {
        let mut hosts = self.repository.all()?;
        hosts.sort_by_cached_key(|host| {
            (
                host.group().is_none(),
                host.group().map(|group| group.as_str().to_lowercase()),
                host.label().as_str().to_lowercase(),
            )
        });
        Ok(hosts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::test_doubles::{InMemoryHosts, host};

    #[test]
    fn sorts_by_group_then_name_with_ungrouped_hosts_last() {
        let repository = InMemoryHosts::with(vec![
            host("zeta", None),
            host("web", Some("Cloud")),
            host("nas", Some("home")),
            host("Api", Some("Cloud")),
        ]);

        let labels: Vec<String> = ListHosts::new(repository)
            .execute()
            .unwrap()
            .iter()
            .map(|host| host.label().to_string())
            .collect();

        assert_eq!(labels, vec!["Api", "web", "nas", "zeta"]);
    }
}
