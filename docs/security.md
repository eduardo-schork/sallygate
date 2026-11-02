# Security

Sallygate holds the keys to the owner's servers. Treat every change here as security-sensitive.

## What we protect

| Asset                            | Where it lives                            |
| -------------------------------- | ----------------------------------------- |
| Private keys                     | `~/.ssh` (read by the core, never copied) |
| Key passphrases and passwords    | OS keychain (later: the vault)            |
| Trusted server fingerprints      | `~/.ssh/known_hosts`                      |
| Host list (addresses, usernames) | `hosts.json`; not secret, but private     |

## Rules

1. **Secrets stay in the core.** Only the `credentials` context reads or writes them, through its `SecretStore` port. They never reach the UI, `hosts.json`, logs, error messages or crash reports.
2. **Never trust a server key silently.** An unknown key needs the user's explicit approval; a changed key is shown as a possible attack, and the connection does not continue without the user's decision.
3. **The UI is untrusted.** The webview can only call the commands the core exposes; it gets no file system or network access of its own (Tauri capabilities stay minimal).
4. **Logs are redacted.** Log host ids, not addresses with credentials; never log key material, passwords or terminal contents.
5. **Dependencies are reviewed.** New crates and npm packages that touch crypto, networking or secrets need a short justification in their PR.

## Threats we consider

- Man-in-the-middle on first connection or after a key change → rule 2.
- A malicious page or script inside the webview → rule 3.
- Secrets leaking through logs, screenshots or support bundles → rules 1 and 4.
- Another local user or process reading the app's files → secrets live in the keychain, not on disk.

## Not covered yet

- Vault encryption design (1.0.0).
- Protecting against a fully compromised user account on the same machine.
