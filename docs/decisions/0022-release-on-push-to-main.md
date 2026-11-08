# ADR-0022: A new version on `main` releases itself

- **Status:** Accepted
- **Date:** 2026-11-08
- **Supersedes in part:** [ADR-0021](0021-ci-and-release-pipelines.md) (what starts a release) and [ADR-0007](0007-semver-and-gitflow.md) (release branches become optional)

## Context

ADR-0021 started a release from a pushed `vX.Y.Z` tag, after a `release/x.y.z` branch. The owner finds that too many steps: the version in `Cargo.toml` and the changelog already say what is being released.

## Decision

The Release workflow runs on every push to `main`:

- It reads the version from `Cargo.toml` and fails if `package.json` says otherwise.
- A version ending in `-dev`, or one that already has a GitHub Release (draft included), releases nothing.
- Any other version is built (universal macOS app) and drafted as a GitHub Release; the tag `vX.Y.Z` is created on that commit when the owner publishes the draft.

To release: on `develop`, set the version without `-dev` in `Cargo.toml` and `package.json` and move _Unreleased_ in `CHANGELOG.md` under it, then merge `develop` into `main` and push. Afterwards, bump `develop` to the next `-dev` version. A `release/x.y.z` branch is still fine for a release that needs fixes before it ships, but not required.

## Consequences

- Pushing `main` with a new version is what ships it: bump the version only when you mean to release.
- Re-pushing `main` before the draft is published does not build a second one.
