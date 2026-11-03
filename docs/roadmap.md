# Roadmap

Versions follow SemVer ([ADR-0007](decisions/0007-semver-and-gitflow.md)). Each roadmap step is a MINOR release while we are on `0.x`.

| Version   | Delivers                                                                                                                                                                                    | Status      |
| --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------- |
| **0.1.0** | Saved hosts, a terminal per tab over SSH, server key confirmation, password and passphrase login typed at connect time, `~/.ssh/config` import (built). ssh-agent login moves to the vault. | In progress |
| **0.2.0** | Dual-pane SFTP with drag and drop and a transfer queue.                                                                                                                                     | Built       |
| **0.3.0** | SSH keys, snippets, port forwarding, jump host, known hosts management.                                                                                                                     | Built       |
| **1.0.0** | Vault: an encrypted file behind a master password, replacing the keychain.                                                                                                                  | Planned     |
| **2.0.0** | Secrets manager (`.env`, tokens), sync across devices, mobile app.                                                                                                                          | Idea        |

## Out of scope for now

- Cloud sync, accounts, teams, sharing.
- Plain FTP.
- A full local terminal emulator.
