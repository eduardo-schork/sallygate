# ADR-0023: Every merge into `main` releases, with the version taken from the commits

- **Status:** Accepted
- **Date:** 2026-11-08
- **Supersedes:** [ADR-0022](0022-release-on-push-to-main.md); in part [ADR-0021](0021-ci-and-release-pipelines.md) (how releases start) and [ADR-0007](0007-semver-and-gitflow.md) (no release branches)

## Context

ADR-0022 still asked the owner to bump the version by hand before merging. The owner wants the flow of their `refined-redmine` repository: merge into the main branch and the release happens, nothing else to edit.

## Decision

The Release workflow runs on every push to `main`:

1. **Plan:** `scripts/ci/release-plan.mjs plan` reads the commit subjects since the last `v*` tag (all history before the first release, counted from 0.0.0). Conventional Commits decide the bump: `feat` raises the minor, `fix` the patch, `!` the major (the minor while on 0.x). Only `docs`, `chore`, `ci`, `refactor`, `test` and the like release nothing. A due version without notes under `## Next version` in `CHANGELOG.md` fails here, before any build.
2. **Check:** the CI workflow runs as a reusable workflow (`check:full`).
3. **Build:** on macOS, the version is written into `Cargo.toml`, `Cargo.lock`, `package.json` and `CHANGELOG.md`, and one universal `.dmg` is built.
4. **Publish:** the same version files are committed as `chore: release X.Y.Z` with tag `vX.Y.Z`, pushed atomically to `main` (nothing is released if `main` moved), and a GitHub Release is published with the `.dmg` and the version's changelog notes.
5. **Sync:** `main` is merged back into `develop` (fast-forward when possible); a conflict stops with instructions in the job summary.

Only the publish and sync jobs have write access; they run no project code besides the release script. The workflow token pushes the release commit, and its pushes do not start workflows, so the release does not trigger itself.

## Consequences

- To release: write the notes under `## Next version`, merge `develop` into `main`. The version files on `develop` are only ever changed by the release commit.
- Commit types now matter for releases: a user-facing change must be `feat` or `fix`.
- If `main` gets branch protection, the publish job needs a token allowed to push there.
- The app is still unsigned (ADR-0021).
