# ADR-0009: DDD with Clean Architecture and Ports and Adapters

- **Status:** Accepted; code layout superseded by [ADR-0011](0011-layer-first-source-layout.md)
- **Date:** 2026-11-02

## Context

The owner wants very high code quality, no coupling, and code split by layer and domain. The app will grow from an SSH client into a vault and secrets manager, so storage, SSH and secret backends will change over time.

## Decision

- Split the code by bounded context: `inventory`, `sessions`, `trust`, `credentials`, later `transfer`, `tunnels`, `snippets`.
- Inside each context, use clean layers: domain → application → infrastructure → app, with dependencies pointing inward only.
- The application layer declares ports (traits); infrastructure provides adapters; the Tauri app wires them in one composition root.
- Enforce the dependency rule with one Cargo crate per layer. The UI mirrors the same contexts and layers.

Details: [architecture.md](../architecture.md), [conventions.md](../conventions.md).

## Alternatives considered

- A flat Tauri app with modules per feature: faster to start, but SSH, storage and UI code would mix, and swapping the keychain for the vault would touch many files.
- One crate per context instead of per layer: stronger context isolation, but the dependency rule between layers would only be a convention.

## Consequences

- Swapping an adapter (keychain → vault, `russh` → another library) touches only infrastructure and the composition root.
- Use cases are testable with in-memory fakes, without SSH or a keychain.
- More files and some mapping code between layers. Keep each piece small so the structure stays cheap.
