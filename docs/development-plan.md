# Development plan

The step-by-step plan for building Sallygate, and the place where progress is tracked. [roadmap.md](roadmap.md) is the short version-level summary; this file holds the detail.

**Status legend:** `[ ]` to do · `[~]` in progress · `[x]` done · `[-]` dropped (say why)

## Current focus

- **Stage:** 2 — Hosts and tabs. Saved hosts, the host drawer and session tabs are built and checked in the native window.
- **What works today:** save, edit and delete hosts in groups (`hosts.json`); search; connect with a private key file (passphrase typed at connect time) or a password typed at connect time, never saved; import hosts from `~/.ssh/config`; confirm an unknown server key, block a changed one; a resizable terminal per tab; several tabs at once; closing a tab closes its session.
- **Next tasks, in order:**
  1. Owner: check the password dialog in the native window (background clicks cannot reach the webview).
  2. Stage 3: SFTP (`transfer` context, dual-pane browser, transfer queue).
  3. Cut `release/0.1.0` when the owner asks (no week-long trial: the owner keeps development going).
- **Owner to do:** `exit` from a shell on their server; make `develop` the default branch on GitHub; register `sallygate.com`; reserve the `sallygate` crate.
- **Open questions:** see [architecture.md](architecture.md#open-questions).
- **Known shortcuts:** none in code (`grep -rn "shortcut:" src`); design-system gaps are listed in [architecture.md](architecture.md#design-system-gaps).
- **Blocked on:** nothing.

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
- [x] Check in the native app window against the test server: trust prompt, shell, resize, close
- [ ] Same check against the owner's server, and ending a session with `exit` from the shell

## Stage 2 — Hosts and tabs (0.1.0, part B)

**Goal:** a usable daily SSH client for saved hosts.
**Branch:** `develop`
**Done when:** the features below are built and checked. The owner chose to skip a week-long trial; `release/0.1.0` is cut when the owner asks.

- [x] `inventory` domain: `Host`, `HostId`, `HostLabel`, `GroupName` (groups are names on hosts); application: `SaveHost`, `ListHosts`, `DeleteHost`; `HostRepository` port; `JsonHostRepository` adapter (`hosts.json` in the app data folder, atomic writes)
- [x] Core and contract for password login and key passphrases typed at connect time (ADR-0015)
- [x] UI: sign-in choice in the host drawer (key file or password) and a password dialog on `secretRequired`
- [ ] Later, with the vault: save secrets (`SecretStore`, Keychain); ssh-agent login
- [x] UI `inventory` context: `inventory-gateway.ts`, `use-hosts`, host card, host form, host drawer, hosts view
- [x] Hosts screen: hosts grouped by name, search, empty state (Termius-style layout, faber-ui). Quick connect without saving was dropped for now: every connection starts from a saved host.
- [x] Host drawer: create, edit and delete (with confirmation) a host
- [x] Window tab bar: Hosts plus one tab per session; switching keeps sessions running; closing a tab closes its session
- [x] Host key prompts in the UI (unknown key, changed key), per session tab
- [x] Import `~/.ssh/config`: `SshConfigSource` port, `SshConfigFile` adapter (`Host`, `HostName`, `User`, `Port`, `IdentityFile`; `Host *` as defaults; `Include` and `Match` skipped), `ImportSshConfig` use case (group "SSH config", skips hosts already saved), "Import from SSH config" button
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

- **2026-11-03** — `~/.ssh/config` import: parser adapter, use case and button with a one-line result. Rust and UI tests pass (`bun run check`). The password dialog could not be checked in the native window: background clicks do not reach the Tauri webview; left to the owner.
- **2026-11-03** — Sallygate design system v0.1 applied as a theme over faber-ui (ADR-0016): ice palette dark and light, Manrope for the interface, JetBrains Mono for the terminal and host addresses, both fonts bundled. `bun run check` passes; hosts screen checked in the native window (terminal font not yet seen in a live session).
- **2026-11-02** — The owner connected to their own cloud server from the app with a key file. Not checked yet: `exit` from the shell, password login on a real server.
- **2026-11-02** — Password UI: the host drawer chooses key file or password; a `SecretDialog` asks for a missing password or passphrase and sends it with one `open_session` call only. The core now checks the secret after the server key, so a first connection asks about the key first and the password is typed once. 51 UI tests and 11 integration tests pass. Not checked in the native window.
- **2026-11-02** — Password login and key passphrases in the core: `AuthMethod::Password`, a `Secret` value (redacted, zeroized), `secretRequired` error, `TAuth` in the contract and `hosts.json`; typed at connect time and never saved (ADR-0015). Owner chose this over Keychain storage to reach their servers sooner. 3 new integration tests (password ok, missing, wrong) and passphrase unit tests pass; the UI still only offers key files.
- **2026-11-02** — Handoff: _Current focus_ now lists what works, the next tasks in order, owner tasks and known shortcuts; the manual native-window check is written down in [conventions.md](conventions.md#tests); `/resume` (`.claude/skills/resume/`) brings a new session up to speed.
- **2026-11-02** — Native window check of Stages 1 and 2 on a debug bundle (`bun run tauri build --debug --bundles app`) against the test server: add a host to a group, connect, trust the key (fingerprint matched the container's), run commands (`sallygate-42`), resize (`stty size` 42×146 → 33×115), open a second tab with no second trust prompt, switch tabs with output kept, close both tabs (server shows no sessions left), edit and delete the host (`hosts.json` empty again). No bugs found. Not checked: `exit` from the shell, the owner's server.
- **2026-11-02** — Stage 2, first part: saved hosts (domain, use cases, `hosts.json`, commands), hosts screen, host drawer and session tabs. `bun run check:full` passes (46 UI tests, Rust unit and integration tests). Checked in the browser pane with an in-memory core: add, list, connect (opens a tab), close tab, edit, delete. Fixed there: card lost its faber-ui styles (`as` vs `forwardedAs`), clipped New host button, drawer title flipping while closing. The native-window check is still pending: Computer use is not enabled in this session.
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
