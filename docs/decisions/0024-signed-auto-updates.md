# ADR-0024: The app updates itself from signed GitHub Releases

- **Status:** Accepted
- **Date:** 2026-11-08

## Context

Releases are published automatically (ADR-0023), but an installed app never learned about them. The owner wants updates to reach installed apps from the first version on: an app without an updater can never update itself.

## Decision

- The app uses Tauri's updater plugin. When it opens, it reads `latest.json` from the latest GitHub Release over HTTPS; if that version is newer, a notice under the tabs offers "Restart to update". Nothing installs without the user's click; a failed check (offline, no release) says nothing.
- Every update package (`Sallygate.app.tar.gz`) is signed with a dedicated minisign key. Its public half is built into the app (`tauri.conf.json`); the updater refuses a package whose signature does not match, so a compromised download or GitHub account alone cannot push code to installed apps.
- The private key lives in `~/.tauri/sallygate-updater.key` (mode 0600, outside the repository), encrypted with a random password kept in the macOS Keychain (`sallygate-updater-key-password`). For CI both go into the repository secrets `TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`, which only the build step of the Release workflow reads.
- The Release workflow publishes the package and `latest.json` (written by `scripts/ci/release-plan.mjs updater-manifest`) next to the `.dmg`. Third-party actions are pinned to commit SHAs.
- The webview gets two more permissions: `updater:default` and `process:allow-restart`.

## Alternatives considered

- No updater until Apple signing exists: every early user would have to reinstall by hand forever.
- An unencrypted key file: one leaked file would be enough to sign a malicious update.

## Consequences

- Losing the private key means installed apps can no longer be updated; back up the key file and the Keychain password together, somewhere safe.
- Rotating the key needs a release signed with the old key that ships the new public key.
- The app is still not signed by Apple (ADR-0021); updates installed by the app itself do not get the quarantine flag.
