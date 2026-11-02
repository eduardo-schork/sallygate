# ADR-0013: UI contract with per-session channels and generated TypeScript types

- **Status:** Accepted
- **Date:** 2026-11-02

## Context

The UI and the core must agree on command inputs, outputs and streamed events. A terminal streams raw bytes continuously, and each stream belongs to one session.

## Decision

- **Streams:** `open_session` receives a Tauri `Channel` from the UI. All output and lifecycle events of that session flow through it, so events never need routing by session id and stop when the session ends. Terminal bytes travel base64-encoded in JSON events.
- **Types:** request, response and event DTOs live in the desktop crate (the presentation layer), derive `ts_rs::TS`, and generate one `*-contract.ts` file per context into the UI. Generated files are committed so the UI builds without Rust; `bun run test:rust` regenerates them, and a diff means the contract changed.
- DTO type names in TypeScript carry the `T` prefix required by the UI conventions.

## Alternatives considered

- Global Tauri events (`emit`/`listen`) with a session id: every listener filters every event, and cleanup is manual.
- `tauri-specta`: also generates command bindings, but its Tauri 2 support is still a release candidate.
- Raw binary channel messages: faster, but mixes binary output with JSON lifecycle events; base64 is fast enough for a terminal and can be revisited if profiling shows otherwise.

## Consequences

- The UI cannot drift from the Rust contract without a visible diff.
- Base64 adds about 33% to output size on the local IPC bridge.
