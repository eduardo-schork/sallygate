# Architecture

How Sallygate is built today. Layers, dependency rule and boundaries are decided ([ADR-0002](decisions/0002-tauri-rust-react-stack.md), [ADR-0009](decisions/0009-ddd-clean-hexagonal-architecture.md), [ADR-0011](decisions/0011-layer-first-source-layout.md)); sections marked _planned_ describe the next stages. How to write code inside this structure is in [conventions.md](conventions.md).

## Runtime view

```
┌──────────────────────────────────────────────┐
│ UI (webview): React + TypeScript + faber-ui  │
└───────────────┬──────────────────────────────┘
                │ Tauri commands (invoke) + one Channel per session
                │ the UI never touches keys or sockets
┌───────────────▼──────────────────────────────┐
│ Core (Rust, tokio)                           │
└───────┬───────────────────────┬──────────────┘
        │                       │
┌───────▼──────────────┐   ┌────▼───────────────────────┐
│ This machine         │   │ Remote servers             │
│ ~/.ssh/known_hosts   │   │ SSH (encrypted), e.g. the  │
│ private key files    │   │ owner's VPS or home server │
│ later: hosts.json,   │   └────────────────────────────┘
│ OS keychain          │
└──────────────────────┘
```

There is no Sallygate server or cloud: the app connects straight to the user's servers, like the `ssh` command.

## Style: DDD + Clean Architecture + Ports and Adapters

- **Domain-driven:** the code is split by bounded context (below), and each context speaks the language in [glossary.md](glossary.md).
- **Clean layers:** four layers, each a folder under `src/`: domain, application, infrastructure, presentation. Inside each layer there is one folder per context.
- **Ports and adapters:** the application layer declares _ports_ (Rust traits) for everything outside it. _Adapters_ in the infrastructure layer implement them. Use cases never know which adapter they get. Async ports are `async-trait` traits behind `Arc<dyn …>` ([ADR-0012](decisions/0012-async-ports-with-async-trait.md)).

### Dependency rule

Dependencies point inward only. Inner layers never import outer ones.

```
 presentation  ──►  infrastructure  ──►  application  ──►  domain
 (desktop app,      (adapters: russh,     (use cases,        (entities, value
  UI)                files, keychain)      ports)             objects, rules)
```

| Layer              | Contains                                                                                       | May depend on                                      | Must not contain                                            |
| ------------------ | ---------------------------------------------------------------------------------------------- | -------------------------------------------------- | ----------------------------------------------------------- |
| **domain**         | Entities, value objects, domain errors, pure domain rules                                      | `std`, `thiserror`, `uuid`                         | I/O, async runtime, serialization, framework types          |
| **application**    | Use cases (one per user intent), ports (traits), application errors                            | domain, `async-trait`, `thiserror`                 | Concrete libraries for SSH, storage, keychain or UI         |
| **infrastructure** | Adapters implementing ports: `russh`, `known_hosts`, later `russh-sftp`, `keyring`, JSON files | application, domain, external crates (never Tauri) | Business rules                                              |
| **presentation**   | `desktop`: Tauri commands, DTOs, the composition root. `ui`: React screens, hooks, gateways    | everything                                         | Business rules, direct library calls that bypass a use case |

Enforced by the compiler (each Rust layer is its own crate and can only use what its `Cargo.toml` lists), by `bun run check:architecture` (allowed dependencies per crate), and by ESLint for the UI.

## Bounded contexts

