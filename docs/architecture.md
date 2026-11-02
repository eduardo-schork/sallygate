# Architecture

Status: **draft**. The layers, dependency rule and runtime boundaries are decided ([ADR-0002](decisions/0002-tauri-rust-react-stack.md), [ADR-0009](decisions/0009-ddd-clean-hexagonal-architecture.md), [ADR-0010](decisions/0010-frontend-follows-faber-ui-conventions.md), [ADR-0011](decisions/0011-layer-first-source-layout.md)). The command contract and data model are proposals to settle during the v0.1 spike. How to write the code inside this structure is in [conventions.md](conventions.md).

## Runtime view

```
┌──────────────────────────────────────────────┐
│ UI (webview): React + TypeScript + faber-ui  │
└───────────────┬──────────────────────────────┘
                │ Tauri commands (invoke) + events (emit/listen)
                │ the UI never touches keys or sockets
┌───────────────▼──────────────────────────────┐
│ Core (Rust, tokio)                           │
└───────┬───────────────────────┬──────────────┘
        │                       │
┌───────▼────────┐   ┌──────────▼─────────────┐
│ Local storage  │   │ Remote servers         │
│ hosts.json     │   │ SSH :22 (encrypted)    │
│ OS keychain    │   │                        │
│ ~/.ssh         │   └────────────────────────┘
└────────────────┘
```

## Style: DDD + Clean Architecture + Ports and Adapters

- **Domain-driven:** the code is split by bounded context (below), and each context speaks the language in [glossary.md](glossary.md).
- **Clean layers:** the code has four layers: domain, application, infrastructure and presentation. Each one is a folder under `src/`, and inside each layer there is one folder per context.
- **Ports and adapters:** the application layer declares _ports_ (Rust traits) for everything outside it (storage, SSH, keychain, clock, events to the UI). _Adapters_ in the infrastructure layer implement them. Use cases never know which adapter they get.

### Dependency rule

Dependencies point inward only. Inner layers never import outer ones.

```
 presentation  ──►  infrastructure  ──►  application  ──►  domain
 (desktop app,      (adapters: russh,     (use cases,        (entities, value
  UI)                keyring, files)       ports)             objects, rules)
```

| Layer              | Contains                                                                                         | May depend on                                             | Must not contain                                            |
| ------------------ | ------------------------------------------------------------------------------------------------ | --------------------------------------------------------- | ----------------------------------------------------------- |
| **domain**         | Entities, value objects, domain errors, pure domain rules                                        | Standard library, small pure crates (`uuid`, `thiserror`) | I/O, async runtime, serialization, framework types          |
| **application**    | Use cases (one per user intent), ports (traits), application errors, DTOs crossing the boundary  | domain                                                    | Concrete libraries for SSH, storage, keychain or UI         |
| **infrastructure** | Adapters implementing ports: `russh`, `russh-sftp`, `keyring`, JSON files, `~/.ssh` parsing      | application, domain, external crates                      | Business rules                                              |
| **presentation**   | `desktop`: Tauri commands and events, the composition root. `ui`: React screens, hooks, gateways | everything                                                | Business rules, direct library calls that bypass a use case |

The rule is enforced by the compiler (each Rust layer is its own crate and can only use what its `Cargo.toml` lists), by `bun run check:architecture`, and by ESLint for the UI.

## Bounded contexts

| Context       | Owns                                                                                                                 | Arrives in |
| ------------- | -------------------------------------------------------------------------------------------------------------------- | ---------- |
| `inventory`   | Hosts, groups, tags, `~/.ssh/config` import                                                                          | 0.1.0      |
| `sessions`    | Interactive terminal sessions: connect, PTY, input/output, resize, keepalive, close                                  | 0.1.0      |
| `trust`       | Server host keys: verify against known hosts, record trusted keys, report unknown or changed keys                    | 0.1.0      |
| `credentials` | Auth methods, SSH keys, passwords and passphrases; the only context that touches secrets (keychain now, vault later) | 0.1.0      |
| `transfer`    | SFTP browsing and the transfer queue                                                                                 | 0.2.0      |
| `tunnels`     | Local, remote and dynamic port forwarding                                                                            | 0.3.0      |
| `snippets`    | Saved commands sent to sessions                                                                                      | 0.3.0      |

Contexts talk to each other only through application-layer interfaces (a use case or a port), never by reaching into another context's domain internals.

## Code layout

Everything lives under `src/`, one folder per layer, read from the inside out ([ADR-0011](decisions/0011-layer-first-source-layout.md)):

