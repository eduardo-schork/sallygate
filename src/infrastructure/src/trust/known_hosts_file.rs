use std::path::PathBuf;

use russh::keys::PublicKey;
use russh::keys::known_hosts::{check_known_hosts_path, learn_known_hosts_path};
use sallygate_application::trust::{KnownHosts, KnownHostsError};
use sallygate_domain::inventory::SshEndpoint;
use sallygate_domain::trust::{HostKey, HostKeyStatus, KnownHostEntry};

use super::openssh_host_key::{host_key_from_public_key, public_key_from_host_key};

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

impl KnownHostsFile {
    fn read_lines(&self) -> Result<Vec<String>, KnownHostsError> {
        match std::fs::read_to_string(&self.path) {
            Ok(text) => Ok(text.lines().map(ToOwned::to_owned).collect()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(error) => Err(KnownHostsError::Read(error.to_string())),
        }
    }
}

/// Reads `hosts keytype base64 [comment]`; `None` for comments, blank lines and marked lines
/// (`@revoked`, `@cert-authority`), which this screen does not manage.
fn parse_entry(line_number: usize, line: &str) -> Option<KnownHostEntry> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') || line.starts_with('@') {
        return None;
    }
    let mut fields = line.split_whitespace();
    let hosts = fields.next()?;
    let key = format!("{} {}", fields.next()?, fields.next()?);
    let host_key = host_key_from_public_key(&PublicKey::from_openssh(&key).ok()?).ok()?;
    Some(KnownHostEntry {
        line: line_number,
        hosts: (!hosts.starts_with('|')).then(|| hosts.to_owned()),
        host_key,
    })
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

    fn entries(&self) -> Result<Vec<KnownHostEntry>, KnownHostsError> {
        Ok(self
            .read_lines()?
            .iter()
            .enumerate()
            .filter_map(|(index, line)| parse_entry(index + 1, line))
            .collect())
    }

    fn forget(&self, entry: &KnownHostEntry) -> Result<(), KnownHostsError> {
        let mut lines = self.read_lines()?;
        let still_there = entry
            .line
            .checked_sub(1)
            .and_then(|index| lines.get(index))
            .and_then(|line| parse_entry(entry.line, line))
            .is_some_and(|found| found == *entry);
        if !still_there {
            return Err(KnownHostsError::Moved);
        }
        lines.remove(entry.line - 1);
        let mut text = lines.join("\n");
        if !text.is_empty() {
            text.push('\n');
        }
        // Written beside the file, then renamed, so a crash never leaves half a list.
        let temporary_path = self.path.with_extension("sallygate.tmp");
        std::fs::write(&temporary_path, text)
            .and_then(|()| std::fs::rename(&temporary_path, &self.path))
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

    #[test]
    fn lists_keys_and_forgets_exactly_one_line() {
        let (directory, known_hosts) = known_hosts_in_temp_dir();
        let path = directory.path().join("known_hosts");
        std::fs::write(
            &path,
            format!(
                "# my servers\n[127.0.0.1]:2222 {SERVER_KEY}\n|1|c2FsdA==|aGFzaA== {OTHER_SERVER_KEY}\nexample.com {OTHER_SERVER_KEY}\n"
            ),
        )
        .unwrap();

        let entries = known_hosts.entries().unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].line, 2);
        assert_eq!(entries[0].hosts.as_deref(), Some("[127.0.0.1]:2222"));
        assert!(entries[0].host_key.fingerprint().starts_with("SHA256:"));
        assert_eq!(entries[1].hosts, None);

        known_hosts.forget(&entries[0]).unwrap();

        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            format!(
                "# my servers\n|1|c2FsdA==|aGFzaA== {OTHER_SERVER_KEY}\nexample.com {OTHER_SERVER_KEY}\n"
            )
        );
        assert_eq!(known_hosts.forget(&entries[0]), Err(KnownHostsError::Moved));
    }
}
