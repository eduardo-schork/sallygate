# Changelog

What changed in each release, newest first. Versions follow [SemVer](https://semver.org) (ADR-0007).

Add what a `feat` or `fix` commit changes under **Next version**. Merging into `main` releases it: the version comes from the commits, and this heading becomes that version (ADR-0023).

## 0.1.0

### Added

- Saved hosts in groups, search, and import from `~/.ssh/config`.
- A terminal per tab over SSH, with key file or password login (secrets typed at connect time, never saved).
- Server key checks against `~/.ssh/known_hosts`, and a Known hosts screen to forget a key.
- Files view per session: this Mac beside the server, drag files or whole folders to copy, transfer progress.
- Snippets, with eight starters, sent to the terminal from its side panel.
- SSH keys screen: list, generate Ed25519, import, copy the public key.
- Port forwarding rules (local and SOCKS) with on/off switches.
- Jump hosts (`ProxyJump`).
- Split view: every open session on screen at once.
- The Sallygate design system: palette, Manrope and JetBrains Mono, app icon.
- Automatic updates: the app offers each new release and installs it after checking its signature.
- Servers on your local network: macOS asks once for local network access.
