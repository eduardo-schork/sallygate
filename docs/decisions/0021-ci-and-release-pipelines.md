# ADR-0021: GitHub Actions for checks and macOS releases

- **Status:** Accepted
- **Date:** 2026-11-08

## Context

Until now the local checks were the only gate and there was no way to ship a build: the owner asked for a pipeline that produces releases of the app. Releases already follow Gitflow ([ADR-0007](0007-semver-and-gitflow.md)): `release/x.y.z` merges into `main` and is tagged `vX.Y.Z`.

## Decision

Two GitHub Actions workflows:

- **CI** (`.github/workflows/ci.yml`): on every push and pull request to `develop` and `main`, on Ubuntu, runs `bun run check:full` against the Docker SSH test server, then fails if the generated TypeScript contracts differ from the committed ones.
- **Release** (`.github/workflows/release.yml`): on a `vX.Y.Z` tag, checks that the tag matches the version in `Cargo.toml`, builds one universal macOS app (Apple silicon and Intel) with `tauri-action`, and drafts a GitHub Release with the `.dmg` attached. The owner reviews the draft and publishes it.

`CHANGELOG.md` lists what each release adds.

## Alternatives considered

- Build Windows and Linux too: the owner uses macOS; more targets come when someone needs them.
- Publish releases automatically: a draft lets the owner read the notes and try the build first.
- Run CI on macOS: slower and costlier; the checks do not depend on the platform, and the release job builds on macOS anyway.

## Consequences

- The app is not signed or notarized yet: macOS asks before opening it the first time. Signing needs an Apple Developer ID and secrets in the release step (a `shortcut:` in the workflow).
- A tag whose version differs from `Cargo.toml` fails before building, so bump the version on the release branch first.
- Local checks stay the gate before committing; CI repeats them on GitHub.
