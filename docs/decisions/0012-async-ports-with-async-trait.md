# ADR-0012: Async ports as `async-trait` traits behind `Arc<dyn …>`

- **Status:** Accepted
- **Date:** 2026-11-02

## Context

Ports such as `SshConnector` are async. Use cases receive them from the composition root and run inside Tauri's async commands, so their futures must be `Send`. Native `async fn` in traits does not promise `Send` and cannot be used as `dyn Trait`.

## Decision

Async ports are traits annotated with `#[async_trait]` and passed around as `Arc<dyn Port>`. Sync ports (e.g. reading `known_hosts`) are plain traits, also behind `Arc<dyn Port>`.

## Alternatives considered

- Native `async fn` in traits with generic use cases (`OpenSession<C: SshConnector>`): no extra crate and static dispatch, but generic parameters spread into every use case and into Tauri state types, and `Send` bounds need `trait_variant` or hand-written `impl Future + Send` signatures.

## Consequences

- Use cases have plain, readable types; the composition root swaps adapters without touching generics.
- One small, widely used dependency (`async-trait`) in the application and infrastructure layers; a boxed future per port call, which is negligible next to network I/O.
