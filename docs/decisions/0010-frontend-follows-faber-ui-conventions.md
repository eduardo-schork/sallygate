# ADR-0010: The UI follows faber-ui's code conventions

- **Status:** Accepted; UI layering superseded by [ADR-0011](0011-layer-first-source-layout.md)
- **Date:** 2026-11-02

## Context

The UI is built on faber-ui. Using the same tooling and file conventions keeps both projects familiar and makes it easy to move a component from Sallygate into faber-ui.

## Decision

Use faber-ui's base: Bun, TypeScript strict, `typescript-eslint` strict type-checked, Prettier, Vitest, styled-components with faber-ui tokens, kebab-case paths, `.ui.tsx` / `.styles.ts` / `.types.ts` / `.constants.ts` / `index.ts` per component folder, `T`-prefixed type aliases and named exports. On top of that, the UI is split by bounded context and layer as in [ADR-0009](0009-ddd-clean-hexagonal-architecture.md).

## Consequences

- Shared lint rules can be copied from faber-ui (including its rule against hard-coded design values).
- Components that prove generic can move to faber-ui with little change.
