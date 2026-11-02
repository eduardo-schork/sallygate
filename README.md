# Sallygate

A desktop SSH and SFTP manager built with Tauri 2, Rust and React + [faber-ui](https://github.com/eduardo-schork/faber-ui).

> Early development. Today you can save hosts in groups and open a terminal on each one, in its own tab, over SSH with a private key. Sallygate asks before trusting a new server key.

Planned: saved hosts and groups, tabbed SSH terminal, dual-pane SFTP, SSH keys, snippets, port forwarding and jump hosts. Later: an encrypted vault and a secrets manager.

## Develop

Requirements: [Rust](https://rustup.rs) (stable), [Bun](https://bun.sh), and the [Tauri prerequisites](https://tauri.app/start/prerequisites/) for your OS.

```bash
bun install
bun run tauri dev        # run the app
```

| Command                      | What it checks                                  |
| ---------------------------- | ----------------------------------------------- |
| `bun run check`              | Everything below except integration, fail-fast  |
| `bun run check:full`         | Everything, including integration tests         |
| `bun run typecheck`          | TypeScript types                                |
| `bun run lint`               | ESLint, layer boundaries and kebab-case paths   |
| `bun run format:check`       | Prettier                                        |
| `bun run test`               | UI and lint-rule tests (Vitest)                 |
| `bun run lint:rust`          | `rustfmt` and Clippy                            |
| `bun run test:rust`          | Rust tests                                      |
| `bun run test:integration`   | Rust tests against the SSH test server          |
| `bun run check:architecture` | Crate dependencies follow the architecture rule |

### SSH test server

Integration tests and local development use a disposable SSH server in Docker, so they never touch a real machine:

```bash
scripts/test-ssh-server.sh up     # 127.0.0.1:2222, user `sallygate`, key in tests/ssh-server/.keys/
scripts/test-ssh-server.sh down
```

Architecture, conventions and decisions are in [docs/](docs/README.md).
