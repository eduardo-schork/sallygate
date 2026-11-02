# Development plan

The step-by-step plan for building Sallygate, and the place where progress is tracked. [roadmap.md](roadmap.md) is the short version-level summary; this file holds the detail.

**Status legend:** `[ ]` to do · `[~]` in progress · `[x]` done · `[-]` dropped (say why)

## Current focus

- **Stage:** 1 — SSH terminal (built; waiting on the manual check)
- **Next task:** the owner tests the app window against the test server, then their own server
- **Then:** Stage 2, starting with the `inventory` domain (`Host`, `Group`)
- **Blocked on:** the owner's manual check

Update this block whenever the focus moves.

---

## Stage 0 — Foundation

**Goal:** a repo, toolchain and empty app that build and run, so every later stage only adds features. Done, except two owner tasks below.
**Branch:** `develop`
**Done when:** `bun run tauri dev` opens an empty Sallygate window styled with faber-ui, and every local check passes.

- [x] Project folder, git repo and GitHub remote
- [x] README, `.gitignore`, agent instructions, project brief
- [x] Docs structure: architecture, decisions, roadmap, security, glossary, this plan
- [x] `develop` branch created locally
- [x] Architecture style and code conventions decided (ADR-0009, ADR-0010, [conventions.md](conventions.md))
- [ ] Push `develop`; make it the default branch on GitHub
- [x] Install toolchain: `rustup` (stable, via Homebrew), Bun; the Tauri CLI is a dev dependency (`@tauri-apps/cli`)
- [x] Scaffold the Tauri 2 app (React + TypeScript + Vite)
- [x] Cargo workspace with the layer crates: `sallygate-domain`, `sallygate-application`, `sallygate-infrastructure`, and the `sallygate` app (now under `src/`, see ADR-0011)
- [-] Empty context modules and UI layer folders: dropped as empty scaffolding; each context's modules and folders are created with its first real code (Stage 1 `sessions`, `trust`)
- [x] Add faber-ui (`styles.css`, `GlobalStyles`, dark theme) and render one faber-ui screen (`src/presentation/ui/app/`)
- [x] Version `0.1.0-dev` in the workspace `Cargo.toml` and `package.json`; `tauri.conf.json` reads it from Cargo
- [x] Rust quality gates: `rustfmt`, Clippy with `-D warnings` + `pedantic` + `unwrap_used`/`expect_used`/`panic` denied, `unsafe_code` forbidden (workspace lints)
- [x] TypeScript quality gates: ESLint strict type-checked, Prettier, faber-ui's kebab-case and no-hard-coded-design-values rules, and layer-boundary import rules
- [x] Architecture check: `bun run check:architecture` fails if a crate gains a dependency that breaks the dependency rule
- [-] CI on GitHub Actions: deferred by the owner; local checks are the gate for now
- [ ] Reserve the `sallygate` crate name on crates.io

## Stage 1 — SSH terminal (0.1.0, part A)

**Goal:** the Rust core holds a real SSH session and its output reaches a terminal in the app. This is also the Rust learning stage.
**Branch:** `develop`
**Done when:** the app opens a working, resizable interactive shell on a real server, typed into an `xterm.js` terminal.

- [x] Local test server: `tests/ssh-server/` (Alpine + OpenSSH, user `sallygate`, key and password auth) on `127.0.0.1:2222`; `scripts/test-ssh-server.sh up|down`
- [x] Throwaway spike in `spikes/ssh-cli/`: key auth, server fingerprint, `exec`, PTY shell and resize all work with `russh` 0.65. Deleted once `RusshConnector` landed.
- [x] Decide the async-port approach, the UI contract and the host key flow (ADR-0012, 0013, 0014)
- [x] Domain: `SshEndpoint` (address, port, username), `AuthMethod::KeyFile`, `HostKey`/`HostKeyStatus`, `SessionId`, `TerminalSize`, `SessionEnd`
- [x] `trust`: `KnownHosts` port, `TrustHostKey` use case, `KnownHostsFile` adapter (unknown, trusted, changed)
- [x] `sessions` application: `OpenSession`, `WriteToSession`, `ResizeSession`, `CloseSession`; `SshConnector`, `LiveSession` and `SessionOutput` ports; `SessionRegistry`; tests with fakes
- [x] `RusshConnector` adapter with 8 integration tests against the Docker server (`bun run test:integration`)
- [x] Tauri driving adapter: `open_session` (with a per-session `Channel`), `write_to_session`, `resize_session`, `close_session`, `trust_host_key`; DTOs generate `*-contract.ts` with `ts-rs`; composition root in `composition.rs`
- [x] Transport decided: per-session Tauri channel with base64 output (ADR-0013); revisit only if profiling shows a need
- [x] UI: `sessions-gateway.ts`, `trust-gateway.ts`, `use-connection` hook, connect form, unknown/changed host key dialogs, `xterm.js` terminal themed from faber-ui tokens, session bar, session-ended notice
- [ ] Manual check in the app window: connect to the test server (trust prompt, shell, resize, close, exit), then to the owner's server

