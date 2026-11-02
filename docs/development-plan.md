# Development plan

The step-by-step plan for building Sallygate, and the place where progress is tracked. [roadmap.md](roadmap.md) is the short version-level summary; this file holds the detail.

**Status legend:** `[ ]` to do · `[~]` in progress · `[x]` done · `[-]` dropped (say why)

## Current focus

- **Stage:** 0 — Foundation
- **Next task:** Stage 1, decide the async-port approach and the UI type-generation tool (ADRs)
- **Blocked on:** nothing

Update this block whenever the focus moves.

---

## Stage 0 — Foundation

**Goal:** a repo, toolchain and empty app that build and run, so every later stage only adds features.
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
- [x] Cargo workspace with the layer crates: `sallygate-domain`, `sallygate-application`, `sallygate-infrastructure`, and the `sallygate` app in `src-tauri/`
- [-] Empty context modules and UI layer folders: dropped as empty scaffolding; each context's modules and folders are created with its first real code (Stage 1 `sessions`, `trust`)
- [x] Add faber-ui (`styles.css`, `GlobalStyles`, dark theme) and render one faber-ui screen (`src/app/`)
- [x] Version `0.1.0-dev` in the workspace `Cargo.toml` and `package.json`; `tauri.conf.json` reads it from Cargo
- [x] Rust quality gates: `rustfmt`, Clippy with `-D warnings` + `pedantic` + `unwrap_used`/`expect_used`/`panic` denied, `unsafe_code` forbidden (workspace lints)
- [x] TypeScript quality gates: ESLint strict type-checked, Prettier, faber-ui's kebab-case and no-hard-coded-design-values rules, and layer-boundary import rules
- [x] Architecture check: `bun run check:architecture` fails if a crate gains a dependency that breaks the dependency rule
- [-] CI on GitHub Actions: deferred by the owner; local checks are the gate for now
- [ ] Reserve the `sallygate` crate name on crates.io

## Stage 1 — Rust SSH spike (0.1.0, part A)

**Goal:** prove that the Rust core can hold a real SSH session and that its output reaches the terminal in the UI. This is also the Rust learning stage.
**Branch:** `develop`
**Done when:** the app opens a working, resizable interactive shell on a real server, typed into an `xterm.js` terminal.

- [x] Local test server: `tests/ssh-server/` (Alpine + OpenSSH, user `sallygate`, key and password auth) on `127.0.0.1:2222`; `scripts/test-ssh-server.sh up|down`
- [x] Throwaway spike in `spikes/ssh-cli/` (gitignored, not part of the layers): key auth, server fingerprint, `exec`, PTY shell and resize all work with `russh` 0.65. Delete it when the `RusshConnector` adapter lands.
- [ ] Decide the async-port approach and the UI type-generation tool (ADRs)
- [ ] `sessions` domain: `SessionId`, `TerminalSize`, session states
- [ ] `trust` domain + `known_hosts` file adapter: verify, unknown key, changed key
- [ ] `sessions` application: `OpenSession`, `WriteToSession`, `ResizeSession`, `CloseSession` use cases; `SshConnector` and `SessionEvents` ports; tests with fakes
- [ ] `RusshConnector` adapter with integration tests against the Docker server
- [ ] Tauri driving adapter: `open_session`, `session_write`, `session_resize`, `session_close` commands and the `session_output` event; wiring in the composition root
- [ ] Measure: events with base64 bytes vs a Tauri channel; record the choice as an ADR
- [ ] UI `sessions` module: gateway port, Tauri gateway, `use-session` hook, `terminal-view` component with `xterm.js`
- [ ] Connect to the owner's real server from the app

## Stage 2 — Hosts and tabs (0.1.0, part B)

**Goal:** a usable daily SSH client for saved hosts.
**Branch:** `develop`
**Done when:** the owner uses Sallygate instead of `ssh` in the terminal for a week. Then cut `release/0.1.0`.

- [ ] `inventory` domain: `Host`, `Group` and their value objects; application: `SaveHost`, `ListHosts`, `DeleteHost`; `HostRepository` port; `JsonHostRepository` adapter
- [ ] `credentials`: `SecretStore` port, `KeychainSecretStore` adapter (`keyring`); auth methods: key file, password, ssh-agent
- [ ] UI `inventory` module: gateway, hooks, presentation
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

- **2026-11-02** — SSH spike with `russh` 0.65 against the test server. Learned: the server key arrives in `client::Handler::check_server_key` as `russh::keys::PublicKeyOrCertificate` (fingerprint via `HashAlg::Sha256`); `russh::keys` has `check_known_hosts_path` / `learn_known_hosts_path`; a terminal is `channel_open_session` → `request_pty` → `request_shell`, resize is `window_change`, output comes from `channel.wait()` as `ChannelMsg::Data` and ends with `ExitStatus`; shells send escape codes (bracketed paste, colors) that `xterm.js` must receive as raw bytes.
- **2026-11-02** — Foundation on `develop`: Tauri 2 + React app with a faber-ui empty state, layer crates, strict Rust and TS lint, layer-boundary checks (both proven to reject a violation). Typecheck, lint, format, tests, UI build and Clippy pass locally; `tauri dev` opens the window. For now: commit straight to `develop` with Conventional Commits, no CI (local checks are the gate).
- **2026-11-02** — Code architecture set: DDD + Clean + Ports and Adapters, one crate per layer (ADR-0009); UI follows faber-ui conventions (ADR-0010); [conventions.md](conventions.md) added; plan reworked around contexts and layers.
- **2026-11-02** — Docs structure created (architecture, ADRs 0001–0008, roadmap, security, glossary, this plan).
- **2026-11-02** — `develop` branch created locally; SemVer and Gitflow adopted (ADR-0007).
- **2026-11-02** — Repo created at github.com/eduardo-schork/sallygate; first commit on `main`.
- **2026-11-02** — Stack, layout and name decided (ADR-0002, 0004, 0008).
