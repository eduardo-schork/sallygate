# Security

Sallygate holds the keys to the owner's servers. Treat every change here as security-sensitive.

## What we protect

| Asset                            | Where it lives                                                                                                                                                                                                                                                                     |
| -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Private keys                     | Key files the user points to, usually in `~/.ssh` (read by the core when a session opens, never sent to the UI). Keys made or imported on the SSH keys screen are written to `~/.ssh` with mode 0600, never over an existing file; the UI only sees public halves and fingerprints |
| Key passphrases and passwords    | Typed when a session opens or a key is generated, used once and never saved ([ADR-0015](decisions/0015-secrets-typed-at-connect-time.md))                                                                                                                                          |
| Trusted server fingerprints      | `~/.ssh/known_hosts`, shared with the `ssh` command                                                                                                                                                                                                                                |
| Updater signing key              | `~/.tauri/sallygate-updater.key` (0600, encrypted, password in the macOS Keychain) and two GitHub secrets read only by the release build step; the public half is in the app (ADR-0024)                                                                                            |
| Host list (addresses, usernames) | `hosts.json` in the app data folder; not secret, but private; never holds keys or passwords                                                                                                                                                                                        |

## Rules

1. **Secrets stay in the core.** Key files are read by the sessions connector when a session opens. A password or passphrase is the one exception that crosses the UI: typed in a password field, sent once with `open_session`, held in the core as a `Secret` (redacted, wiped on drop) and never kept ([ADR-0015](decisions/0015-secrets-typed-at-connect-time.md)). Secrets never reach `hosts.json`, logs, error messages or crash reports.
2. **Never trust a server key silently.** An unknown key needs the user's explicit approval; a changed key is shown as a possible attack, and the connection does not continue without the user's decision.
3. **The UI is untrusted.** The webview can only call the commands the core exposes; it gets no file system or network access of its own (Tauri capabilities stay minimal).
4. **Logs are redacted.** Log host ids, not addresses with credentials; never log key material, passwords or terminal contents.
5. **Dependencies are reviewed.** New crates and npm packages that touch crypto, networking or secrets need a short justification in their PR.

## What protects the app today

- The Known hosts screen removes one line of `~/.ssh/known_hosts` only after the user confirms, and only if that line still holds the key shown; the file is rewritten atomically.
- Unknown server keys need the user's confirmation; changed keys block the connection with no shortcut ([ADR-0014](decisions/0014-confirm-unknown-host-keys-before-connecting.md)).
- The webview has Tauri's `core:default` permissions plus `dialog:allow-open` (the system open dialog, to pick files to upload) and a strict Content Security Policy in production builds. It reads no files itself: it passes paths the user picked or dropped to the core.
- Updates install only after the user clicks, and only if the package's minisign signature matches the public key built into the app (ADR-0024). Release workflow actions are pinned to commit SHAs; pull request CI runs without secrets and with read-only access.
- Local network access is asked for by macOS on first use (`NSLocalNetworkUsageDescription` in `Info.plist`); the user can revoke it in Privacy & Security → Local Network.
- Forwarding rules listen on `127.0.0.1` only, so nothing on the network can use a server through the Mac; each rule logs in with the same server key check as a terminal (ADR-0018).
- SFTP runs over the session's own SSH connection, so it inherits the server key check and the login; there is no second connection to trust (ADR-0017).
- Command errors carry a code and a message written for the user; key material never goes into them.
- Nothing is logged yet.

## Threats we consider

- Man-in-the-middle on first connection or after a key change → rule 2.
- A malicious update (stolen GitHub account, tampered download) → signed update packages; the signing key never enters the repository or the logs.
- A malicious page or script inside the webview → rule 3.
- Secrets leaking through logs, screenshots or support bundles → rules 1 and 4.
- Another local user or process reading the app's files → secrets live in the keychain, not on disk.

## Not covered yet

- Vault encryption design (1.0.0).
- Protecting against a fully compromised user account on the same machine.
