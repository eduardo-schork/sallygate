# Architecture

Status: **draft**. The layers, dependency rule and runtime boundaries are decided ([ADR-0002](decisions/0002-tauri-rust-react-stack.md), [ADR-0009](decisions/0009-ddd-clean-hexagonal-architecture.md), [ADR-0010](decisions/0010-frontend-follows-faber-ui-conventions.md)). The command contract and data model are proposals to settle during the v0.1 spike. How to write the code inside this structure is in [conventions.md](conventions.md).

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
- **Clean layers:** each context has domain, application and infrastructure code; the Tauri app is the outermost layer.
- **Ports and adapters:** the application layer declares _ports_ (Rust traits, TypeScript types) for everything outside it (storage, SSH, keychain, clock, UI events). _Adapters_ in the infrastructure layer implement them. Use cases never know which adapter they get.

### Dependency rule

Dependencies point inward only. Inner layers never import outer ones.

```
 presentation / tauri app  ──►  infrastructure  ──►  application  ──►  domain
        (driving adapters,        (driven adapters:      (use cases,        (entities, value
         composition root)         russh, keyring, fs)     ports)             objects, rules)
```

| Layer              | Contains                                                                                        | May depend on                                             | Must not contain                                            |
| ------------------ | ----------------------------------------------------------------------------------------------- | --------------------------------------------------------- | ----------------------------------------------------------- |
| **domain**         | Entities, value objects, domain errors, pure domain rules                                       | Standard library, small pure crates (`uuid`, `thiserror`) | I/O, async runtime, serialization, framework types          |
| **application**    | Use cases (one per user intent), ports (traits), application errors, DTOs crossing the boundary | domain                                                    | Concrete libraries for SSH, storage, keychain or UI         |
| **infrastructure** | Adapters implementing ports: `russh`, `russh-sftp`, `keyring`, JSON files, `~/.ssh` parsing     | application, domain, external crates                      | Business rules                                              |
| **app** (Tauri)    | Tauri commands and events (driving adapters), dependency wiring (composition root), app config  | everything                                                | Business rules, direct library calls that bypass a use case |

The rule is enforced by the compiler: each layer is its own crate, and a crate can only use what its `Cargo.toml` lists.

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

## Code layout (Rust)

A Cargo workspace with one crate per layer and one module per context inside each crate:

```
crates/
  domain/            sallygate-domain
    src/inventory/   host.rs, group.rs, host_address.rs, port.rs, ...
    src/sessions/
    src/trust/
    src/credentials/
  application/       sallygate-application
    src/inventory/   use cases (save_host.rs, list_hosts.rs), ports (host_repository.rs)
    src/sessions/    open_session.rs, ports (ssh_connector.rs, session_events.rs)
    ...
  infrastructure/    sallygate-infrastructure
    src/inventory/   json_host_repository.rs, ssh_config_importer.rs
    src/sessions/    russh_connector.rs
    src/trust/       known_hosts_file.rs
    src/credentials/ keychain_secret_store.rs
src-tauri/           sallygate (the app)
  src/commands/      one file per context: thin handlers that call use cases
  src/events/        maps port events to Tauri events
  src/composition.rs builds adapters and use cases (the only place that knows concrete types)
```

## Code layout (UI)

The UI follows the same contexts and layers, with faber-ui's file conventions ([ADR-0010](decisions/0010-frontend-follows-faber-ui-conventions.md)):

```
src/
  app/                       app shell: providers, theme, routing, composition root
  modules/
    inventory/
      domain/                types and pure rules for the UI (no React, no Tauri)
      application/           ports (TInventoryGateway) and use-case hooks (use-hosts.ts)
      infrastructure/        tauri-inventory-gateway.ts: the only code that calls invoke()
      presentation/          components: host-card/host-card.ui.tsx, .styles.ts, .types.ts, index.ts
    sessions/
    trust/
    ...
  shared/                    cross-context building blocks with no domain knowledge
```

Components never call `invoke()` or `listen()` directly; they use hooks from `application/`, which talk to a gateway port. Tests swap the Tauri gateway for an in-memory one.

## Contract between UI and core

- **Commands** for request/response: list hosts, save a host, open a session, resize a terminal, list a remote directory.
- **Events** for streams the core pushes: terminal output, session state changes, transfer progress.
- Every command returns `Result<TDto, TAppError>`. Errors carry a stable code and a message safe to show the user, never a secret.
- DTOs are defined once in Rust and the TypeScript types are generated from them (open question below), so the two sides cannot drift.

## Data flow: opening a terminal

1. `presentation`: the user clicks _Open terminal_ on a host card; `useOpenSession()` calls the gateway.
2. `tauri-sessions-gateway` invokes `open_session(host_id, cols, rows)`.
3. `src-tauri` command calls the `OpenSession` use case.
4. `OpenSession` loads the host (`HostRepository` port), asks `credentials` for auth material, and connects through the `SshConnector` port.
5. The `russh` adapter connects; `trust` verifies the server key. Unknown or changed key → the `SessionEvents` port emits `host_key_prompt` and the use case waits for the user's decision.
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

- Terminal output: base64 bytes in events, or a Tauri channel for binary streams? Measure during the spike.
- Type generation for the UI contract: `tauri-specta`, `ts-rs`, or hand-written types checked by tests?
- Async ports: native `async fn` in traits with generics, or `async-trait` for `dyn` ports?
- One SSH connection shared by the terminal and SFTP tabs of the same host, or one per tab?
- Reconnect policy: automatic with backoff, or ask the user?