## Stage 2 — Hosts and tabs (0.1.0, part B)

**Goal:** a usable daily SSH client for saved hosts.
**Branch:** `develop`
**Done when:** the owner uses Sallygate instead of `ssh` in the terminal for a week. Then cut `release/0.1.0`.

- [ ] `inventory` domain: `Host`, `Group` and their value objects; application: `SaveHost`, `ListHosts`, `DeleteHost`; `HostRepository` port; `JsonHostRepository` adapter
- [ ] `credentials`: `SecretStore` port, `KeychainSecretStore` adapter (`keyring`); auth methods: key file, password, ssh-agent
- [ ] UI `inventory` context: gateway, hooks, components
- [ ] Hosts screen: group and host cards, search, quick connect (Termius-style layout, faber-ui)
- [ ] Host drawer: create and edit a host
- [ ] Window tab bar: one tab per session, close and switch
- [ ] Host key prompts in the UI (unknown key, changed key)
- [ ] Import `~/.ssh/config`
- [ ] Release `0.1.0`: `release/0.1.0`, changelog, tag `v0.1.0`, signed macOS build

## Stage 3 — SFTP (0.2.0)

**Goal:** move files without leaving the app.
**Done when:** files go both ways between the Mac and a server by drag and drop, with progress.

- [ ] `transfer` context: domain, use cases, `SftpClient` port, `russh-sftp` adapter on top of an existing session
- [ ] Dual-pane browser: local and remote, path bar, file list
- [ ] Upload and download with a transfer queue and progress events
- [ ] Create, rename and delete files and folders
- [ ] Release `0.2.0`

## Stage 4 — Power tools (0.3.0)

- [ ] SSH keys screen: list, generate Ed25519, import, copy public key
- [ ] Snippets: save commands, send to a session
- [ ] Port forwarding: local, remote and SOCKS rules with on/off switches
- [ ] Jump host (`ProxyJump`)
- [ ] Known hosts screen: review and remove trusted keys
- [ ] Release `0.3.0`

## Stage 5 — Vault (1.0.0)

To be detailed when Stage 4 is done. Starts with an ADR on the vault's encryption design.

## Stage 6 — Secrets manager, sync, mobile (2.0.0)

To be detailed later.

---

## Progress log

Newest first. One line per meaningful step: date, what happened, link to the PR or commit.

- **2026-11-02** — Desktop commands and the Stage 1 UI landed: connect form, host key dialogs, `xterm.js` terminal. 11 Rust tests for the contract and 28 UI tests pass; the form and error display were checked in the browser. The native window flow still needs a manual check.
- **2026-11-02** — Sessions core done: domain values, use cases with ports, `KnownHostsFile` and `RusshConnector` adapters. 8 integration tests pass against the Docker server (unknown/changed key, shell output, resize, exit status, user close, rejected login, unreachable server). Spike deleted.
- **2026-11-02** — Source layout reorganized by layer under `src/` (`domain`, `application`, `infrastructure`, `presentation/{desktop,ui}`); UI simplified to gateway + hooks + components per context (ADR-0011). All checks pass and the app launches from the new layout.
- **2026-11-02** — SSH spike with `russh` 0.65 against the test server. Learned: the server key arrives in `client::Handler::check_server_key` as `russh::keys::PublicKeyOrCertificate` (fingerprint via `HashAlg::Sha256`); `russh::keys` has `check_known_hosts_path` / `learn_known_hosts_path`; a terminal is `channel_open_session` → `request_pty` → `request_shell`, resize is `window_change`, output comes from `channel.wait()` as `ChannelMsg::Data` and ends with `ExitStatus`; shells send escape codes (bracketed paste, colors) that `xterm.js` must receive as raw bytes.
- **2026-11-02** — Foundation on `develop`: Tauri 2 + React app with a faber-ui empty state, layer crates, strict Rust and TS lint, layer-boundary checks (both proven to reject a violation). Typecheck, lint, format, tests, UI build and Clippy pass locally; `tauri dev` opens the window. For now: commit straight to `develop` with Conventional Commits, no CI (local checks are the gate).
- **2026-11-02** — Code architecture set: DDD + Clean + Ports and Adapters, one crate per layer (ADR-0009); UI follows faber-ui conventions (ADR-0010); [conventions.md](conventions.md) added; plan reworked around contexts and layers.
- **2026-11-02** — Docs structure created (architecture, ADRs 0001–0008, roadmap, security, glossary, this plan).
- **2026-11-02** — `develop` branch created locally; SemVer and Gitflow adopted (ADR-0007).
- **2026-11-02** — Repo created at github.com/eduardo-schork/sallygate; first commit on `main`.
- **2026-11-02** — Stack, layout and name decided (ADR-0002, 0004, 0008).
