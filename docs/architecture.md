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
| **domain**         | Entities, value objects, domain errors, pure domain rules                                      | `std`, `thiserror`, `uuid`, `zeroize`              | I/O, async runtime, serialization, framework types          |
| **application**    | Use cases (one per user intent), ports (traits), application errors                            | domain, `async-trait`, `thiserror`                 | Concrete libraries for SSH, storage, keychain or UI         |
| **infrastructure** | Adapters implementing ports: `russh`, `known_hosts`, later `russh-sftp`, `keyring`, JSON files | application, domain, external crates (never Tauri) | Business rules                                              |
| **presentation**   | `desktop`: Tauri commands, DTOs, the composition root. `ui`: React screens, hooks, gateways    | everything                                         | Business rules, direct library calls that bypass a use case |

Enforced by the compiler (each Rust layer is its own crate and can only use what its `Cargo.toml` lists), by `bun run check:architecture` (allowed dependencies per crate), and by ESLint for the UI.

## Bounded contexts

| Context       | Owns                                                                                                | Status                                                                                                                     |
| ------------- | --------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `sessions`    | Interactive terminal sessions: connect, PTY, input/output, resize, keepalive, close                 | Built (0.1.0)                                                                                                              |
| `trust`       | Server host keys: verify against `known_hosts`, record trusted keys, report unknown or changed keys | Built (0.1.0)                                                                                                              |
| `credentials` | Auth methods and secrets; the only context that touches secrets (keychain now, vault later)         | Partly built: key file or password, secrets typed at connect time (ADR-0015)                                               |
| `inventory`   | Saved hosts and their groups; `~/.ssh/config` import; later tags                                    | Built (0.1.0): hosts, groups by name, import from `~/.ssh/config`                                                          |
| `transfer`    | SFTP browsing and the transfer queue                                                                | Built (0.2.0): browse, create, rename, delete, upload and download with progress, over the session's connection (ADR-0017) |
| `tunnels`     | Local, remote and dynamic port forwarding                                                           | Built (0.3.0): local and SOCKS rules, one connection each (ADR-0018)                                                       |
| `snippets`    | Saved commands sent to sessions                                                                     | Built (0.3.0): `snippets.json`, screen, panel beside the terminal                                                          |

Contexts talk to each other only through application-layer interfaces (a use case or a port), never by reaching into another context's domain internals.

## Code layout

Everything lives under `src/`, one folder per layer, read from the inside out ([ADR-0011](decisions/0011-layer-first-source-layout.md)). The tree shows what exists today.

