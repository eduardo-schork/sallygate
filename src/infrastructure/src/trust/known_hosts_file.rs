use std::path::PathBuf;

use russh::keys::known_hosts::{check_known_hosts_path, learn_known_hosts_path};
use sallygate_application::trust::{KnownHosts, KnownHostsError};
use sallygate_domain::inventory::SshEndpoint;
use sallygate_domain::trust::{HostKey, HostKeyStatus};

use super::openssh_host_key::public_key_from_host_key;

/// Trusted server keys in an `OpenSSH` `known_hosts` file, shared with the `ssh` command.
pub struct KnownHostsFile {
    path: PathBuf,
}

impl KnownHostsFile {
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl KnownHosts for KnownHostsFile {
    fn check(
        &self,
        endpoint: &SshEndpoint,
        key: &HostKey,
    ) -> Result<HostKeyStatus, KnownHostsError> {
        let public_key = public_key_from_host_key(key).map_err(KnownHostsError::Read)?;

        // russh reports a missing file as "no keys", so a first connection is simply unknown.
        match check_known_hosts_path(
            endpoint.address().as_str(),
            endpoint.port().value(),
            &public_key,
            &self.path,
        ) {
            Ok(true) => Ok(HostKeyStatus::Trusted),
            Ok(false) => Ok(HostKeyStatus::Unknown),
            Err(russh::keys::Error::KeyChanged { .. }) => Ok(HostKeyStatus::Changed),
            Err(error) => Err(KnownHostsError::Read(error.to_string())),
        }
    }

    fn trust(&self, endpoint: &SshEndpoint, key: &HostKey) -> Result<(), KnownHostsError> {
        let public_key = public_key_from_host_key(key).map_err(KnownHostsError::Write)?;

        learn_known_hosts_path(
            endpoint.address().as_str(),
            endpoint.port().value(),
            &public_key,
            &self.path,
        )
        .map_err(|error| KnownHostsError::Write(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use russh::keys::PublicKey;
    use sallygate_domain::inventory::{HostAddress, Port, Username};

    use super::*;
    use crate::trust::host_key_from_public_key;

    const SERVER_KEY: &str =
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIO8s/hhw5ji20pKlZuA5sy2IAShfDR14IXb9OXjRypwr";
    const OTHER_SERVER_KEY: &str =
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAICvziHveVjY25jcGXdPZThvPGd4RwnvIf6fCTELKrUoP";

    fn endpoint() -> SshEndpoint {
        SshEndpoint::new(
            HostAddress::parse("127.0.0.1").unwrap(),
            Port::try_from(2222).unwrap(),
            Username::parse("sallygate").unwrap(),
        )
    }

    fn host_key(openssh: &str) -> HostKey {
        host_key_from_public_key(&PublicKey::from_openssh(openssh).unwrap()).unwrap()
    }

    fn known_hosts_in_temp_dir() -> (tempfile::TempDir, KnownHostsFile) {
        let directory = tempfile::tempdir().unwrap();
        let file = KnownHostsFile::new(directory.path().join("known_hosts"));
        (directory, file)
    }

    #[test]
    fn a_server_is_unknown_when_the_file_does_not_exist_yet() {
        let (_directory, known_hosts) = known_hosts_in_temp_dir();

        let status = known_hosts.check(&endpoint(), &host_key(SERVER_KEY));

        assert_eq!(status, Ok(HostKeyStatus::Unknown));
    }

    #[test]
    fn a_trusted_key_is_recognized_afterwards() {
        let (_directory, known_hosts) = known_hosts_in_temp_dir();
        known_hosts
            .trust(&endpoint(), &host_key(SERVER_KEY))
            .unwrap();

        let status = known_hosts.check(&endpoint(), &host_key(SERVER_KEY));

        assert_eq!(status, Ok(HostKeyStatus::Trusted));
    }

    #[test]
    fn a_different_key_for_a_trusted_server_is_reported_as_changed() {
        let (_directory, known_hosts) = known_hosts_in_temp_dir();
        known_hosts
            .trust(&endpoint(), &host_key(SERVER_KEY))
            .unwrap();

        let status = known_hosts.check(&endpoint(), &host_key(OTHER_SERVER_KEY));

        assert_eq!(status, Ok(HostKeyStatus::Changed));
    }

    #[test]
    fn keys_are_stored_in_the_format_openssh_reads() {
        let (directory, known_hosts) = known_hosts_in_temp_dir();
        known_hosts
            .trust(&endpoint(), &host_key(SERVER_KEY))
            .unwrap();

        let contents = std::fs::read_to_string(directory.path().join("known_hosts")).unwrap();

        assert_eq!(contents.trim(), format!("[127.0.0.1]:2222 {SERVER_KEY}"));
    }
}
