# Security

Sallygate holds the keys to the owner's servers. Treat every change here as security-sensitive.

## What we protect

| Asset                            | Where it lives                                                                                                            |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| Private keys                     | Key files the user points to, usually in `~/.ssh` (read by the core when a session opens, never copied or sent to the UI) |
| Key passphrases and passwords    | Typed when a session opens, used once and never saved ([ADR-0015](decisions/0015-secrets-typed-at-connect-time.md))       |
| Trusted server fingerprints      | `~/.ssh/known_hosts`, shared with the `ssh` command                                                                       |
| Host list (addresses, usernames) | `hosts.json` in the app data folder; not secret, but private; never holds keys or passwords                               |

## Rules

1. **Secrets stay in the core.** Key files are read by the sessions connector when a session opens. A password or passphrase is the one exception that crosses the UI: typed in a password field, sent once with `open_session`, held in the core as a `Secret` (redacted, wiped on drop) and never kept ([ADR-0015](decisions/0015-secrets-typed-at-connect-time.md)). Secrets never reach `hosts.json`, logs, error messages or crash reports.
2. **Never trust a server key silently.** An unknown key needs the user's explicit approval; a changed key is shown as a possible attack, and the connection does not continue without the user's decision.
3. **The UI is untrusted.** The webview can only call the commands the core exposes; it gets no file system or network access of its own (Tauri capabilities stay minimal).
4. **Logs are redacted.** Log host ids, not addresses with credentials; never log key material, passwords or terminal contents.
5. **Dependencies are reviewed.** New crates and npm packages that touch crypto, networking or secrets need a short justification in their PR.

## What protects the app today

- Unknown server keys need the user's confirmation; changed keys block the connection with no shortcut ([ADR-0014](decisions/0014-confirm-unknown-host-keys-before-connecting.md)).
- The webview has Tauri's `core:default` permissions plus `dialog:allow-open` (the system open dialog, to pick files to upload) and a strict Content Security Policy in production builds. It reads no files itself: it passes paths the user picked or dropped to the core.
- SFTP runs over the session's own SSH connection, so it inherits the server key check and the login; there is no second connection to trust (ADR-0017).
- Command errors carry a code and a message written for the user; key material never goes into them.
- Nothing is logged yet.

## Threats we consider

- Man-in-the-middle on first connection or after a key change → rule 2.
- A malicious page or script inside the webview → rule 3.
- Secrets leaking through logs, screenshots or support bundles → rules 1 and 4.
- Another local user or process reading the app's files → secrets live in the keychain, not on disk.

## Not covered yet

- Vault encryption design (1.0.0).
- Protecting against a fully compromised user account on the same machine.