```
src/
├── domain/                  Rust crate sallygate-domain: pure rules, no I/O
│   └── src/
│       ├── inventory/       host.rs, group.rs, host_address.rs, port.rs …
│       ├── sessions/        session_id.rs, terminal_size.rs …
│       ├── trust/           host_key.rs, fingerprint.rs …
│       └── credentials/     auth_method.rs …
├── application/             Rust crate sallygate-application: use cases and ports
│   └── src/
│       ├── inventory/       save_host.rs, list_hosts.rs, host_repository.rs (port) …
│       └── sessions/        open_session.rs, ssh_connector.rs (port), session_events.rs (port) …
├── infrastructure/          Rust crate sallygate-infrastructure: adapters
│   └── src/
│       ├── inventory/       json_host_repository.rs, ssh_config_importer.rs
│       ├── sessions/        russh_connector.rs
│       ├── trust/           known_hosts_file.rs
│       └── credentials/     keychain_secret_store.rs
└── presentation/            how the user reaches the app
    ├── desktop/             Rust crate sallygate (Tauri): the native app
    │   └── src/
    │       ├── commands/    one file per context: thin handlers that call use cases
    │       ├── events/      maps port events to Tauri events
    │       └── composition.rs   builds adapters and use cases (the only place that knows concrete types)
    └── ui/                  TypeScript: React + faber-ui, inside the desktop window
        ├── app/             app shell: entry point, global styles, layout
        ├── sessions/
        │   ├── sessions-gateway.ts      the only file that calls invoke()/listen() for this context
        │   ├── use-session.ts           hooks: state and actions for components
        │   └── terminal-view/           terminal-view.ui.tsx, .styles.ts, .types.ts, index.ts
        ├── inventory/
        └── shared/          building blocks with no context knowledge
```

Context folders are created with their first real code; empty folders are not committed.

### UI rules

- The UI is a presentation layer: it shows state and sends the user's intents. Business rules live in Rust, not in TypeScript.
- Each context has one `*-gateway.ts`, the only code allowed to import `@tauri-apps/*`.
- Components (`*.ui.tsx`) talk to the core through hooks, never through a gateway or Tauri directly. Tests replace the gateway with an in-memory one.
- File conventions follow faber-ui ([ADR-0010](decisions/0010-frontend-follows-faber-ui-conventions.md)).

## Design-system gaps

Things the UI needs that faber-ui does not offer yet. Each is a local stand-in until faber-ui adds it.

- **Monospace font token:** the terminal uses the system monospace stack (`terminal-view.constants.ts`).

## Contract between UI and core

- **Commands** for request/response: list hosts, save a host, open a session, resize a terminal, list a remote directory.
- **Events** for streams the core pushes: terminal output, session state changes, transfer progress.
- Every command returns `Result<TDto, TAppError>`. Errors carry a stable code and a message safe to show the user, never a secret.
- Each session streams its output and lifecycle events through its own Tauri `Channel`, opened by `open_session` ([ADR-0013](decisions/0013-ui-contract-channels-and-generated-types.md)).
- DTOs are defined once in Rust (desktop crate) and `ts-rs` generates a `*-contract.ts` per context in the UI, so the two sides cannot drift.
- Ports are `async-trait` traits behind `Arc<dyn …>` ([ADR-0012](decisions/0012-async-ports-with-async-trait.md)).

## Data flow: opening a terminal

1. `presentation/ui`: the user clicks _Open terminal_ on a host card; the `useOpenSession()` hook calls `sessions-gateway.ts`.
2. The gateway invokes the Tauri command `open_session(host_id, cols, rows)`.
3. `presentation/desktop` handles the command and calls the `OpenSession` use case.
4. `OpenSession` loads the host (`HostRepository` port), asks `credentials` for auth material, and connects through the `SshConnector` port.
5. The `russh` adapter connects to the remote server (e.g. the owner's VPS) and checks the server key through the `KnownHosts` port. Unknown or changed key → the session is not opened and the error carries the key; the UI asks the user, and on trust calls `trust_host_key` and opens again ([ADR-0014](decisions/0014-confirm-unknown-host-keys-before-connecting.md)).
6. On success the use case returns a `SessionId`; output flows through `SessionEvents` as `session_output { session_id, bytes }`.
7. Keystrokes go back through `session_write`, size changes through `session_resize`, and closing the tab calls `session_close`.

## Data model (proposed, `inventory` domain)

```
Host {
  id: HostId
  label: HostLabel
  address: HostAddress       // hostname or IP, validated
  port: Port                 // default 22
  username: Username
  auth_method: AuthMethod    // KeyFile { path } | Password | Agent
  group_id: Option<GroupId>
  tags: Vec<Tag>
  jump_host_id: Option<HostId>
}

Group { id: GroupId, name: GroupName }
```

Value objects (`HostAddress`, `Port`, `Username`…) validate on creation, so an invalid host cannot exist. Secrets are never fields of these types; `credentials` looks them up by `HostId`.

## Local storage

- `hosts.json` in the app data directory: hosts, groups, tags, ports, usernames. **No secrets.**
- OS keychain via `keyring`: passwords and key passphrases, keyed by host id.
- `~/.ssh`: read-only source for import (config, keys, known_hosts). The app writes to `known_hosts` only after the user trusts a new key.

## Open questions

- One SSH connection shared by the terminal and SFTP tabs of the same host, or one per tab?
- Reconnect policy: automatic with backoff, or ask the user?
