# Decisions (ADRs)

One file per decision, numbered in order. Copy [0000-template.md](0000-template.md) for a new one. To change a decision, write a new ADR that supersedes the old one and set the old one's status to `Superseded by ADR-XXXX`.

| ADR                                                         | Decision                                                                | Status                                                                  |
| ----------------------------------------------------------- | ----------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| [0001](0001-ssh-and-sftp-over-one-protocol.md)              | SSH and SFTP, no FTP                                                    | Accepted                                                                |
| [0002](0002-tauri-rust-react-stack.md)                      | Tauri 2 + Rust core + React/TypeScript UI                               | Accepted                                                                |
| [0003](0003-faber-ui-design-system.md)                      | faber-ui as the design system                                           | Accepted                                                                |
| [0004](0004-termius-style-layout.md)                        | Termius-style layout                                                    | Accepted                                                                |
| [0005](0005-secrets-in-keychain-until-vault.md)             | Secrets in the OS keychain until the vault exists                       | Accepted                                                                |
| [0006](0006-english-only.md)                                | English for all written artifacts                                       | Accepted                                                                |
| [0007](0007-semver-and-gitflow.md)                          | Semantic Versioning and Gitflow                                         | Release branches optional since [0022](0022-release-on-push-to-main.md) |
| [0008](0008-name-sallygate.md)                              | Name the project Sallygate                                              | Accepted                                                                |
| [0009](0009-ddd-clean-hexagonal-architecture.md)            | DDD with Clean Architecture and Ports and Adapters                      | Accepted (layout superseded by 0011)                                    |
| [0010](0010-frontend-follows-faber-ui-conventions.md)       | The UI follows faber-ui's code conventions                              | Accepted (UI layering superseded by 0011)                               |
| [0011](0011-layer-first-source-layout.md)                   | Layer-first source layout under `src/`, UI as part of presentation      | Accepted                                                                |
| [0012](0012-async-ports-with-async-trait.md)                | Async ports as `async-trait` traits behind `Arc<dyn …>`                 | Accepted                                                                |
| [0013](0013-ui-contract-channels-and-generated-types.md)    | UI contract with per-session channels and generated TypeScript types    | Accepted                                                                |
| [0014](0014-confirm-unknown-host-keys-before-connecting.md) | Unknown host keys are confirmed by the user before connecting           | Accepted                                                                |
| [0015](0015-secrets-typed-at-connect-time.md)               | Passwords and key passphrases are typed at connect time                 | Accepted                                                                |
| [0016](0016-sallygate-brand-theme.md)                       | Sallygate brand theme on top of faber-ui                                | Accepted                                                                |
| [0017](0017-sftp-over-the-session-connection.md)            | SFTP runs over the open session's connection                            | Superseded in part by [0020](0020-two-pane-file-browser.md)             |
| [0018](0018-port-forwarding-rules.md)                       | Port forwarding rules, one SSH connection each                          | Accepted                                                                |
| [0019](0019-jump-hosts.md)                                  | Jump hosts go through a saved host that needs no typed secret           | Accepted                                                                |
| [0020](0020-two-pane-file-browser.md)                       | The file browser shows this Mac beside the server                       | Accepted                                                                |
| [0021](0021-ci-and-release-pipelines.md)                    | GitHub Actions for checks and macOS releases                            | Superseded in part by [0022](0022-release-on-push-to-main.md)           |
| [0022](0022-release-on-push-to-main.md)                     | A new version on main releases itself                                   | Superseded by [0023](0023-version-from-commits.md)                      |
| [0023](0023-version-from-commits.md)                        | Every merge into main releases, with the version taken from the commits | Accepted                                                                |
