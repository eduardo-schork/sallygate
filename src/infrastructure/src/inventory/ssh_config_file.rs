use std::path::PathBuf;

use sallygate_application::inventory::{SshConfigEntry, SshConfigError, SshConfigSource};

/// The key files `ssh` tries when the config names none, in its order.
const DEFAULT_KEY_FILES: [&str; 3] = ["id_ed25519", "id_ecdsa", "id_rsa"];
const DEFAULT_PORT: u16 = 22;

/// Reads the user's OpenSSH config (`~/.ssh/config`).
///
/// Understands `Host`, `HostName`, `User`, `Port` and `IdentityFile`; other keywords are ignored.
/// shortcut: `Include` and `Match` blocks are skipped, and `Host *` only fills values a host
/// leaves unset (OpenSSH takes the first value in file order); upgrade if users report misses.
pub struct SshConfigFile {
    path: PathBuf,
    ssh_folder: PathBuf,
    local_username: String,
}

impl SshConfigFile {
    /// `ssh_folder` is where the default keys live; `local_username` is the user `ssh` logs in
    /// as when the config names none.
    #[must_use]
    pub fn new(path: PathBuf, ssh_folder: PathBuf, local_username: String) -> Self {
        Self {
            path,
            ssh_folder,
            local_username,
        }
    }

    fn default_key_file(&self) -> Option<String> {
        DEFAULT_KEY_FILES
            .iter()
            .map(|name| self.ssh_folder.join(name))
            .find(|path| path.is_file())
            .map(|path| path.to_string_lossy().into_owned())
    }
}

impl SshConfigSource for SshConfigFile {
    fn entries(&self) -> Result<Vec<SshConfigEntry>, SshConfigError> {
        let text = match std::fs::read_to_string(&self.path) {
            Ok(text) => text,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(SshConfigError::Read(error.to_string())),
        };
        let default_key = self.default_key_file();
        Ok(parse(&text)
            .into_iter()
            .map(|block| SshConfigEntry {
                address: block.host_name.unwrap_or_else(|| block.alias.clone()),
                port: block.port.unwrap_or(DEFAULT_PORT),
                username: block.user.unwrap_or_else(|| self.local_username.clone()),
                key_file: block.identity_file.or_else(|| default_key.clone()),
                alias: block.alias,
            })
            .collect())
    }
}

/// The settings found for one host alias.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct HostBlock {
    alias: String,
    host_name: Option<String>,
    user: Option<String>,
    port: Option<u16>,
    identity_file: Option<String>,
}

impl HostBlock {
    /// Keeps the first value of each setting, like OpenSSH.
    fn set(&mut self, keyword: &str, value: &str) {
        match keyword {
            "hostname" => {
                self.host_name.get_or_insert_with(|| value.to_owned());
            }
            "user" => {
                self.user.get_or_insert_with(|| value.to_owned());
            }
            // An unreadable port becomes 0, which the use case reports as invalid.
            "port" => {
                self.port.get_or_insert_with(|| value.parse().unwrap_or(0));
            }
            "identityfile" => {
                self.identity_file.get_or_insert_with(|| value.to_owned());
            }
            _ => {}
        }
    }

    fn fill_from(&mut self, defaults: &Self) {
        self.host_name = self.host_name.take().or_else(|| defaults.host_name.clone());
        self.user = self.user.take().or_else(|| defaults.user.clone());
        self.port = self.port.or(defaults.port);
        self.identity_file = self
            .identity_file
            .take()
            .or_else(|| defaults.identity_file.clone());
    }
}

fn is_pattern(alias: &str) -> bool {
    alias.contains(['*', '?', '!'])
}

/// Splits `Keyword value`, `Keyword=value` or `Keyword "quoted value"`.
fn split_line(line: &str) -> Option<(String, &str)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let split_at = line.find(|c: char| c.is_whitespace() || c == '=')?;
    let (keyword, rest) = line.split_at(split_at);
    let value = rest
        .trim_start_matches(|c: char| c.is_whitespace() || c == '=')
        .trim();
    Some((keyword.to_lowercase(), value.trim_matches('"')))
}