```
src/
├── domain/                     crate sallygate-domain: pure rules, no I/O
│   └── src/
│       ├── inventory/          host.rs, host_id.rs, host_label.rs, group_name.rs,
│       │                       host_address.rs, port.rs, username.rs, ssh_endpoint.rs
│       ├── credentials/        auth_method.rs, key_file_path.rs, secret.rs, ssh_key.rs
│       ├── trust/              host_key.rs, host_key_status.rs, known_host_entry.rs
│       ├── transfer/           file_entry.rs (entries on either machine, path helpers)
│       ├── snippets/           snippet.rs (id, name, command)
│       ├── tunnels/            tunnel.rs (id, name, kind: local or dynamic)
│       └── sessions/           session_id.rs, terminal_size.rs, session_end.rs
├── application/                crate sallygate-application: use cases and ports
│   └── src/
│       ├── credentials/        manage_ssh_keys.rs; key_store.rs (port)
│       ├── inventory/          list_hosts.rs, save_host.rs, delete_host.rs, import_ssh_config.rs;
│       │                       ports: host_repository.rs, ssh_config_source.rs
│       ├── trust/              trust_host_key.rs, manage_known_hosts.rs; known_hosts.rs (port)
│       ├── snippets/           manage_snippets.rs; snippet_repository.rs (port)
│       ├── tunnels/            manage_tunnels.rs; ports: tunnel_repository.rs, tunnel_opener.rs
│       ├── transfer/           browse_remote_files.rs, browse_local_files.rs, transfer_files.rs,
│       │                       open_file_systems.rs, folder_listing.rs; ports: remote_files.rs
│       │                       (RemoteFiles, TransferProgress), local_files.rs
│       └── sessions/           open_session.rs, write_to_session.rs, resize_session.rs,
│                               close_session.rs, session_registry.rs; ports: ssh_connector.rs,
│                               live_session.rs, session_output.rs
├── infrastructure/             crate sallygate-infrastructure: adapters
│   ├── src/
│   │   ├── credentials/        ssh_folder_key_store.rs
│   │   ├── inventory/          json_host_repository.rs, ssh_config_file.rs
│   │   ├── transfer/           sftp_remote_files.rs, local_file_system.rs
│   │   ├── snippets/           json_snippet_repository.rs
│   │   ├── tunnels/            json_tunnel_repository.rs, russh_tunnel_opener.rs, socks5.rs
│   │   ├── json_file.rs        atomic JSON files in the app data folder
│   │   ├── trust/              known_hosts_file.rs, openssh_host_key.rs
│   │   └── sessions/           russh_connector.rs, russh_live_session.rs,
│   │                           server_key_check.rs, key_file.rs
│   └── tests/                  integration tests against the Docker SSH server
└── presentation/
    ├── desktop/                crate sallygate (Tauri): the native app; icons/ (generated) and
    │                           icons/source/ (the design system's SVGs)
    │   └── src/
    │       ├── credentials/    commands.rs, contract.rs
    │       ├── inventory/      commands.rs, contract.rs
    │       ├── sessions/       commands.rs, contract.rs (DTOs), channel_session_output.rs,
    │       │                   error_mapping.rs
    │       ├── trust/          commands.rs, contract.rs
    │       ├── transfer/       commands.rs, contract.rs (DTOs, progress channel)
    │       ├── snippets/       commands.rs, contract.rs
    │       ├── tunnels/        commands.rs, contract.rs
    │       ├── command_error.rs    the error every command returns
    │       └── composition.rs      builds adapters and use cases (the only place that knows them)
    └── ui/                     TypeScript: React + faber-ui, inside the desktop window
        ├── app/                entry point, theme, the window (tab bar, side menu app-side-nav/, a
        │                       screen, one session or a split grid of sessions)
        ├── credentials/        credentials-gateway.ts, credentials-contract.ts (generated), use-ssh-keys.ts,
        │                       key-users.ts, keys-view/, generate-key-dialog/
        ├── inventory/          inventory-gateway.ts, inventory-contract.ts (generated), use-hosts.ts,
        │                       group-hosts.ts, hosts-view/, host-card/, host-form/, host-drawer/
        ├── sessions/           sessions-gateway.ts, sessions-contract.ts (generated),
        │                       use-connection.ts, use-session-tabs.ts, session-request.ts,
        │                       session-output-buffer.ts, session-tab-bar/, session-panel/,
        │                       terminal-view/, session-ended-notice/
        ├── updates/            updates-gateway.ts, use-app-update.ts, update-notice/
        ├── tunnels/            tunnels-gateway.ts, tunnels-contract.ts (generated), use-tunnels.ts,
        │                       describe-tunnel.ts, forwarding-view/, tunnel-dialog/
        ├── snippets/           snippets-gateway.ts, snippets-contract.ts (generated), use-snippets.ts,
        │                       snippet-input.ts, snippets-view/, snippet-dialog/, snippet-panel/
        ├── transfer/           transfer-gateway.ts, transfer-contract.ts (generated),
        │                       use-remote-files.ts, files-view/, name-dialog/
        ├── trust/              trust-gateway.ts, trust-contract.ts (generated), use-known-hosts.ts,
        │                       known-hosts-view/,
        │                       unknown-host-key-dialog/, changed-host-key-dialog/
        └── shared/             command-error.ts, command-error-contract.ts (generated),
                                format-endpoint.ts
```

Context folders are created with their first real code; empty folders are not committed.

### UI rules

- The UI is a presentation layer: it shows state and sends the user's intents. Business rules live in Rust.
- Each context has one `*-gateway.ts`, the only code allowed to import `@tauri-apps/*`.
- Components (`*.ui.tsx`) reach the core through hooks, never through a gateway or Tauri directly. Hooks take gateways as parameters, so tests pass in-memory ones.
- File conventions follow faber-ui ([ADR-0010](decisions/0010-frontend-follows-faber-ui-conventions.md)); colors, sizes and spacing come from faber-ui tokens, including the terminal's colors, which are read from the theme's CSS variables at runtime. The Sallygate brand (palette, Manrope, JetBrains Mono) is a theme over faber-ui's variables in `ui/app/sallygate-theme.css`; brand tokens with no faber-ui slot are read through `ui/shared/brand-tokens.ts` ([ADR-0016](decisions/0016-sallygate-brand-theme.md)).

### Screens (0.1.0)

The window has a tab bar: **Hosts** first, then one tab per session.

- **Hosts:** saved hosts grouped by name, with search. _New host_ and the pencil on a card open the host drawer (create, edit, delete). _Connect_ opens a session tab.
- **Session tab:** `session-panel` owns one `useConnection`. It connects when the tab opens and closes the session when the tab closes. Hidden tabs stay mounted so their terminals keep running.

| Connection state               | Session tab shows                                      |
| ------------------------------ | ------------------------------------------------------ |
| `connecting`                   | Spinner with the destination                           |
| `confirmingHostKey`            | "Trust this server?" dialog (only on the tab in front) |
| `hostKeyChanged`               | "This server's key changed" dialog                     |
| `idle` (after a dialog closes) | "Not connected" with Connect and Close tab             |
| `failed`                       | The error, what to do, Try again and Close tab         |
| `connected`                    | Full-tab terminal                                      |
| `ended`                        | "Session ended" with Reconnect and Close tab           |

