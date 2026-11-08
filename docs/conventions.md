# Code conventions

How we write code inside the structure described in [architecture.md](architecture.md). The bar is very high: low coupling, clear layers, and code that explains itself.

## Principles

1. **Respect the layers.** Follow the dependency rule in [architecture.md](architecture.md). If a change needs an inner layer to know about an outer one, add a port instead.
2. **One reason to change.** A function does one thing, a file holds one concept, a module serves one context.
3. **Make invalid states unrepresentable.** Validate at the edges with value objects and enums; inside the domain, trust the types.
4. **Self-explanatory code.** Names say what something is or does in the domain's language, so the code reads without comments.
5. **Explicit over clever.** Prefer a plain `match`, a small function or a named type over a dense one-liner or macro magic.
6. **No speculative code.** No option, abstraction or layer "for later". Add it when a real use case needs it.

## Naming

- Full words from the [glossary](glossary.md): `host_address`, not `addr`; `session_id`, not `sid`. Accepted short forms: `id`, `url`, `ssh`, `sftp`, `pty`, `dto`, `ui`.
- Booleans read as questions: `is_connected`, `has_passphrase`, `should_reconnect`.
- Functions start with a verb: `open_session`, `verify_host_key`, `list_hosts`.
- Use cases are named after the user's intent: `SaveHost`, `OpenSession`, `TrustHostKey`.
- Ports are named after the capability, adapters after the technology plus the capability: `HostRepository` → `JsonHostRepository`; `SecretStore` → `KeychainSecretStore`.

## Comments

- Comment only the **why** the code cannot show: a protocol quirk, a security reason, a workaround with a link.
- Public items in `application` and `domain` get a one-line doc comment saying what they are for.
- No commented-out code, no comments that repeat the code.

## Rust

- Workspace crates per layer, each in its layer folder under `src/`: `sallygate-domain`, `sallygate-application`, `sallygate-infrastructure`, and the `sallygate` app in `src/presentation/desktop`.
- Lints live in the workspace `Cargo.toml` (`[workspace.lints]`) and every crate opts in with `[lints] workspace = true`: `unsafe_code` is forbidden everywhere.
- Errors: one `thiserror` enum per layer and context (`InventoryError`, `OpenSessionError`). Map errors at layer boundaries; do not leak `russh` or `std::io` errors into the domain. `anyhow` only in `src/presentation/desktop` startup code, if at all.
- No `unwrap()`, `expect()` or `panic!` outside tests. Enforced with Clippy (`unwrap_used`, `expect_used`, `panic` = deny).
- Clippy runs with `-D warnings` and the `pedantic` group on; allow individual lints only with a reason.
- `rustfmt` with the default style.
- Constructors that can fail return `Result` (`HostAddress::parse`); constructors that cannot are `new`.
- Domain types do not derive `Serialize`/`Deserialize`. Serialization DTOs live in the infrastructure adapter or the app layer and map to domain types.
- Async code uses `tokio`. Async ports are `#[async_trait]` traits used as `Arc<dyn Port>` ([ADR-0012](decisions/0012-async-ports-with-async-trait.md)); a sync trait method that a library forces to look async returns `std::future::ready(…)`.
- Logs use `tracing`, never print secrets, addresses with credentials or terminal contents ([security.md](security.md)).

## TypeScript and React

Follows faber-ui's conventions ([ADR-0010](decisions/0010-frontend-follows-faber-ui-conventions.md)):

- Package manager **Bun**; **Vite** for the app; **Vitest** + Testing Library for tests; **ESLint** (`typescript-eslint` strict type-checked) + **Prettier**.
- TypeScript `strict`; no `any`, no non-null assertions (`!`), no `as` casts except at a validated boundary.
- Type aliases with a `T` prefix (`THost`, `TOpenSessionInput`); no `interface`. Named exports only, no default exports.
- `import type` for type-only imports.
- Paths and file names in kebab case. A component lives in its own folder:
  `host-card/host-card.ui.tsx`, `host-card.styles.ts`, `host-card.types.ts`, `host-card.constants.ts`, `host-card.test.tsx`, `index.ts`.
- Build screens from faber-ui components first; use a raw element only when no component fits. Styling with styled-components and faber-ui tokens; no hard-coded colors, sizes, radii, shadows or fonts. A missing token is a faber-ui change ([ADR-0003](decisions/0003-faber-ui-design-system.md)).
- To render a styled wrapper of a faber-ui component as another element, pass `forwardedAs`, not `as`: `as` on a `styled(Component)` replaces the component and drops its styles.
- Components render and forward intents; state and actions live in hooks; each context's `*-gateway.ts` is the only file that imports `@tauri-apps/api` ([architecture.md](architecture.md), UI rules).
- Semantic HTML, accessible names, keyboard support and `prefers-reduced-motion` support on every screen.

## Tests

| Layer          | What to test                              | How                                                                                                                                                                   |
| -------------- | ----------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| domain         | Every rule and value-object validation    | Plain unit tests, no mocks needed                                                                                                                                     |
| application    | Every use case, success and failure paths | Fake in-memory adapters for each port                                                                                                                                 |
| infrastructure | Each adapter against the real thing       | Integration tests in `src/infrastructure/tests/`: temp dirs for files, the Docker SSH server for SSH/SFTP; marked `#[ignore]` and run with `bun run test:integration` |
| UI             | Hooks and components                      | Vitest + Testing Library with an in-memory gateway                                                                                                                    |

A bug fix starts with a failing test that reproduces it.

**By hand in the native window**, after a large group of changes: `scripts/test-ssh-server.sh up`, then `bun run build:app` (a debug bundle without the signed update package) and open `target/debug/bundle/macos/Sallygate.app` (an agent can drive a bundled app, not the `tauri dev` binary). Add a host for `127.0.0.1:2222`, user `sallygate`, key `tests/ssh-server/.keys/id_ed25519`; check that the fingerprint matches `docker exec sallygate-test-ssh-server-1 ssh-keygen -lf /etc/ssh/ssh_host_ed25519_key.pub`, then connect, type, resize (`stty size`), open a second tab, close tabs (`ps` on the server shows no sessions left), edit and delete the host. Agents drive only the Sallygate window, never the whole screen; typing into the webview from the background drops characters, so leave typing-heavy steps to the owner.

## Commits and branches

- **For now, commit straight to `develop`.** Typed branches (`feat/`, `fix/`, `refactor/`, `docs/`, `test/`, `chore/`, `ci/` plus a short kebab-case name, e.g. `feat/host-drawer`) come back later ([ADR-0007](decisions/0007-semver-and-gitflow.md)).
- Work in the repository's main checkout; no git worktrees.
- Commit only work that is implemented, tested, and passes every local check: `bun run check` (typecheck, lint, format, UI tests, architecture, Rust lint and tests; stops at the first failure), or `bun run check:full` (adds the integration tests; needs `scripts/test-ssh-server.sh up`) when infrastructure adapters changed. Commit types decide releases (ADR-0023): `feat` raises the minor version, `fix` the patch; other types release nothing. These checks are the gate; CI on GitHub Actions repeats `check:full` on every push and pull request ([ADR-0021](decisions/0021-ci-and-release-pipelines.md)).
- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org): `<type>(<scope>): <summary>`, imperative, lower case, no period. Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, `build`, `ci`, `perf`, `style`. The scope is a bounded context (`sessions`, `inventory`…) or an area (`ui`, `core`, `docs`). Example: `feat(sessions): open a pty channel over russh`.
- One concern per commit. A commit updates the docs it affects ([docs/README.md](README.md), living memory rules).
- No AI attribution in commits or PRs.