fn parse(text: &str) -> Vec<HostBlock> {
    let mut defaults = HostBlock::default();
    let mut hosts: Vec<HostBlock> = Vec::new();
    // Indexes into `hosts` the current block applies to; `None` means the defaults.
    let mut current: Option<Vec<usize>> = None;
    let mut in_match = false;

    for (keyword, value) in text.lines().filter_map(split_line) {
        match keyword.as_str() {
            "host" => {
                in_match = false;
                let aliases: Vec<&str> = value.split_whitespace().collect();
                if aliases.iter().all(|alias| *alias == "*") {
                    current = None;
                    continue;
                }
                let mut indexes = Vec::new();
                for alias in aliases.into_iter().filter(|alias| !is_pattern(alias)) {
                    indexes.push(hosts.len());
                    hosts.push(HostBlock {
                        alias: alias.to_owned(),
                        ..HostBlock::default()
                    });
                }
                current = Some(indexes);
            }
            "match" => in_match = true,
            _ if in_match => {}
            _ => match &current {
                None => defaults.set(&keyword, value),
                Some(indexes) => {
                    for &index in indexes {
                        hosts[index].set(&keyword, value);
                    }
                }
            },
        }
    }

    for host in &mut hosts {
        host.fill_from(&defaults);
    }
    hosts
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn source(folder: &Path, config: &str) -> SshConfigFile {
        let path = folder.join("config");
        std::fs::write(&path, config).unwrap();
        SshConfigFile::new(path, folder.to_path_buf(), "me".to_owned())
    }

    #[test]
    fn reads_each_concrete_host_with_its_settings() {
        let folder = tempfile::tempdir().unwrap();
        let config = "\
# my servers
Host oracle
    HostName 10.0.0.1
    User ubuntu
    IdentityFile ~/Downloads/ssh-key.key

Host nas backup
  hostname=10.0.0.2
  Port 2222
";
        let entries = source(folder.path(), config).entries().unwrap();

        assert_eq!(entries.len(), 3);
        assert_eq!(
            entries[0],
            SshConfigEntry {
                alias: "oracle".to_owned(),
                address: "10.0.0.1".to_owned(),
                port: 22,
                username: "ubuntu".to_owned(),
                key_file: Some("~/Downloads/ssh-key.key".to_owned()),
            }
        );
        assert_eq!(entries[1].alias, "nas");
        assert_eq!(entries[2].alias, "backup");
        assert_eq!(entries[2].port, 2222);
        assert_eq!(entries[2].username, "me");
        assert_eq!(entries[2].key_file, None);
    }

    #[test]
    fn leaves_out_patterns_and_match_blocks_and_uses_host_star_as_defaults() {
        let folder = tempfile::tempdir().unwrap();
        let config = "\
Host *.internal
    User admin
Host web
    HostName web.example.com
Match host web
    User ignored
Host *
    User deploy
    Port 2200
";
        let entries = source(folder.path(), config).entries().unwrap();

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].alias, "web");
        assert_eq!(entries[0].username, "deploy");
        assert_eq!(entries[0].port, 2200);
    }

    #[test]
    fn falls_back_to_the_default_key_and_the_alias_as_address() {
        let folder = tempfile::tempdir().unwrap();
        std::fs::write(folder.path().join("id_rsa"), "key").unwrap();
        std::fs::write(folder.path().join("id_ed25519"), "key").unwrap();

        let entries = source(folder.path(), "Host box\n").entries().unwrap();

        assert_eq!(entries[0].address, "box");
        let expected = folder.path().join("id_ed25519");
        assert_eq!(entries[0].key_file.as_deref(), expected.to_str());
    }

    #[test]
    fn a_missing_config_has_no_hosts() {
        let folder = tempfile::tempdir().unwrap();
        let missing = SshConfigFile::new(
            folder.path().join("config"),
            folder.path().to_path_buf(),
            "me".to_owned(),
        );

        assert_eq!(missing.entries().unwrap(), Vec::new());
    }
}
