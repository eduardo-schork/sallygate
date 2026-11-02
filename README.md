# Sallygate

A desktop SSH and SFTP manager built with Tauri 2, Rust and React + [faber-ui](https://github.com/eduardo-schork/faber-ui).

> Early development. The app opens, but has no features yet.

Planned: saved hosts and groups, tabbed SSH terminal, dual-pane SFTP, SSH keys, snippets, port forwarding and jump hosts. Later: an encrypted vault and a secrets manager.

## Develop

Requirements: [Rust](https://rustup.rs) (stable), [Bun](https://bun.sh), and the [Tauri prerequisites](https://tauri.app/start/prerequisites/) for your OS.

```bash
bun install
bun run tauri dev        # run the app
```

| Command                      | What it checks                                  |
| ---------------------------- | ----------------------------------------------- |
| `bun run typecheck`          | TypeScript types                                |
| `bun run lint`               | ESLint, layer boundaries and kebab-case paths   |
| `bun run format:check`       | Prettier                                        |
| `bun run test`               | UI and lint-rule tests (Vitest)                 |
| `bun run lint:rust`          | `rustfmt` and Clippy                            |
| `bun run test:rust`          | Rust tests                                      |
| `bun run check:architecture` | Crate dependencies follow the architecture rule |

Architecture, conventions and decisions are in [docs/](docs/README.md).