| Context       | Owns                                                                                                | Status                                         |
| ------------- | --------------------------------------------------------------------------------------------------- | ---------------------------------------------- |
| `sessions`    | Interactive terminal sessions: connect, PTY, input/output, resize, keepalive, close                 | Built (0.1.0)                                  |
| `trust`       | Server host keys: verify against `known_hosts`, record trusted keys, report unknown or changed keys | Built (0.1.0)                                  |
| `credentials` | Auth methods and secrets; the only context that touches secrets (keychain now, vault later)         | Partly built: key file without passphrase only |
| `inventory`   | Hosts, groups, tags, `~/.ssh/config` import                                                         | Partly built: `SshEndpoint` and its values     |
| `transfer`    | SFTP browsing and the transfer queue                                                                | Planned (0.2.0)                                |
| `tunnels`     | Local, remote and dynamic port forwarding                                                           | Planned (0.3.0)                                |
| `snippets`    | Saved commands sent to sessions                                                                     | Planned (0.3.0)                                |

Contexts talk to each other only through application-layer interfaces (a use case or a port), never by reaching into another context's domain internals.

## Code layout

Everything lives under `src/`, one folder per layer, read from the inside out ([ADR-0011](decisions/0011-layer-first-source-layout.md)). The tree shows what exists today.

```
src/
├── domain/                     crate sallygate-domain: pure rules, no I/O
│   └── src/
│       ├── inventory/          host_address.rs, port.rs, username.rs, ssh_endpoint.rs
│       ├── credentials/        auth_method.rs, key_file_path.rs
│       ├── trust/              host_key.rs, host_key_status.rs
│       └── sessions/           session_id.rs, terminal_size.rs, session_end.rs
├── application/                crate sallygate-application: use cases and ports
│   └── src/
│       ├── trust/              trust_host_key.rs; known_hosts.rs (port)
│       └── sessions/           open_session.rs, write_to_session.rs, resize_session.rs,
│                               close_session.rs, session_registry.rs; ports: ssh_connector.rs,
│                               live_session.rs, session_output.rs
├── infrastructure/             crate sallygate-infrastructure: adapters
│   ├── src/
│   │   ├── trust/              known_hosts_file.rs, openssh_host_key.rs
│   │   └── sessions/           russh_connector.rs, russh_live_session.rs,
│   │                           server_key_check.rs, key_file.rs
│   └── tests/                  integration tests against the Docker SSH server
└── presentation/
    ├── desktop/                crate sallygate (Tauri): the native app
    │   └── src/
    │       ├── sessions/       commands.rs, contract.rs (DTOs), channel_session_output.rs,
    │       │                   error_mapping.rs
    │       ├── trust/          commands.rs, contract.rs
    │       ├── command_error.rs    the error every command returns
    │       └── composition.rs      builds adapters and use cases (the only place that knows them)
    └── ui/                     TypeScript: React + faber-ui, inside the desktop window
        ├── app/                entry point, global styles, the screen switch
        ├── sessions/           sessions-gateway.ts, sessions-contract.ts (generated),
        │                       use-connection.ts, session-output-buffer.ts,
        │                       connect-form/, terminal-view/, session-bar/, session-ended-notice/
        ├── trust/              trust-gateway.ts, trust-contract.ts (generated),
        │                       unknown-host-key-dialog/, changed-host-key-dialog/
        └── shared/             command-error.ts, command-error-contract.ts (generated)
```

Context folders are created with their first real code; empty folders are not committed.

### UI rules

- The UI is a presentation layer: it shows state and sends the user's intents. Business rules live in Rust.
- Each context has one `*-gateway.ts`, the only code allowed to import `@tauri-apps/*`.
- Components (`*.ui.tsx`) reach the core through hooks, never through a gateway or Tauri directly. Hooks take gateways as parameters, so tests pass in-memory ones.
- File conventions follow faber-ui ([ADR-0010](decisions/0010-frontend-follows-faber-ui-conventions.md)); colors, sizes and spacing come from faber-ui tokens, including the terminal's colors, which are read from the theme's CSS variables at runtime.

### Screens (0.1.0)

`use-connection.ts` holds one connection's state; `app.ui.tsx` picks the screen from it:

