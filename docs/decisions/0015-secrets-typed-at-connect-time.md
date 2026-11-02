# ADR-0015: Passwords and key passphrases are typed at connect time

- **Status:** Accepted
- **Date:** 2026-11-02

## Context

The owner needs password login (a new server before it has a key) and encrypted private keys before the vault exists. [security.md](../security.md) rule 1 says secrets never reach the UI, but the only way for the user to give a password is to type it into a field in the window. Storing secrets in the Keychain ([ADR-0005](0005-secrets-in-keychain-until-vault.md)) is not needed for the first release.

## Decision

A password or key passphrase is typed in a password field when a session opens, sent once with the `open_session` command and dropped right after. It is never saved: not in `hosts.json`, not in the Keychain, not in UI state after the request.

- A host stores only its auth method: a key file path, or "password".
- The core keeps the secret in a `Secret` value that redacts itself in `Debug` and is zeroized when dropped.
- When the core needs a secret it was not given (a password host, or an encrypted key), it fails with `secretRequired`; the UI asks for it and opens the session again, like the host key flow in [ADR-0014](0014-confirm-unknown-host-keys-before-connecting.md).

## Alternatives considered

- Native dialog owned by the Rust core: Tauri has no password prompt, and a custom window is more code for the same exposure (the secret is typed into a webview either way).
- ssh-agent only: no secret passes through the app, but it cannot do password login.
- Save it in the Keychain now: more code and a "remember" choice nobody asked for yet; comes with the credentials store.

## Consequences

- The UI touches a secret for the time of one command; it must keep it out of state, logs and error messages.
- The user types the password on every connection until secrets can be saved.
- `security.md` rule 1 gets this one exception.
- The domain may depend on `zeroize` (pure memory wiping, no I/O) for `Secret`.
