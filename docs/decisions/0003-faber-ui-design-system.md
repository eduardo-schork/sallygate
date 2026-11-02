# ADR-0003: faber-ui as the design system

- **Status:** Accepted
- **Date:** 2026-11-02

## Context

The owner maintains faber-ui (React, styled-components, `--faber-ui-*` CSS variables, dark and light themes).

## Decision

Build every screen with faber-ui components and tokens. Missing tokens (e.g. a monospace font, a secondary surface) are added to faber-ui, not hard-coded in Sallygate. The terminal is the one area with its own dark palette in both themes.

## Consequences

- Consistent look with the owner's other projects; theme switching for free.
- Sallygate work may require faber-ui releases first.
