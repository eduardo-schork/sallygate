# Development plan

The step-by-step plan for building Sallygate, and the place where progress is tracked. [roadmap.md](roadmap.md) is the short version-level summary; this file holds the detail.

**Status legend:** `[ ]` to do · `[~]` in progress · `[x]` done · `[-]` dropped (say why)

## Current focus

- **Stage:** 0 — Foundation
- **Next task:** push `develop` and set it as the default branch on GitHub
- **Blocked on:** nothing

Update this block whenever the focus moves.

---

## Stage 0 — Foundation

**Goal:** a repo, toolchain and empty app that build and run, so every later stage only adds features.
**Branch:** `feature/foundation` (from `develop`)
**Done when:** `cargo tauri dev` opens an empty Sallygate window styled with faber-ui, and CI is green on `develop`.

- [x] Project folder, git repo and GitHub remote
- [x] README, `.gitignore`, agent instructions, project brief
- [x] Docs structure: architecture, decisions, roadmap, security, glossary, this plan
- [x] `develop` branch created locally
- [ ] Push `develop`; make it the default branch on GitHub
- [ ] Install toolchain: `rustup` (stable), Tauri CLI, Node + package manager (decide which; faber-ui uses Bun)
- [ ] Scaffold the Tauri 2 app (React + TypeScript + Vite template)
- [ ] Add faber-ui (theme provider, global styles, fonts) and render one faber-ui screen
- [ ] Set version `0.1.0-dev` in `Cargo.toml`, `package.json` and `tauri.conf.json`
- [ ] Lint and format: `rustfmt`, `clippy`, ESLint, Prettier
- [ ] CI on GitHub Actions: build, lint and test on every PR to `develop`
- [ ] Reserve the `sallygate` crate name on crates.io

## Stage 1 — Rust SSH spike (0.1.0, part A)

**Goal:** prove that the Rust core can hold a real SSH session and that its output reaches the terminal in the UI. This is also the Rust learning stage.
**Branch:** `feature/ssh-spike`
**Done when:** the app opens a working, resizable interactive shell on a real server, typed into an `xterm.js` terminal.

- [ ] Local test server: an `openssh-server` Docker container with a test user and key
- [ ] CLI-only Rust binary: connect with `russh`, authenticate with a key file, run `uname -a`, print the output
- [ ] Interactive PTY: open a shell channel, forward stdin/stdout in the CLI
- [ ] Host key check against `~/.ssh/known_hosts`; refuse changed keys
- [ ] Move the session code into the Tauri core as a `session` module
- [ ] Tauri commands `open_session`, `session_write`, `session_resize`, `session_close` and the `session_output` event
- [ ] Measure: events with base64 bytes vs a Tauri channel; record the choice as an ADR
- [ ] `xterm.js` in a React component wired to those commands
- [ ] Connect to the owner's real server from the app

## Stage 2 — Hosts and tabs (0.1.0, part B)

**Goal:** a usable daily SSH client for saved hosts.
**Branch:** one `feature/*` per item group
**Done when:** the owner uses Sallygate instead of `ssh` in the terminal for a week. Then cut `release/0.1.0`.

- [ ] `hosts` module: `Host` and `Group` model, `hosts.json` load/save with validation
- [ ] `secrets` module: passwords and passphrases in the keychain via `keyring`
- [ ] `auth` module: key file, password and ssh-agent
- [ ] Hosts screen: group and host cards, search, quick connect (Termius-style layout, faber-ui)
- [ ] Host drawer: create and edit a host
- [ ] Window tab bar: one tab per session, close and switch
- [ ] Host key prompts in the UI (unknown key, changed key)
- [ ] Import `~/.ssh/config`
- [ ] Release `0.1.0`: `release/0.1.0`, changelog, tag `v0.1.0`, signed macOS build

## Stage 3 — SFTP (0.2.0)

**Goal:** move files without leaving the app.
**Done when:** files go both ways between the Mac and a server by drag and drop, with progress.

- [ ] `sftp` module on top of an existing session (`russh-sftp`)
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

- **2026-11-02** — Docs structure created (architecture, ADRs 0001–0008, roadmap, security, glossary, this plan).
- **2026-11-02** — `develop` branch created locally; SemVer and Gitflow adopted (ADR-0007).
- **2026-11-02** — Repo created at github.com/eduardo-schork/sallygate; first commit on `main`.
- **2026-11-02** — Stack, layout and name decided (ADR-0002, 0004, 0008).