| State                          | Screen                                            |
| ------------------------------ | ------------------------------------------------- |
| `idle`, `connecting`, `failed` | Connect form, with the error and what to do next  |
| `confirmingHostKey`            | Connect form + "Trust this server?" dialog        |
| `hostKeyChanged`               | Connect form + "This server's key changed" dialog |
| `connected`                    | Session bar + full-window terminal                |
| `ended`                        | "Session ended" notice with Reconnect and Back    |

## Design-system gaps

Things the UI needs that faber-ui does not offer yet. Each is a local stand-in until faber-ui adds it.

- **Monospace font token:** the terminal uses the system monospace stack (`terminal-view.constants.ts`).

## Contract between UI and core

- **Commands:** `open_session`, `write_to_session`, `resize_session`, `close_session`, `trust_host_key`.
- **Streams:** `open_session` receives a Tauri `Channel`; the session sends `output` (base64 bytes), `exited` and `disconnected` events through it ([ADR-0013](decisions/0013-ui-contract-channels-and-generated-types.md)).
- **Errors:** every command fails with `TCommandError { kind, message, hostKey? }`. `kind` is a stable code the UI branches on; `message` is safe to show and never contains a secret.
- **Types:** DTOs live in the desktop crate and derive `ts_rs::TS`; `bun run test:rust` regenerates the `*-contract.ts` files in the UI. They are committed, and a diff means the contract changed.

## Data flow: opening a terminal

1. The user fills the connect form; `useConnection().connect(request)` calls `sessions-gateway.ts`.
2. The gateway invokes `open_session(request, onEvent)` with a new `Channel`.
3. `presentation/desktop` turns the DTO into domain values (`SshEndpoint`, `AuthMethod`, `TerminalSize`) and calls the `OpenSession` use case.
4. `OpenSession` asks the `SshConnector` port to connect, passing the `KnownHosts` port and a `SessionOutput` that writes to the channel.
5. `RusshConnector` connects; `ServerKeyCheck` asks `KnownHosts` about the server key:
   - trusted → it authenticates with the key file, opens a PTY and a shell, and returns a live session;
   - unknown or changed → no session; the error carries the key, the UI asks the user, and on trust calls `trust_host_key` and opens again ([ADR-0014](decisions/0014-confirm-unknown-host-keys-before-connecting.md)).
6. The use case registers the session in `SessionRegistry` and returns its `SessionId`. Output streams through the channel; `session-output-buffer.ts` keeps it until the terminal mounts.
7. Keystrokes go back through `write_to_session`, size changes through `resize_session`, and _Close session_ calls `close_session`. If the shell exits or the connection drops on its own, the session sends `exited` or `disconnected` and leaves the registry.

## Data model

Built: `SshEndpoint { address: HostAddress, port: Port, username: Username }`, `AuthMethod::KeyFile(KeyFilePath)`, `HostKey`, `HostKeyStatus`, `SessionId`, `TerminalSize`, `SessionEnd`. Value objects validate on creation, so an invalid endpoint cannot exist.

_Planned_ (Stage 2, `inventory`):

```
Host {
  id: HostId
  label: HostLabel
  endpoint: SshEndpoint
  auth_method: AuthMethod    // KeyFile | Password | Agent
  group_id: Option<GroupId>
  tags: Vec<Tag>
  jump_host_id: Option<HostId>
}

Group { id: GroupId, name: GroupName }
```

Secrets are never fields of these types; `credentials` looks them up by `HostId`.

## Local storage

- **Today:** `~/.ssh/known_hosts` holds trusted server keys, shared with the `ssh` command. Sallygate writes to it only after the user trusts a key. Private key files are read when a session opens and are never copied.
- _Planned:_ `hosts.json` in the app data directory for hosts and groups (no secrets); the OS keychain via `keyring` for passwords and passphrases.

## Open questions

- One SSH connection shared by the terminal and SFTP tabs of the same host, or one per tab?
- Reconnect policy: automatic with backoff, or ask the user?
- Reading `~/.ssh/known_hosts` when the file exists but cannot be read: `russh` treats it as empty (every server looks unknown). Report it instead?
