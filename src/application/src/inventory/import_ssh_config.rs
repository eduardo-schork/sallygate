use std::sync::Arc;

use sallygate_domain::credentials::{AuthMethod, KeyFilePath};
use sallygate_domain::inventory::{
    GroupName, Host, HostAddress, HostId, HostLabel, Port, SshEndpoint, Username,
};
use thiserror::Error;

use super::{HostRepository, HostStoreError, SshConfigEntry, SshConfigError, SshConfigSource};

/// The group imported hosts go into, so the user can find and tidy them.
const IMPORTED_GROUP: &str = "SSH config";

/// What an import did.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ImportReport {
    pub imported: Vec<Host>,
    /// Hosts already saved with the same address, port and username.
    pub already_saved: usize,
    /// Entries left out, with the reason, e.g. `("web", "the port must not be 0")`.
    pub invalid: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ImportError {
    #[error(transparent)]
    Config(#[from] SshConfigError),
    #[error(transparent)]
    Store(#[from] HostStoreError),
}

/// Use case: save the hosts from `~/.ssh/config` that are not saved yet. Running it again adds
/// only what is new.
pub struct ImportSshConfig {
    config: Arc<dyn SshConfigSource>,
    repository: Arc<dyn HostRepository>,
}

impl ImportSshConfig {
    #[must_use]
    pub fn new(config: Arc<dyn SshConfigSource>, repository: Arc<dyn HostRepository>) -> Self {
        Self { config, repository }
    }

    /// # Errors
    ///
    /// Returns an error when the config cannot be read or the hosts cannot be saved.
    pub fn execute(&self) -> Result<ImportReport, ImportError> {
        let mut known_endpoints: Vec<SshEndpoint> = self
            .repository
            .all()?
            .iter()
            .map(|host| host.endpoint().clone())
            .collect();
        let mut report = ImportReport::default();

        for entry in self.config.entries()? {
            let host = match to_host(&entry) {
                Ok(host) => host,
                Err(reason) => {
                    report.invalid.push((entry.alias, reason));
                    continue;
                }
            };
            if known_endpoints.contains(host.endpoint()) {
                report.already_saved += 1;
                continue;
            }
            self.repository.save(&host)?;
            known_endpoints.push(host.endpoint().clone());
            report.imported.push(host);
        }
        Ok(report)
    }
}

fn to_host(entry: &SshConfigEntry) -> Result<Host, String> {
    let text = |error: &dyn ToString| error.to_string();
    let endpoint = SshEndpoint::new(
        HostAddress::parse(&entry.address).map_err(|error| text(&error))?,
        Port::try_from(entry.port).map_err(|error| text(&error))?,
        Username::parse(&entry.username).map_err(|error| text(&error))?,
    );
    let auth = match &entry.key_file {
        Some(path) => AuthMethod::KeyFile(KeyFilePath::parse(path).map_err(|error| text(&error))?),
        None => AuthMethod::Password,
    };
    Ok(Host::new(
        HostId::new(),
        HostLabel::parse(&entry.alias).map_err(|error| text(&error))?,
        endpoint,
        auth,
        Some(GroupName::parse(IMPORTED_GROUP).map_err(|error| text(&error))?),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::test_doubles::{InMemoryHosts, host};

    struct FixedConfig(Vec<SshConfigEntry>);

    impl SshConfigSource for FixedConfig {
        fn entries(&self) -> Result<Vec<SshConfigEntry>, SshConfigError> {
            Ok(self.0.clone())
        }
    }

    fn entry(alias: &str, address: &str, port: u16, key_file: Option<&str>) -> SshConfigEntry {
        SshConfigEntry {
            alias: alias.to_owned(),
            address: address.to_owned(),
            port,
            username: "ubuntu".to_owned(),
            key_file: key_file.map(ToOwned::to_owned),
        }
    }

    fn import(entries: Vec<SshConfigEntry>, repository: &Arc<InMemoryHosts>) -> ImportReport {
        ImportSshConfig::new(Arc::new(FixedConfig(entries)), repository.clone())
            .execute()
            .unwrap()
    }

    #[test]
    fn saves_each_host_in_the_ssh_config_group() {
        let repository = InMemoryHosts::with(vec![]);

        let report = import(
            vec![
                entry("oracle", "10.0.0.1", 22, Some("~/.ssh/oracle.key")),
                entry("nas", "10.0.0.2", 2222, None),
            ],
            &repository,
        );

        assert_eq!(report.imported.len(), 2);
        let saved = repository.all().unwrap();
        assert_eq!(saved[0].label().as_str(), "oracle");
        assert_eq!(saved[0].group().unwrap().as_str(), "SSH config");
        assert!(matches!(saved[0].auth(), AuthMethod::KeyFile(_)));
        assert_eq!(saved[1].endpoint().port().value(), 2222);
        assert_eq!(saved[1].auth(), &AuthMethod::Password);
    }

    #[test]
    fn skips_hosts_that_are_already_saved() {
        // The test double's host points at ubuntu@example.com:22.
        let repository = InMemoryHosts::with(vec![host("web", None)]);

        let report = import(
            vec![
                entry("web-again", "example.com", 22, None),
                entry("dup", "10.0.0.9", 22, None),
                entry("dup-twice", "10.0.0.9", 22, None),
            ],
            &repository,
        );

        assert_eq!(report.imported.len(), 1);
        assert_eq!(report.already_saved, 2);
        assert_eq!(repository.all().unwrap().len(), 2);
    }

    #[test]
    fn reports_entries_it_cannot_use_and_keeps_going() {
        let repository = InMemoryHosts::with(vec![]);

        let report = import(
            vec![
                entry("zero", "10.0.0.1", 0, None),
                entry("ok", "10.0.0.2", 22, None),
            ],
            &repository,
        );

        assert_eq!(report.imported.len(), 1);
        assert_eq!(report.invalid.len(), 1);
        assert_eq!(report.invalid[0].0, "zero");
    }
}
