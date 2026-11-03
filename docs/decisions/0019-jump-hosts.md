# ADR-0019: Jump hosts go through a saved host that needs no typed secret

- **Status:** Accepted
- **Date:** 2026-11-03

## Context

Some servers are reachable only from another server (a bastion). OpenSSH calls this `ProxyJump`. The prototype's host editor has a "Jump host" field. Each connection already may ask the user to check a server key (ADR-0014) and to type a secret (ADR-0015); a jump doubles both.

## Decision

A saved host may name another saved host as its jump host. Terminal sessions, SFTP (which rides the session) and forwarding rules then log in to the jump host first, open a `direct-tcpip` channel to the target's address and port as the jump host sees them, and run the target's SSH connection inside that channel. The target's server key is checked as usual and may be trusted from the dialog.

For now the jump host must:

- already be trusted: an unknown or changed key on it fails with "connect to it on its own first";
- sign in with a key file without a passphrase: a jump host that needs a typed secret fails with a clear message.

Only one jump is supported. The UI resolves the jump host from the saved hosts and sends its endpoint and auth method with the request; the core holds the jump connection for as long as the session or rule lives.

## Alternatives considered

- Ask for both secrets and both keys in sequence: more dialogs and state in the session flow, for a case the owner does not have yet.
- Chains of jumps (`ProxyJump a,b`): rare; the same code can loop later.
- Use `~/.ssh/config`'s `ProxyCommand`: Sallygate does not run shell commands to connect.

## Consequences

- A jump host with a password needs the vault (secrets saved) before it can be used as a jump.
- The import of `~/.ssh/config` does not read `ProxyJump` yet.
