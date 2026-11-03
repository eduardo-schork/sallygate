# ADR-0016: Sallygate brand theme on top of faber-ui

- **Status:** Accepted
- **Date:** 2026-11-03

## Context

The owner delivered the Sallygate design system v0.1: an "ice" palette for dark (default) and light, Manrope for the interface, JetBrains Mono for anything the user could paste into a terminal, and radii. The tokens file is the source of truth; the PDF explains it. Until now the app used faber-ui's own colors and font, and the rules said not to change them.

## Decision

The brand is a theme, not a fork. `src/presentation/ui/app/sallygate-theme.css` holds the `--sg-*` tokens as delivered and maps them onto faber-ui's theme variables (`--faber-ui-color-*`, `--faber-ui-font-family-base`), the slots faber-ui offers for brand themes. faber-ui itself is not changed; its sizes, spacing and component shapes stay.

- Brand accent → faber-ui `PRIMARY` (main button, focus ring); muted → `TEXT_SECONDARY` and `BORDER_STRONG`; raised → `DISABLED_BACKGROUND`.
- Hover and pressed tones of the accent are `color-mix` of the accent with white (dark) or black (light): the design system has no values for them yet.
- faber-ui's neutral `ACCENT` has no brand match, so it uses the text color.
- Overlay, shadow and disabled text keep faber-ui's values.
- Tokens faber-ui has no slot for (the mono font) are read through `shared/brand-tokens.ts`, never as raw values. The terminal resolves them to real values at runtime, like its colors.
- Both fonts are bundled with `@fontsource-variable/*`, so the app works offline.
- Dark is the default (`data-theme="dark"` in `index.html`); light is defined but has no switch yet.

## Alternatives considered

- Change faber-ui's palette: it is a shared library; the brand belongs to Sallygate.
- `ThemeProvider` with a theme object: it sets the variables on a wrapper element, and the terminal reads them from the root; the stylesheet also carries the fonts and the `--sg-*` names the design system uses.
- Load the fonts from Google Fonts: a desktop app must work without network.

## Consequences

- faber-ui stays the component base; the "do not change its colors or fonts" rule now means "change them only through this theme".
- Success and warning colors exist as `--sg-success` and `--sg-warning` but nothing uses them yet: connection states come with the status list.
- faber-ui's `CodeBlock` uses the base font, so fingerprints are not mono yet; a mono font token belongs in faber-ui.
- The brand radii (8/12/16) are defined but not mapped: the design system leaves spacing and components to v0.2.
