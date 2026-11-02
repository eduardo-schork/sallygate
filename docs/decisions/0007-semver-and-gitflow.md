# ADR-0007: Semantic Versioning and Gitflow

- **Status:** Accepted
- **Date:** 2026-11-02

## Decision

- **SemVer:** `MAJOR.MINOR.PATCH`, tagged `vX.Y.Z` on `main`, starting at `0.1.0`. The version lives in the workspace `Cargo.toml` and `package.json`, kept in sync; `tauri.conf.json` reads it from Cargo.
- **Gitflow:** `main` holds released code; `develop` integrates work; work branches from `develop` are named by type (`feat/*`, `fix/*`, `refactor/*`, `docs/*`, `test/*`, `chore/*`, `ci/*`) and merge back into `develop`; `release/x.y.z` from `develop` into `main` and `develop`; `hotfix/x.y.z` from `main` into `main` and `develop`. No direct commits to `main`.

- **For now:** while the project is early and has one developer, work is committed straight to `develop` and there is no CI; local checks are the gate. Typed branches and CI return when the owner decides.
- **Commits:** Conventional Commits (`feat(sessions): ...`).

## Consequences

- Every release is a tagged merge on `main`.
- Commits and PRs carry no AI attribution trailers or watermarks.