## Design-system gaps

Things the UI needs that faber-ui does not offer yet. Each is a local stand-in until faber-ui adds it.

- **Monospace font token:** the brand's JetBrains Mono is read from `--sg-font-mono` (`shared/brand-tokens.ts`); `CodeBlock` still uses the base font, so fingerprints are not mono.
- **Success and warning colors:** defined as `--sg-success` and `--sg-warning`, not yet in faber-ui's theme slots.

## Contract between UI and core

- **Commands:** `list_hosts`, `save_host`, `delete_host`, `open_session`, `write_to_session`, `resize_session`, `close_session`, `trust_host_key`.
- **Streams:** `open_session` receives a Tauri `Channel`; the session sends `output` (base64 bytes), `exited` and `disconnected` events through it ([ADR-0013](decisions/0013-ui-contract-channels-and-generated-types.md)).
- **Errors:** every command fails with `TCommandError { kind, message, hostKey? }`. `kind` is a stable code the UI branches on; `message` is safe to show and never contains a secret.
- **Types:** DTOs live in the desktop crate and derive `ts_rs::TS`; `bun run test:rust` regenerates the `*-contract.ts` files in the UI. They are committed, and a diff means the contract changed.

## Data flow: opening a terminal

1. The user clicks _Connect_ on a host card; a session tab opens and its `useConnection` calls `sessions-gateway.ts` with the host's endpoint and auth method (`TAuth`), plus a password or passphrase if the user just typed one.
2. The gateway invokes `open_session(request, onEvent)` with a new `Channel`.
3. `presentation/desktop` turns the DTO into domain values (`SshEndpoint`, `AuthMethod`, `Secret`, `TerminalSize`) and calls the `OpenSession` use case.
4. `OpenSession` asks the `SshConnector` port to connect, passing the `KnownHosts` port and a `SessionOutput` that writes to the channel.
5. `RusshConnector` connects; `ServerKeyCheck` asks `KnownHosts` about the server key:
   - trusted → it authenticates with the key file or password, opens a PTY and a shell, and returns a live session;
   - a needed secret is missing (password host, encrypted key) → it fails with `secretRequired`; the UI asks for it and opens again ([ADR-0015](decisions/0015-secrets-typed-at-connect-time.md));
   - unknown or changed → no session; the error carries the key, the UI asks the user, and on trust calls `trust_host_key` and opens again ([ADR-0014](decisions/0014-confirm-unknown-host-keys-before-connecting.md)).
6. The use case registers the session in `SessionRegistry` and returns its `SessionId`. Output streams through the channel; `session-output-buffer.ts` keeps it until the terminal mounts.
7. Keystrokes go back through `write_to_session`, size changes through `resize_session`, and closing the tab calls `close_session`. If the shell exits or the connection drops on its own, the session sends `exited` or `disconnected` and leaves the registry.

## Data model

```
Host {
  id: HostId
  label: HostLabel           // 1–64 characters
  endpoint: SshEndpoint      // HostAddress, Port, Username
  auth: AuthMethod           // KeyFile(KeyFilePath) | Password; Agent planned
  group: Option<GroupName>   // groups are just names; a group exists while a host uses it
}
```

Also: `HostKey`, `HostKeyStatus`, `SessionId`, `TerminalSize`, `SessionEnd`. Value objects validate on creation, so an invalid host cannot exist, and `JsonHostRepository` re-validates every saved entry when it reads the file. Secrets are never fields of these types.

_Planned:_ tags, a jump host, and password and ssh-agent auth.

## Local storage

- **`tunnels.json`** in the app data folder: forwarding rules (host id, kind, ports), versioned, written atomically.
- **`snippets.json`** in the app data folder: saved snippets (name and command), versioned, written atomically like `hosts.json`.
- **`hosts.json`** in the app data folder (each host may name a jump host by id, ADR-0019) (on macOS `~/Library/Application Support/dev.sallygate.app/`): saved hosts and their groups, versioned (`"version": 1`), written atomically. No secrets, only the key file path.
- **`~/.ssh/known_hosts`**: trusted server keys, shared with the `ssh` command. Sallygate writes to it only after the user trusts a key.
- **`~/.ssh/config`**: read, never written, when the user imports it (`SshConfigFile`). Imported hosts land in the "SSH config" group; importing again adds only hosts not saved yet (same address, port and username).
- **Private key files** are read when a session opens and are never copied.
- **Downloads** land in the folder open in the Mac pane under a free name; uploads read only files the user picked, dropped or dragged from the Mac pane (ADR-0020).
- _Planned:_ the OS keychain via `keyring` for passwords and passphrases.

## Open questions

- One SSH connection shared by the terminal and SFTP tabs of the same host, or one per tab?
- Reconnect policy: automatic with backoff, or ask the user?
- Reading `~/.ssh/known_hosts` when the file exists but cannot be read: `russh` treats it as empty (every server looks unknown). Report it instead?
