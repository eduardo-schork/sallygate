# ADR-0002: Tauri 2 + Rust core + React/TypeScript UI

- **Status:** Accepted
- **Date:** 2026-11-02

## Context

The UI must use faber-ui, a React design system. The app holds SSH keys and will grow into a vault and secrets manager. Mobile is a long-term wish. The owner wants to learn Rust.

## Decision

Tauri 2 as the app shell, a Rust core (`tokio`, `russh`, `russh-sftp`, `keyring`, `serde`) and a React + TypeScript UI with `xterm.js`.

## Alternatives considered

- Electron + Node `ssh2`: fastest start, all TypeScript, very mature SSH library; ~150 MB apps, no mobile.
- Wails + Go (`x/crypto/ssh`, `pkg/sftp`): most battle-tested SSH stack, small apps; no mobile.

## Consequences

- Small binaries (~10 MB), memory-safe core, a path to iOS/Android.
- Rust learning curve slows v0.1; the spike exists to retire that risk early.
- `russh-sftp` is less proven than Go or Node equivalents; validate it in 0.2.0.
