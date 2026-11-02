# ADR-0004: Termius-style layout

- **Status:** Accepted
- **Date:** 2026-11-02

## Context

Two layouts were prototyped: a minimal "command bar" home screen, and a layout modeled on Termius.

## Decision

Use the Termius-style layout: a window tab bar (home, SFTP, one tab per session), a side nav (Hosts, SSH keys, Port forwarding, Snippets, Vault), search with quick connect, group and host cards, and a side drawer for editing a host. The terminal goes full screen with a side panel for snippets and host info.

## Alternatives considered

- Minimal command-bar layout: rejected by the owner.

## Consequences

- A familiar layout for Termius users. We copy the layout patterns, not Termius' branding or visual identity.
- Prototype: https://claude.ai/artifact/MStPzhyRV2e4r1fSkJ7ZRk
