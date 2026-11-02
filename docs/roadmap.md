# Roadmap

Versions follow SemVer ([ADR-0007](decisions/0007-semver-and-gitflow.md)). Each roadmap step is a MINOR release while we are on `0.x`.

| Version | Delivers | Status |
|---|---|---|
| **0.1.0** | Spike: the Rust core connects to a real server and a terminal shows up in the app. Then: host list, tabs, `~/.ssh/config` import. | Next |
| **0.2.0** | Dual-pane SFTP with drag and drop and a transfer queue. | Planned |
| **0.3.0** | SSH keys, snippets, port forwarding, jump host, known hosts management. | Planned |
| **1.0.0** | Vault: an encrypted file behind a master password, replacing the keychain. | Planned |
| **2.0.0** | Secrets manager (`.env`, tokens), sync across devices, mobile app. | Idea |

## Out of scope for now

- Cloud sync, accounts, teams, sharing.
- Plain FTP.
- A full local terminal emulator.
