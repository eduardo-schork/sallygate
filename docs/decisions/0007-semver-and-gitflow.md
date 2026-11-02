# ADR-0007: Semantic Versioning and Gitflow

- **Status:** Accepted
- **Date:** 2026-11-02

## Decision

- **SemVer:** `MAJOR.MINOR.PATCH`, tagged `vX.Y.Z` on `main`, starting at `0.1.0`. The version is kept in sync in `Cargo.toml`, `package.json` and `tauri.conf.json`.
- **Gitflow:** `main` holds released code; `develop` integrates work; `feature/*` from `develop`; `release/x.y.z` from `develop` into `main` and `develop`; `hotfix/x.y.z` from `main` into `main` and `develop`. No direct commits to `main`.

## Consequences

- Every release is a tagged merge on `main`.
- Commits and PRs carry no AI attribution trailers or watermarks.
