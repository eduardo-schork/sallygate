# ADR-0005: Secrets in the OS keychain until the vault exists

- **Status:** Accepted
- **Date:** 2026-11-02

## Context

The app needs to remember passwords and key passphrases. Designing an encrypted vault is planned for 1.0.0.

## Decision

Until then, store secrets in the OS keychain through the `keyring` crate, keyed by host id. `hosts.json` holds only non-sensitive data. Only the core's `credentials` context touches secrets, through a `SecretStore` port ([ADR-0009](0009-ddd-clean-hexagonal-architecture.md)).

## Consequences

- No custom crypto in early versions.
- Moving to the vault later means a new `SecretStore` adapter and a change in the composition root.
- Keychain prompts on macOS may appear the first time a secret is read.
