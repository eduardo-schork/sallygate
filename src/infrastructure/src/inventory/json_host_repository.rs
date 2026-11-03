use std::path::PathBuf;
use std::sync::{Mutex, PoisonError};

use sallygate_application::inventory::{HostRepository, HostStoreError};
use sallygate_domain::credentials::{AuthMethod, KeyFilePath};
use sallygate_domain::inventory::{
    GroupName, Host, HostAddress, HostId, HostLabel, Port, SshEndpoint, Username,
};
use serde::{Deserialize, Serialize};

use crate::json_file;

/// The file format version; bump it and migrate when the shape changes.
const FORMAT_VERSION: u32 = 1;

/// Saved hosts in one JSON file. It holds no secrets, only where and how to connect.
pub struct JsonHostRepository {
    path: PathBuf,
    // Serializes read-modify-write cycles so two saves cannot lose each other's change.
    write_lock: Mutex<()>,
}

impl JsonHostRepository {
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            write_lock: Mutex::new(()),
        }
    }

    fn read_file(&self) -> Result<Vec<Host>, HostStoreError> {
        let Some(file) = json_file::read::<HostsFile>(&self.path).map_err(HostStoreError::Read)?
        else {
            return Ok(Vec::new());
        };
        file.hosts.into_iter().map(HostRecord::into_host).collect()
    }

    fn write_file(&self, hosts: &[Host]) -> Result<(), HostStoreError> {
        let file = HostsFile {
            version: FORMAT_VERSION,
            hosts: hosts.iter().map(HostRecord::from_host).collect(),
        };
        json_file::write_atomically(&self.path, &file).map_err(HostStoreError::Write)
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, ()> {
        self.write_lock
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }
}

impl HostRepository for JsonHostRepository {
    fn all(&self) -> Result<Vec<Host>, HostStoreError> {
        let _guard = self.lock();
        self.read_file()
    }

    fn save(&self, host: &Host) -> Result<(), HostStoreError> {
        let _guard = self.lock();
        let mut hosts = self.read_file()?;
        match hosts.iter_mut().find(|saved| saved.id() == host.id()) {
            Some(saved) => *saved = host.clone(),
            None => hosts.push(host.clone()),
        }
        self.write_file(&hosts)
    }

    fn delete(&self, host_id: HostId) -> Result<(), HostStoreError> {
        let _guard = self.lock();
        let mut hosts = self.read_file()?;
        hosts.retain(|host| host.id() != host_id);
        self.write_file(&hosts)
    }
}

