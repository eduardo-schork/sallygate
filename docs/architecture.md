# Architecture

Status: **draft**. The boundaries below are decided (see [decisions](decisions/)). Module layout, command contract and data model are proposals to settle during the v0.1 spike.

## Overview

```
┌──────────────────────────────────────────────┐
│ UI (webview): React + TypeScript + faber-ui  │
│ hosts, tabs, terminal (xterm.js), SFTP       │
└───────────────┬──────────────────────────────┘
                │ Tauri commands (invoke) + events (emit/listen)
                │ the UI never touches keys or sockets
┌───────────────▼──────────────────────────────┐
│ Core (Rust, tokio)                           │
│ sessions · SFTP · tunnels · known hosts ·    │
│ jump host · ssh-agent                        │
└───────┬───────────────────────┬──────────────┘
        │                       │
┌───────▼────────┐   ┌──────────▼─────────────┐
│ Local storage  │   │ Remote servers         │
│ hosts.json     │   │ SSH :22 (encrypted)    │
│ OS keychain    │   │                        │
│ ~/.ssh         │   └────────────────────────┘
└────────────────┘
```

## Components

### UI (webview)

- React + TypeScript, styled with faber-ui. See [ADR-0003](decisions/0003-faber-ui-design-system.md).
- Renders state and sends intents ("connect to host X", "write these bytes to session Y"). Holds no secrets.
- `xterm.js` draws the terminal. It sends keystrokes to the core and writes what the core sends back.

### Bridge (Tauri)

- **Commands** for request/response: list hosts, save a host, open a session, resize a terminal, list a remote directory.
- **Events** for streams pushed by the core: terminal output, session state changes, transfer progress.
- Every command returns `Result<T, Error>`. Errors carry a code and a message safe to show the user (never a secret).

### Core (Rust)

Proposed modules:

| Module | Responsibility |
|---|---|
| `hosts` | Load, save and validate hosts and groups (`hosts.json`). Import `~/.ssh/config`. |
| `session` | One SSH connection per session: connect, authenticate, open a PTY channel, keepalive, reconnect, close. |
| `known_hosts` | Check server fingerprints against `~/.ssh/known_hosts`; report unknown or changed keys to the UI for a decision. |
| `auth` | Pick the auth method (key file, password, ssh-agent) and fetch secrets from `secrets`. |
| `secrets` | The only module that reads or writes secrets. Keychain now, the vault later. |
| `sftp` | SFTP subsystem over an existing session: list, upload, download, transfer queue. |
| `tunnel` | Local, remote and dynamic (SOCKS) port forwarding. |

### Local storage

- `hosts.json` in the app data directory: hosts, groups, tags, ports, usernames. **No secrets.**
- OS keychain via `keyring`: passwords and key passphrases, keyed by host id.
- `~/.ssh`: read-only source for import (config, keys, known_hosts). The app writes to `known_hosts` only after the user trusts a new key.

## Data flow: opening a terminal

1. UI calls `open_session(host_id, cols, rows)`.
2. Core loads the host, connects, and checks the server key with `known_hosts`.
   - Unknown or changed key → core emits `host_key_prompt`; the UI asks the user; core continues only on explicit trust.
3. Core authenticates (`auth` → `secrets` if needed) and opens a PTY channel.
4. Core returns a `session_id` and starts emitting `session_output { session_id, bytes }`.
5. UI writes keystrokes with `session_write(session_id, bytes)` and resizes with `session_resize(session_id, cols, rows)`.
6. Closing the tab calls `session_close(session_id)`; the core emits `session_closed`.

## Data model (proposed)

```
Host {
  id: Uuid
  label: String
  address: String          // hostname or IP
  port: u16                // default 22
  username: String
  auth: AuthMethod         // KeyFile { path } | Password | Agent
  group_id: Option<Uuid>
  tags: Vec<String>
  jump_host_id: Option<Uuid>
}

Group { id: Uuid, name: String }
```

Secrets are never fields of these structs; `secrets` looks them up by `Host.id`.

## Open questions

- Terminal output: send raw bytes as base64 in events, or use a Tauri channel for binary streams? Measure during the spike.
- One SSH connection shared by terminal and SFTP tabs of the same host, or one connection per tab?
- Where exactly `hosts.json` lives on each OS (Tauri `app_data_dir`).
- Reconnect policy: automatic with backoff, or ask the user?
