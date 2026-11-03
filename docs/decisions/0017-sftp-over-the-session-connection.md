# ADR-0017: SFTP runs over the open session's connection

- **Status:** Accepted
- **Date:** 2026-11-03

## Context

Stage 3 adds file transfer. A file browser needs an authenticated SSH connection to the server. Opening a new connection would ask again for the server key check and, for password hosts, for the password (ADR-0015 never saves it).

## Decision

The file browser belongs to a session tab. The first time a tab shows its files, the core opens an SFTP channel (`russh-sftp`) on that session's SSH connection and keeps it until the session closes.

- `LiveSession` (port of `sessions`) gains `open_files()`, which returns the `transfer` context's `RemoteFiles` port. That is the only link between the two contexts.
- The `transfer` application layer caches one `RemoteFiles` per open session and drops it once the session is gone.
- Downloads go to the user's Downloads folder with a free name (`notes (1).txt`); uploads go to the folder the browser shows. Both report progress through a `TransferProgress` port, which the desktop layer forwards to the UI over a Tauri `Channel`, like session output.
- Local files are picked with the native open dialog or dropped onto the window (Tauri gives the dropped paths). The plan's local pane comes later.

## Alternatives considered

- A separate SSH connection per browser: a second host key check and, for password hosts, a second password prompt.
- `scp` through the shell: no listing, no progress, depends on the server's shell.

## Consequences

- Closing a session tab also ends its file browser and stops its transfers.
- A server without the SFTP subsystem shows an error in the files view; the terminal still works.
- `sallygate-application`'s `sessions` module depends on `transfer` for the `RemoteFiles` type.