#[derive(Serialize, Deserialize)]
struct HostsFile {
    version: u32,
    hosts: Vec<HostRecord>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HostRecord {
    id: String,
    label: String,
    address: String,
    port: u16,
    username: String,
    auth: AuthRecord,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    group: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum AuthRecord {
    KeyFile {
        path: String,
    },
    /// Only the method; the password itself is never saved.
    Password,
}

impl HostRecord {
    fn from_host(host: &Host) -> Self {
        Self {
            id: host.id().to_string(),
            label: host.label().to_string(),
            address: host.endpoint().address().to_string(),
            port: host.endpoint().port().value(),
            username: host.endpoint().username().to_string(),
            auth: match host.auth() {
                AuthMethod::KeyFile(key_file_path) => AuthRecord::KeyFile {
                    path: key_file_path.as_path().display().to_string(),
                },
                AuthMethod::Password => AuthRecord::Password,
            },
            group: host.group().map(ToString::to_string),
        }
    }

    /// A saved host goes through the same validation as a new one; a broken entry is an
    /// error, never silently dropped.
    fn into_host(self) -> Result<Host, HostStoreError> {
        let invalid =
            |reason: String| HostStoreError::Read(format!("host \"{}\": {reason}", self.label));
        let auth = match &self.auth {
            AuthRecord::KeyFile { path } => AuthMethod::KeyFile(
                KeyFilePath::parse(path).map_err(|error| invalid(error.to_string()))?,
            ),
            AuthRecord::Password => AuthMethod::Password,
        };

        Ok(Host::new(
            self.id
                .parse::<HostId>()
                .map_err(|error| invalid(error.to_string()))?,
            HostLabel::parse(&self.label).map_err(|error| invalid(error.to_string()))?,
            SshEndpoint::new(
                HostAddress::parse(&self.address).map_err(|error| invalid(error.to_string()))?,
                Port::try_from(self.port).map_err(|error| invalid(error.to_string()))?,
                Username::parse(&self.username).map_err(|error| invalid(error.to_string()))?,
            ),
            auth,
            self.group
                .as_deref()
                .map(GroupName::parse)
                .transpose()
                .map_err(|error| invalid(error.to_string()))?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn host(label: &str, group: Option<&str>) -> Host {
        Host::new(
            HostId::new(),
            HostLabel::parse(label).unwrap(),
            SshEndpoint::new(
                HostAddress::parse("10.0.0.5").unwrap(),
                Port::try_from(2222).unwrap(),
                Username::parse("ubuntu").unwrap(),
            ),
            AuthMethod::KeyFile(KeyFilePath::parse("~/.ssh/id_ed25519").unwrap()),
            group.map(|name| GroupName::parse(name).unwrap()),
        )
    }

    fn repository_in_temp_dir() -> (tempfile::TempDir, JsonHostRepository) {
        let folder = tempfile::tempdir().unwrap();
        let repository = JsonHostRepository::new(folder.path().join("data").join("hosts.json"));
        (folder, repository)
    }

    #[test]
    fn has_no_hosts_before_the_first_save() {
        let (_folder, repository) = repository_in_temp_dir();
        assert_eq!(repository.all(), Ok(vec![]));
    }

    #[test]
    fn keeps_saved_hosts_across_instances() {
        let (folder, repository) = repository_in_temp_dir();
        let web = host("web", Some("Cloud"));
        let nas = host("nas", None);
        repository.save(&web).unwrap();
        repository.save(&nas).unwrap();

        let reopened = JsonHostRepository::new(folder.path().join("data").join("hosts.json"));

        assert_eq!(reopened.all(), Ok(vec![web, nas]));
    }

    #[test]
    fn saving_an_existing_host_replaces_it() {
        let (_folder, repository) = repository_in_temp_dir();
        let original = host("web", None);
        repository.save(&original).unwrap();
        let regrouped = Host::new(
            original.id(),
            original.label().clone(),
            original.endpoint().clone(),
            original.auth().clone(),
            Some(GroupName::parse("Cloud").unwrap()),
        );

        repository.save(&regrouped).unwrap();

        assert_eq!(repository.all(), Ok(vec![regrouped]));
    }

    #[test]
    fn deletes_a_host() {
        let (_folder, repository) = repository_in_temp_dir();
        let web = host("web", None);
        repository.save(&web).unwrap();

        repository.delete(web.id()).unwrap();

        assert_eq!(repository.all(), Ok(vec![]));
    }

    #[test]
    fn reports_a_broken_entry_instead_of_dropping_it() {
        let (folder, repository) = repository_in_temp_dir();
        fs::create_dir_all(folder.path().join("data")).unwrap();
        fs::write(
            folder.path().join("data").join("hosts.json"),
            r#"{"version":1,"hosts":[{"id":"not-an-id","label":"web","address":"x","port":22,"username":"u","auth":{"type":"keyFile","path":"k"}}]}"#,
        )
        .unwrap();

        assert!(
            matches!(repository.all(), Err(HostStoreError::Read(message)) if message.contains("web"))
        );
    }

    #[test]
    fn writes_no_secrets_only_the_key_file_path() {
        let (folder, repository) = repository_in_temp_dir();
        repository.save(&host("web", None)).unwrap();

        let contents = fs::read_to_string(folder.path().join("data").join("hosts.json")).unwrap();

        assert!(contents.contains(r#""path": "~/.ssh/id_ed25519""#));
        assert!(!contents.contains("PRIVATE KEY"));
    }
}
