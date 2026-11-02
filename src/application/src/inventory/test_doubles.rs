//! An in-memory host repository, for use case tests.

use std::sync::{Arc, Mutex};

use sallygate_domain::credentials::{AuthMethod, KeyFilePath};
use sallygate_domain::inventory::{
    GroupName, Host, HostAddress, HostId, HostLabel, Port, SshEndpoint, Username,
};

use super::{HostRepository, HostStoreError};

pub fn host(label: &str, group: Option<&str>) -> Host {
    Host::new(
        HostId::new(),
        HostLabel::parse(label).unwrap(),
        SshEndpoint::new(
            HostAddress::parse("example.com").unwrap(),
            Port::default(),
            Username::parse("ubuntu").unwrap(),
        ),
        AuthMethod::KeyFile(KeyFilePath::parse("~/.ssh/id_ed25519").unwrap()),
        group.map(|name| GroupName::parse(name).unwrap()),
    )
}

#[derive(Default)]
pub struct InMemoryHosts(Mutex<Vec<Host>>);

impl InMemoryHosts {
    pub fn with(hosts: Vec<Host>) -> Arc<Self> {
        Arc::new(Self(Mutex::new(hosts)))
    }
}

impl HostRepository for InMemoryHosts {
    fn all(&self) -> Result<Vec<Host>, HostStoreError> {
        Ok(self.0.lock().unwrap().clone())
    }

    fn save(&self, host: &Host) -> Result<(), HostStoreError> {
        let mut hosts = self.0.lock().unwrap();
        match hosts.iter_mut().find(|saved| saved.id() == host.id()) {
            Some(saved) => *saved = host.clone(),
            None => hosts.push(host.clone()),
        }
        Ok(())
    }

    fn delete(&self, host_id: HostId) -> Result<(), HostStoreError> {
        self.0.lock().unwrap().retain(|host| host.id() != host_id);
        Ok(())
    }
}
