# ADR-0014: Unknown host keys are confirmed by the user before connecting

- **Status:** Accepted
- **Date:** 2026-11-02

## Context

When a server presents a key that is not in `known_hosts`, the user must decide whether to trust it ([security.md](../security.md), rule 2). The key arrives in the middle of the SSH handshake, inside the connector.

## Decision

The connector never waits for the user mid-handshake. It checks the key through the `KnownHosts` port and:

- **trusted** → continues;
- **unknown** → aborts with `OpenSessionError::UnknownHostKey(HostKey)`; the UI shows the fingerprint, and if the user trusts it, calls `trust_host_key` and opens the session again;
- **changed** → aborts with `OpenSessionError::HostKeyChanged(HostKey)`; the UI shows a warning and offers no one-click way to continue.

## Consequences

- No half-open connections waiting on a dialog; the handshake code stays simple.
- Trusting a new server costs one extra handshake.
- Recovering from a changed key is a deliberate manual step (removing the old key), as with OpenSSH.
