# Glossary

## Domain

- **SSH**: protocol for running a terminal on a remote server over an encrypted connection. Default port 22.
- **SFTP**: file transfer that runs inside an SSH connection: same port, same login, same key. Not related to FTP.
- **FTP**: an older, unencrypted file transfer protocol with its own server. Out of scope.
- **PTY**: pseudo-terminal. The remote side of an interactive shell; it needs to know the terminal size (cols × rows).
- **Port forwarding / tunnel**: carrying a TCP connection through SSH. _Local_ (`-L`) exposes a remote service on a local port; _remote_ (`-R`) does the opposite; _dynamic_ (`-D`) is a SOCKS proxy.
- **Jump host**: a server you connect to first in order to reach another one behind it (`ProxyJump`).
- **Known hosts**: the list of server key fingerprints already trusted, in `~/.ssh/known_hosts`. A changed fingerprint can mean a man-in-the-middle attack.
- **ssh-agent**: a local process that holds unlocked keys so apps can sign without reading the key file.

## Architecture

- **Bounded context**: a part of the domain with its own model and language. Ours: `inventory`, `sessions`, `trust`, `credentials`, later `transfer`, `tunnels`, `snippets`.
- **Entity**: a domain object with an identity that lasts over time (a `Host`).
- **Value object**: a domain value defined only by its content and validated on creation (`HostAddress`, `Port`).
- **Use case**: one application-layer operation for one user intent (`OpenSession`).
- **Port**: an interface the application layer needs from the outside world (`HostRepository`, `SshConnector`).
- **Adapter**: an implementation of a port for one technology (`JsonHostRepository`, `RusshConnector`), or a driver of use cases (Tauri commands).
- **Composition root**: the one place that builds the concrete adapters and hands them to the use cases.
- **Gateway** (UI): a context's `*-gateway.ts`, the only UI code that calls the Rust core through Tauri.
- **Contract**: the DTOs exchanged between the UI and the core. Defined in Rust, generated into `*-contract.ts` files with `ts-rs`.
- **Channel** (Tauri): a one-way stream from the core to the UI. Each session gets its own for output and its end.

## Project

- **Host**: a saved server entry (label, address, port, username, auth method, group, tags). Part of `inventory`.
- **Group**: a name hosts are filed under (`Cloud`, `Home`). It exists while at least one host uses it. Part of `inventory`.
- **Session**: one live SSH connection with a shell, shown as a terminal in its own tab. Part of `sessions`.
- **Tab**: the Hosts screen or one session; closing a session tab closes its session.
- **Endpoint**: where to connect and as whom, `username@address:port` (`SshEndpoint`).
- **Host key**: a server's public key, verified by `trust` against known hosts.
- **Core**: the Rust side of the app. Owns connections and secrets.
- **Vault**: the planned encrypted store for secrets, behind a master password (1.0.0).
- **Sallygate**: the gate of a sally port, the fortified passage a garrison used to leave and re-enter a fort.
