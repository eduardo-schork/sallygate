# ADR-0011: Layer-first source layout under `src/`, UI as part of presentation

- **Status:** Accepted
- **Date:** 2026-11-02
- **Supersedes:** the code layout in [ADR-0009](0009-ddd-clean-hexagonal-architecture.md) and the UI layering in [ADR-0010](0010-frontend-follows-faber-ui-conventions.md)

## Context

The first layout used tool-driven names (`crates/`, `src/` for the UI, `src-tauri/`) and repeated all four layers inside every UI context. The owner found it scattered and not self-explanatory: the folder names did not say which layer lived where.

## Decision

- All source code lives under `src/`, one folder per layer: `domain/`, `application/`, `infrastructure/`, `presentation/`.
- `presentation/` holds both ways the user reaches the app: `desktop/` (the Tauri app: commands, events, composition root) and `ui/` (React + faber-ui).
- Inside every layer, code is grouped by bounded context with the same names (`sessions`, `inventory`…).
- The UI is a presentation layer only: per context, a `*-gateway.ts` (the only code that talks to Tauri), hooks and components. Business rules stay in Rust.
- Each Rust layer is still its own crate, so the dependency rule is still enforced by the compiler.

## Alternatives considered

- Context-first (`src/sessions/{domain,application,…}`): groups a feature together, but one crate per context would lose compiler enforcement of the layers.
- Keeping the tool defaults (`src-tauri/`, `crates/`): familiar to Tauri users, but hides the architecture.

## Consequences

- The folder tree reads as the architecture, from the inside (`domain`) out (`presentation`).
- Tauri and Vite need explicit paths (`tauri.conf.json` build `cwd` and `frontendDist`, `index.html` entry, `tsconfig` include).
- ESLint ignores the Rust folders under `src/` and enforces the UI gateway/hook/component boundary instead of four UI layers.
