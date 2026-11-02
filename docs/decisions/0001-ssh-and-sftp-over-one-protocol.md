# ADR-0001: SSH and SFTP, no FTP

- **Status:** Accepted
- **Date:** 2026-11-02

## Context

The app must open remote terminals and move files. FTP was mentioned as an option for file transfer.

## Decision

Support SSH for terminals and SFTP for files. Both run over the same SSH connection, port and credentials. No FTP or FTPS.

## Alternatives considered

- FTP/FTPS: needs a separate server, separate credentials, and plain FTP is unencrypted.

## Consequences

- One auth flow and one trust model (known_hosts) for everything.
- Servers only need `sshd`, which they already run.
